# QSnap

**High-Performance Video Splitter, Canvas Compositor & Anti-Detection Studio in Rust**  
*Native Linux & Windows • Pure Rust 2021 • Hardware-Accelerated Slint GUI • NVIDIA NVENC & FFmpeg Engine*

---

### Overview

**QSnap** is an open-source desktop application designed for high-throughput creation of vertical (9:16) video clips from long-form source media. It combines primary footage with secondary retention layers, dynamic central promotional dividers, and multi-parameter anti-detection filtering to produce unique video outputs.

---

### Key Features

1. **Hardware-Accelerated Split-Screen Engine**
   - Native NVIDIA NVENC (`h264_nvenc`) GPU encoding with automatic CPU fallback (`libx264`).
   - Configurable vertical split ratios (50/50, 60/40) with seamless B-roll looping.
   - Dynamic central promotional divider with custom typography and color schemes.
   - Single-frame preview rendering in <1 second directly from the interface.

2. **Smart Scene Detection & Snapping**
   - Automatically probes natural scene transitions using FFmpeg threshold algorithms.
   - Snaps clip boundaries to natural visual cuts instead of arbitrary slicing.

3. **Anti-Detection & Uniqueization Engine**
   - Micro-speed adjustments (1.01x to 1.04x).
   - Audio pitch shifting and jitter.
   - Dynamic micro-zoom and crop variations (2% to 4%).
   - Subtle digital noise and grain layer.
   - Automatic metadata purging (`-map_metadata -1`).

4. **Batch Generation Pipeline**
   - Multi-threaded asynchronous rendering powered by Tokio and background worker threads.
   - Real-time progress tracking, time estimates, and live console logging.
   - Strict monochrome design system built with hardware-accelerated Slint.

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
│   • Scene Change Detector & Boundary Snapper           │
│   • Anti-Fraud Parameter Generator                     │
│   • Complex FFmpeg Filtergraph Builder                 │
└───────────────────────────┬────────────────────────────┘
                            │
┌───────────────────────────▼────────────────────────────┐
│             NVIDIA NVENC / FFmpeg 9.0+                 │
│     Hardware GPU Encoding / Filter Graph Compositor    │
└────────────────────────────────────────────────────────┘
```

---

### Building and Testing

#### Prerequisites
- Rust 1.80+ (`cargo`)
- FFmpeg 6.0+ (`ffmpeg`, `ffprobe`)

```bash
# Run unit tests
cargo test

# Build optimized release binary
cargo build --release
```

The resulting binary will be placed in `target/release/qsnap`.

---

### License

Distributed under the MIT License.
