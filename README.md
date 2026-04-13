# Flowsurface

[![Crates.io](https://img.shields.io/crates/v/flowsurface)](https://crates.io/crates/flowsurface)
[![Lint](https://github.com/flowsurface-rs/flowsurface/actions/workflows/lint.yml/badge.svg)](https://github.com/flowsurface-rs/flowsurface/actions/workflows/lint.yml)
[![Format](https://github.com/flowsurface-rs/flowsurface/actions/workflows/format.yml/badge.svg)](https://github.com/flowsurface-rs/flowsurface/actions/workflows/format.yml)
[![License: GPL v3](https://img.shields.io/badge/License-GPLv3-blue.svg)](https://github.com/flowsurface-rs/flowsurface/blob/main/LICENSE)
[![Made with iced](https://iced.rs/badge.svg)](https://github.com/iced-rs/iced)

An experimental open-source desktop charting application. Supports Binance, Bybit, Hyperliquid and OKX

<div align="center">
  <img
    src="https://github.com/user-attachments/assets/baddc444-e079-48e5-82b2-4f97094eba07"
    alt="Flowsurface screenshot"
    style="max-width: 100%; height: auto;"
  />
</div>

### Key Features

-   Multiple chart/panel types:
    -   **Heatmap (Historical DOM):** Uses live trades and L2 orderbook to create a time-series heatmap chart. Supports customizable price grouping, different time aggregations, fixed or visible range volume profiles.
    -   **Candlestick:** Traditional kline chart supporting both time-based and custom tick-based intervals.
    -   **Footprint:** Price grouped and interval aggregated views for trades on top of a candlestick chart. Supports different clustering methods, configurable imbalance and naked-POC studies.
    -   **Time & Sales:** Scrollable list of live trades.
    -   **DOM (Depth of Market) / Ladder:** Displays current L2 orderbook alongside recent trade volumes on grouped price levels.
    -   **Comparison:** Line graph for comparing multiple data sources, normalized by kline `close` prices on a percentage scale
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

### Method 1: Prebuilt Binaries

Standalone executables are available for Windows, macOS, and Linux on the [Releases page](https://github.com/flowsurface-rs/flowsurface/releases).

<details>
<summary><strong>Having trouble running the file? (Permission/Security warnings)</strong></summary>
 
Since these binaries are currently unsigned they might get flagged.

-   **Windows**: If you see a "Windows protected your PC" pop-up, click **More info** -> **Run anyway**.
-   **macOS**: If you see "Developer cannot be verified", control-click (right-click) the app and select **Open**, or go to _System Settings > Privacy & Security_ to allow it.
</details>

### Method 2: Build from Source

#### Requirements

-   [Rust toolchain](https://www.rust-lang.org/tools/install)
-   [Git version control system](https://git-scm.com/)
-   System dependencies:
    -   **Linux**:
        -   Debian/Ubuntu: `sudo apt install build-essential pkg-config libasound2-dev libx11-dev libxrandr-dev libxi-dev libxcursor-dev libxkbcommon-dev libwayland-dev libdbus-1-dev`
        -   Arch / Manjaro: `sudo pacman -S base-devel alsa-lib libx11 libxrandr libxi libxcursor libxkbcommon wayland mesa dbus`
        -   Fedora: `sudo dnf install gcc make alsa-lib-devel libX11-devel libXrandr-devel libXi-devel libXcursor-devel libxkbcommon-devel wayland-devel dbus-devel mesa-libGL-devel`
    -   **macOS**: Install Xcode Command Line Tools: `xcode-select --install`
    -   **Windows**: No additional dependencies required

#### Option A: `cargo install`

```bash
# Install latest globally
cargo install --git https://github.com/flowsurface-rs/flowsurface flowsurface

# Run
flowsurface
```

#### Option B: Cloning the repo

```bash
# Clone the repository
git clone https://github.com/flowsurface-rs/flowsurface

cd flowsurface

# Build and run
cargo build --release
cargo run --release
```

## Open Deviation Bars (ODB) — Local ClickHouse Setup

Open Deviation Bars are range bars that close when price deviates a configurable percentage from the bar's open.
Their microstructure data (trade count, OFI, intensity) is pre-computed by the
[opendeviationbar-py](https://github.com/terrylica/opendeviationbar-py) sidecar and stored in a local
[ClickHouse](https://clickhouse.com/) database. Without this database the ODB chart pane will show a
*"Fetching Klines…"* message indefinitely.

Standard time-based candlestick charts and all other panels work without any database setup.

### Resource usage (Docker container)

| Resource | Idle | Active (Flowsurface querying / ingestion) |
|----------|----|-------------------------------------------|
| RAM      | ≤ 500 MB | 1–4 GB (query-dependent) |
| CPU      | < 1 % | Brief spikes during bulk queries / ingestion |
| Disk     | Grows with data — plan for several GB per symbol/threshold pair over months |

### Quick start — spin up ClickHouse with Docker

**Requirements:** [Docker](https://docs.docker.com/get-docker/) and
[Docker Compose](https://docs.docker.com/compose/install/) (v2 / `docker compose` plugin).

```bash
# 1. Start ClickHouse (runs in the background)
docker compose up -d

# 2. Verify it is healthy
docker compose ps
# The "flowsurface-clickhouse" service should show "healthy".

# 3. Run Flowsurface — it will connect to localhost:8123 by default
cargo run --release
```

The `docker-compose.yml` automatically runs `clickhouse/init-db.sql` on first startup, creating the
`opendeviationbar_cache` database and `open_deviation_bars` table.

### Populate the database

The database starts **empty**. To see ODB charts you need to feed data into it:

```bash
# Clone and run the Python ingestion tool (requires Python 3.11+)
git clone https://github.com/terrylica/opendeviationbar-py
cd opendeviationbar-py
pip install -e .
# Follow that project's README to configure and start the sidecar.
# It will listen to Binance and write computed bars to your local ClickHouse.
```

### Override the ClickHouse connection

The application reads two environment variables:

| Variable              | Default     | Example override        |
|-----------------------|-------------|-------------------------|
| `FLOWSURFACE_CH_HOST` | `localhost` | `192.168.1.10`          |
| `FLOWSURFACE_CH_PORT` | `8123`      | `18123`                 |

```bash
FLOWSURFACE_CH_HOST=192.168.1.10 FLOWSURFACE_CH_PORT=18123 cargo run --release
```

### Credits and thanks to

-   [Kraken Desktop](https://www.kraken.com/desktop) (formerly [Cryptowatch](https://blog.kraken.com/product/cryptowatch-to-sunset-kraken-pro-to-integrate-cryptowatch-features)), the main inspiration that sparked this project
-   [Halloy](https://github.com/squidowl/halloy), an excellent open-source reference for the foundational code design and the project architecture
-   And of course, [iced](https://github.com/iced-rs/iced), the GUI library that makes all of this possible
