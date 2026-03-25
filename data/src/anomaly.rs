//! Reusable anomaly detection primitives for rolling-window statistical analysis.
//!
//! **Design for upstream extraction**: This module is pure math with zero framework
//! dependencies (no iced, no async, no alloc beyond what `&[f32]` provides). It can
//! be extracted into a standalone crate (e.g., `opendeviationbar-anomaly`) for use as
//! ClickHouse columnar features computed during bar construction in opendeviationbar-py.
//!
//! Three complementary layers, all O(1) per bar on a pre-sorted window:
//! 1. **Adjusted Boxplot fence** — Quartile Skewness (Bahri 2024) + Hubert (2008) exponential formula
//! 2. **Conformal p-values** — distribution-free calibrated severity scoring
//! 3. **CUSUM** — cumulative sum control chart for sustained regime shift detection
//!
//! All functions operate on a sorted `&[f32]` window (ascending order).
//! For upstream use with f64, the algorithms are identical — only the type changes.

/// Result from the adjusted boxplot fence computation.
///
/// Contains all intermediate values needed for downstream severity scoring,
/// so callers don't need to recompute quartiles.
#[derive(Debug, Clone, Copy)]
pub struct FenceResult {
    /// Lower fence value. Observations below this are flagged as anomalous.
    pub fence: f32,
    /// Interquartile range (Q3 - Q1). Used for graduated severity: `(fence - x) / iqr`.
    pub iqr: f32,
    /// Quartile Skewness (Bowley coefficient) in [-1, 1]. Positive = right-skewed.
    /// Equivalent to Medcouple at 12.5% breakdown point (vs MC's 25%).
    pub skewness: f32,
}

/// Adjusted Boxplot lower fence using Quartile Skewness (Bahri et al. 2024).
///
/// Replaces O(n²) Medcouple with O(1) Quartile Skewness:
///   `QS = (Q3 + Q1 - 2*median) / (Q3 - Q1)`
/// The Hubert (2008) exponential formula adapts the fence multiplier for skewness:
///   `h_lower = 1.5 * exp(-4*QS)` when QS >= 0 (right-skewed → tighter fence)
///   `h_lower = 1.5 * exp(-3*QS)` when QS < 0 (left-skewed → looser fence)
///   `fence = Q1 - h_lower * IQR`
///
/// # Requirements
/// - `sorted` must be in ascending order
/// - At least 20 elements for stable quartile estimates
///
/// # Returns
/// `None` if window too small or degenerate (IQR ≈ 0).
///
/// # References
/// - Hubert & Vandervieren (2008), "An adjusted boxplot for skewed distributions"
/// - Bahri et al. (2024), "Online boxplot derived outlier detection"
pub fn adjusted_lower_fence(sorted: &[f32]) -> Option<FenceResult> {
    let n = sorted.len();
    if n < 20 {
        return None;
    }
    let q1 = sorted[n / 4];
    let median = sorted[n / 2];
    let q3 = sorted[3 * n / 4];
    let iqr = q3 - q1;
    if iqr <= f32::EPSILON {
        return None;
    }
    let qs = (q3 + q1 - 2.0 * median) / iqr;
    let h_lower = if qs >= 0.0 {
        1.5 * (-4.0 * qs).exp()
    } else {
        1.5 * (-3.0 * qs).exp()
    };
    Some(FenceResult {
        fence: q1 - h_lower * iqr,
        iqr,
        skewness: qs,
    })
}

/// Compute conformal p-value: fraction of window values with nonconformity score
/// at least as extreme as the test point.
///
/// Uses `|x - median| / MAD` as the nonconformity measure (MAD approximated as
/// half-IQR for O(1) computation on sorted data).
///
/// # Guarantees
/// - Distribution-free: valid under exchangeability (no distributional assumptions)
/// - Calibrated: P(p-value ≤ α) ≤ α for any significance level α
/// - Range: [0, 1] where lower = more anomalous
///
/// # Complexity
/// O(n) — iterates the sorted window once. For O(1) amortized, maintain a
/// pre-computed count of values exceeding each nonconformity threshold.
///
/// # References
/// - Vovk, Gammerman & Shafer (2005), "Algorithmic Learning in a Random World"
pub fn conformal_pvalue(sorted: &[f32], value: f32) -> f32 {
    let n = sorted.len();
    if n < 3 {
        return 1.0;
    }
    let median = sorted[n / 2];
    let mad = (sorted[3 * n / 4] - sorted[n / 4]) * 0.5;
    if mad <= f32::EPSILON {
        return 1.0;
    }
    let test_score = (value - median).abs() / mad;
    let count = sorted
        .iter()
        .filter(|&&v| (v - median).abs() / mad >= test_score)
        .count();
    count as f32 / (n + 1) as f32
}

/// CUSUM (Cumulative Sum) control chart for detecting sustained downward shifts.
///
/// Accumulates evidence that the process has shifted below the reference level.
/// Resets to zero when the process returns above reference + allowance.
///
/// # Arguments
/// * `cusum_prev` — previous CUSUM accumulator value (0.0 initially)
/// * `value` — current observation
/// * `reference` — expected value under null hypothesis (typically rolling median)
/// * `allowance` — minimum shift to detect (half the expected deviation under H₁)
///
/// # Returns
/// Updated CUSUM value (≥ 0). Compare against `DEFAULT_CUSUM_THRESHOLD` to trigger alarms.
///
/// # Example (upstream columnar use)
/// ```text
/// // In opendeviationbar-py bar construction:
/// let cusum = cusum_negative(prev_cusum, log10_intensity, window_median, 0.15);
/// bar.anomaly_cusum = cusum;  // stored as ClickHouse Float32 column
/// ```
///
/// # References
/// - Page (1954), "Continuous Inspection Schemes"
pub fn cusum_negative(cusum_prev: f32, value: f32, reference: f32, allowance: f32) -> f32 {
    (cusum_prev + (reference - value) - allowance).max(0.0)
}

/// Default CUSUM allowance for log₁₀(trade_intensity).
/// A shift of 0.3 in log space ≈ 2× intensity change; allowance = half that.
pub const DEFAULT_CUSUM_ALLOWANCE: f32 = 0.15;

/// Default CUSUM alarm threshold. Higher = fewer false alarms, slower detection.
pub const DEFAULT_CUSUM_THRESHOLD: f32 = 2.0;
