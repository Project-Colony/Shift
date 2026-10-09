# Shift

**A fast, local image viewer built in Rust.**

Shift is a lightweight, keyboard-friendly desktop image viewer built with [Iced](https://iced.rs). It is part of the Colony family of small, focused desktop tools.

[![Rust](https://img.shields.io/badge/Rust-2024-black?logo=rust)](#build-from-source)
[![UI](https://img.shields.io/badge/UI-Iced-7c3aed)](https://iced.rs)
[![Status](https://img.shields.io/badge/status-prototype-8b5cf6)](#status)
[![License](https://img.shields.io/badge/license-GPL--3.0--or--later-blue.svg)](LICENSE)

> **Status:** early prototype. There are no releases yet, and the interface is currently in French only. Build it from source to try it.

## What it does

- Opens an image from a native file picker, or a whole folder from a native folder picker.
- Lists the other supported images in the same folder and lets you move through them.
- Fit-to-window and manual zoom, with mouse wheel zoom and a scrollable zoomed view.
- A fullscreen mode that hides the surrounding chrome.
- A status line with the position in the folder, file name, dimensions, file size and last modified time.
- Supported formats: PNG, JPEG, WebP, GIF and BMP.

### Keyboard shortcuts

| Keys | Action |
| --- | --- |
| `O` | Open an image |
| `D` | Open a folder |
| `Left` / `P` / `[` | Previous image |
| `Right` / `N` / `]` | Next image |
| `Home` / `End` | First / last image |
| `+` / `=` / `.` | Zoom in |
| `-` / `,` | Zoom out |
| `0` / `F` | Toggle fit and manual zoom |
| `1` / `R` | Reset zoom |
| `M` | Toggle fullscreen |

## Privacy

Shift has no network access and collects nothing. It reads the image you open and the other images in the same folder, and nothing else.

## Build from source

Requires Rust 1.88 or newer.

```bash
git clone https://github.com/Project-Colony/Shift.git
cd Shift
cargo run --release
```

## License

Shift is licensed under the [GNU General Public License v3.0 or later](LICENSE).
