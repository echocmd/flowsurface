// GitHub Issue: https://github.com/terrylica/opendeviationbar-py/issues/97
//! Streaming ZigZag swing structure overlay for the main candle chart.
//!
//! NOTE: The `qta` crate (private dependency) has been removed to allow building
//! on Linux. This indicator is stubbed out and does nothing until `qta` is
//! available again.

use crate::chart::indicator::kline::KlineIndicatorImpl;
use crate::chart::{Message, ViewState};

use data::chart::PlotData;
use data::chart::kline::KlineDataPoint;
use exchange::unit::Price;
use exchange::{Kline, Trade};

use iced::widget::center;
use std::ops::RangeInclusive;

/// ZigZag overlay indicator (stubbed — renders nothing until `qta` crate is available).
#[derive(Default)]
pub struct ZigZagOverlayIndicator;

impl ZigZagOverlayIndicator {
    pub fn new() -> Self {
        Self::default()
    }
}

impl KlineIndicatorImpl for ZigZagOverlayIndicator {
    fn clear_all_caches(&mut self) {}

    fn clear_crosshair_caches(&mut self) {}

    fn element<'a>(
        &'a self,
        _chart: &'a ViewState,
        _visible_range: RangeInclusive<u64>,
    ) -> iced::Element<'a, Message> {
        center(iced::widget::text("")).into()
    }

    fn rebuild_from_source(&mut self, _source: &PlotData<KlineDataPoint>) {}

    fn on_insert_klines(&mut self, _klines: &[Kline]) {}

    fn on_insert_trades(
        &mut self,
        _trades: &[Trade],
        _old_dp_len: usize,
        _source: &PlotData<KlineDataPoint>,
    ) {
    }

    fn on_ticksize_change(&mut self, _source: &PlotData<KlineDataPoint>) {}

    fn on_basis_change(&mut self, _source: &PlotData<KlineDataPoint>) {}

    fn draw_overlay(
        &self,
        _frame: &mut iced::widget::canvas::Frame,
        _total_len: usize,
        _earliest_visual: usize,
        _latest_visual: usize,
        _price_to_y: &dyn Fn(Price) -> f32,
        _interval_to_x: &dyn Fn(u64) -> f32,
        _palette: &iced::theme::palette::Extended,
    ) {
        // ZigZag overlay disabled (qta crate unavailable).
    }
}
