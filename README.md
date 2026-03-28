# Flowsurface

[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/flowsurface-rs/flowsurface/blob/main/LICENSE)
[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

An experimental open-source desktop charting application. Supports Binance, Bybit, Hyperliquid and OKX.

> **This is a community fork** of [terrylica/flowsurface](https://github.com/terrylica/flowsurface), which is itself based on [flowsurface-rs/flowsurface](https://github.com/flowsurface-rs/flowsurface).
> It adds open-source build support (removing the private `qta` dependency) and includes all ODB chart extensions from the terrylica fork.

---

## What's Different in This Fork?

### ✅ Open-Source Build Support (Linux / macOS / Windows)

The terrylica fork depends on a private `qta` crate that is not publicly available, preventing compilation. This fork removes that dependency to enable open builds:

- **X11 and Wayland rendering** have been re-enabled in the `iced` dependency, so the app draws correctly on any Linux desktop.
- **The `qta` dependency has been removed.** The ZigZag swing-structure overlay it powered is stubbed out — the menu entry is still visible but the overlay renders nothing until a compatible open-source replacement is available.

### 🆕 Open Deviation Bar (ODB) Charts

Inherited from the upstream fork. ODB is a range-bar chart type where each bar closes once price deviates a configurable percentage from its open, rather than after a fixed time interval.

Four threshold presets are available. To open an ODB chart pane, right-click the dashboard and choose **"Open Deviation Bar Chart"**, then select a threshold:

| Label  | Threshold | Meaning                              |
|--------|-----------|--------------------------------------|
| BPR10  | 100 dbps  | Bar closes after 0.10% price move   |
| BPR25  | 250 dbps  | Bar closes after 0.25% price move   |
| BPR50  | 500 dbps  | Bar closes after 0.50% price move   |
| BPR75  | 750 dbps  | Bar closes after 0.75% price move   |

> **Note:** ODB charts require a running ClickHouse instance populated by [opendeviationbar-py](https://github.com/terrylica/opendeviationbar-py). Without it the pane will remain in "Fetching Klines…" state. The rest of the app (all standard chart types) works without ClickHouse.

### 🆕 Additional Indicators

These indicators are present in this fork and are accessible from the indicator panel (settings gear icon on any Candlestick or ODB chart pane):

| Indicator | Works on | Description |
|-----------|----------|-------------|
| **Volume** | Time / Tick / ODB | Bar volume subplot |
| **Delta** | Time / Tick / ODB | Buy minus sell volume per bar |
| **Trade Count** | Time / Tick / ODB | Number of trades per bar |
| **OFI** | Time / Tick / ODB | Order Flow Imbalance: (buy_vol − sell_vol) / total_vol, range [−1, 1] |
| **OFI Σ EMA** | Time / Tick / ODB | Cumulative OFI smoothed with an EMA |
| **Trade Intensity** | ODB only | Trades per second per bar — reveals urgency; works best on ODB bars where duration varies |
| **Intensity Heatmap** | ODB only | Rolling log-quantile percentile heatmap of trade intensity, coloured blue→red |
| **RSI** | Time / Tick / ODB | 14-period Relative Strength Index subplot with 70/30 reference lines |
| **ZigZag** *(stub)* | — | Menu entry is visible but **renders nothing** — requires the `qta` crate which has been removed as it is not publicly available |
| **Open Interest** | Perps only | Open interest subplot (perpetual futures markets only) |

---

<div align="center">
  <img
    src="https://github.com/user-attachments/assets/baddc444-e079-48e5-82b2-4f97094eba07"
    alt="Flowsurface screenshot"
    style="max-width: 100%; height: auto;"
  />
</div>

## Key Features

### Chart & Panel Types

-   **Heatmap (Historical DOM):** Uses live trades and L2 orderbook to create a time-series heatmap chart. Supports customizable price grouping, different time aggregations, fixed or visible range volume profiles.
-   **Candlestick:** Traditional kline chart supporting both time-based and custom tick-based intervals.
-   **Footprint:** Price grouped and interval aggregated views for trades on top of a candlestick chart. Supports different clustering methods, configurable imbalance and naked-POC studies.
-   **Open Deviation Bar (ODB) Chart** *(fork addition — requires ClickHouse)*: Range-bar chart where each bar closes once price deviates a set percentage from its open. See threshold table in the "What's Different" section above.
-   **Time & Sales:** Scrollable list of live trades.
-   **DOM (Depth of Market) / Ladder:** Displays current L2 orderbook alongside recent trade volumes on grouped price levels.
-   **Comparison:** Line graph for comparing multiple data sources, normalized by kline `close` prices on a percentage scale.

### Other Features

-   Real-time sound effects driven by trade streams
-   Multi window/monitor support
-   Pane linking for quickly switching tickers across multiple panes
-   Persistent layouts and customizable themes with editable color palettes

##### Market data is received directly from exchanges' public REST APIs and WebSockets

#

#### Historical Trades on Footprint Charts:

-   By default, it captures and plots live trades in real time via WebSocket.
-   For Binance tickers, you can optionally backfill the visible time range by enabling trade fetching in the settings:
    -   [data.binance.vision](https://data.binance.vision/): Fast daily bulk downloads (no intraday).
    -   REST API (e.g., `/fapi/v1/aggTrades`): Slower, paginated intraday fetching (subject to rate limits).
    -   The Binance connector can use either or both methods to retrieve historical data as needed.
-   Fetching trades for Bybit/Hyperliquid is not supported, as both lack a suitable REST API. OKX is WIP.

## Installation

### Build from Source

#### Requirements

-   [Rust toolchain](https://www.rust-lang.org/tools/install)
-   [Git version control system](https://git-scm.com/)
-   System dependencies:
    -   **Manjaro / Arch Linux:**
        ```bash
        sudo pacman -S base-devel rustup libx11 libxrandr libxi libxcursor libxkbcommon wayland mesa dbus alsa-lib
        rustup default stable
        ```
    -   **Debian / Ubuntu:**
        ```bash
        sudo apt install build-essential pkg-config libx11-dev libxrandr-dev libxi-dev libxcursor-dev libxkbcommon-dev libwayland-dev libdbus-1-dev libasound2-dev
        ```
    -   **Fedora:**
        ```bash
        sudo dnf install gcc make libX11-devel libXrandr-devel libXi-devel libXcursor-devel libxkbcommon-devel wayland-devel dbus-devel alsa-lib-devel
        ```
    -   **macOS**: Install Xcode Command Line Tools: `xcode-select --install`
    -   **Windows**: No additional dependencies required

#### Clone and run

```bash
# Clone this fork
git clone https://github.com/echocmd/flowsurface
cd flowsurface

# Build and run (release mode recommended for UI responsiveness)
cargo run --release
```

### Credits and thanks to

-   [Kraken Desktop](https://www.kraken.com/desktop) (formerly [Cryptowatch](https://blog.kraken.com/product/cryptowatch-to-sunset-kraken-pro-to-integrate-cryptowatch-features)), the main inspiration that sparked this project
-   [Halloy](https://github.com/squidowl/halloy), an excellent open-source reference for the foundational code design and the project architecture
-   And of course, [iced](https://github.com/iced-rs/iced), the GUI library that makes all of this possible
