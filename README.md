# QSnap

**High-Performance Video Splitter, Canvas Compositor & Anti-Detection Studio in Rust**  
*Native Linux & Windows • Pure Rust 2021 • Hardware-Accelerated Slint GUI • FFmpeg Engine*

---

### Overview

**QSnap** is an open-source desktop application designed for high-throughput creation of vertical (9:16) video clips from long-form source media. It combines primary footage with secondary retention layers (such as B-roll or gameplay sequences), dynamic informational banners, and anti-detection filtering to produce unique video outputs.

---

### Key Features

1. **Split-Screen Canvas Engine (9:16)**
   - Configurable vertical split ratios (e.g. 50/50, 60/40).
   - Seamless loop integration for secondary background footage.
   - Dynamic central promotional divider with custom typography and color schemes.

2. **Anti-Detection & Uniqueization Filters**
   - Micro-speed adjustments (1.01x to 1.04x).
   - Audio pitch shifting and jitter.
   - Dynamic micro-zoom and crop variations.
   - Subtle digital noise and grain layer.
   - Automatic metadata purging (`-map_metadata -1`).

3. **Batch Generation Pipeline**
   - Automated timeline slicing across arbitrary clip counts and durations.
   - Non-blocking asynchronous processing via Tokio and background worker threads.
   - Real-time progress tracking and live status reporting.

---

### Architecture

```
┌────────────────────────────────────────────────────────┐
│                   QSnap Slint GUI                      │
│      (Templates • Media Library • Anti-Fraud)          │
└───────────────────────────┬────────────────────────────┘
                            │ (Events & Callbacks)
┌───────────────────────────▼────────────────────────────┐
│                   Rust Core Pipeline                   │
│   • Tokio Task Dispatcher                              │
│   • Layout Template Parser                             │
│   • Anti-Fraud Parameter Generator                     │
│   • Complex FFmpeg Filtergraph Builder                 │
└───────────────────────────┬────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────┐
│                     FFmpeg 9.0+                        │
│     Hardware Encoding / Filter Graph Compositor        │
└────────────────────────────────────────────────────────┘
```

---

### Building from Source

#### Prerequisites
- Rust 1.80+ (`cargo`)
- FFmpeg 6.0+ (`ffmpeg`, `ffprobe`)

```bash
git clone https://github.com/Qyzom/QSnap.git
cd QSnap
cargo build --release
```

The resulting binary will be placed in `target/release/qsnap`.

---

### License

Distributed under the MIT License.
