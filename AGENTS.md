# Developer & Contributor Guide (QSnap)

## Architecture Overview

QSnap is written in Rust 2021 with a Slint GUI and Tokio asynchronous runtime.

### Core Modules
- `src/templates/`: Split-screen layout configurations and JSON serialization.
- `src/antifraud/`: Algorithmic uniqueization parameters (speed, pitch, crop, grain, metadata stripping).
- `src/media/`: Probe duration via `ffprobe` and directory asset scanners.
- `src/render/`: FFmpeg filtergraph construction (`filter_complex`) and asynchronous execution.
- `ui/`: Hardware-accelerated Slint interface (`app.slint`).

## Code Standards
- All video filtergraph generation logic must strictly sanitize and escape user-supplied strings.
- Non-blocking I/O: Any long-running video encode task must be dispatched via `tokio::task::spawn_blocking` to avoid freezing the Slint GUI thread.
- Memory safety: Zero unsafe blocks in business logic.
