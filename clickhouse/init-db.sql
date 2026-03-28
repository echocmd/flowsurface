-- Flowsurface local ClickHouse schema
-- Creates the database and table expected by the ODB (Open Deviation Bar) adapter.
--
-- This schema mirrors the opendeviationbar-py cache format.
-- Populate data by running: https://github.com/terrylica/opendeviationbar-py

CREATE DATABASE IF NOT EXISTS opendeviationbar_cache;

CREATE TABLE IF NOT EXISTS opendeviationbar_cache.open_deviation_bars
(
    -- Timestamps in microseconds (µs since Unix epoch)
    close_time_us              Int64,
    open_time_us               Nullable(Int64),

    -- OHLCV
    open                       Float64,
    high                       Float64,
    low                        Float64,
    close                      Float64,
    buy_volume                 Float64,
    sell_volume                Float64,

    -- Microstructure
    individual_trade_count     Nullable(UInt32),
    ofi                        Nullable(Float64),    -- Order Flow Imbalance [-1, 1]
    trade_intensity            Nullable(Float64),    -- trades per second

    -- Aggregated trade ID range (for gap-fill continuity checks)
    first_agg_trade_id         Nullable(UInt64),
    last_agg_trade_id          Nullable(UInt64),

    -- Partition / filter keys
    symbol                     String,              -- e.g. 'BTCUSDT'
    threshold_decimal_bps      UInt32,              -- e.g. 250 = BPR25 (0.25%)
    ouroboros_mode             String,              -- 'day' or 'month'

    -- Schema version tag (set by opendeviationbar-py sidecar)
    opendeviationbar_version   Nullable(String)
)
ENGINE = MergeTree()
ORDER BY (symbol, threshold_decimal_bps, ouroboros_mode, close_time_us);
