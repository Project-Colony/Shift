<div align="center">

# Shift

**A fast, local image viewer for people who browse folders of photos and screenshots from the keyboard.**

</div>

[![License: GPL-3.0-or-later](https://img.shields.io/badge/License-GPL--3.0--or--later-blue.svg)](LICENSE)
[![Colony app](https://img.shields.io/badge/Colony-graphics-purple)](https://github.com/Project-Colony/Colony)
[![Platforms](https://img.shields.io/badge/platforms-linux%20%7C%20windows%20%7C%20macOS-lightgrey)](#installation)

Looking through a folder of screenshots usually means a photo manager that
wants to import it into a library first, or a different built-in viewer on
every operating system. Shift opens one image, or a whole folder, and steps
through the folder in natural order from the keyboard. It works offline, keeps
no library and never changes your files.

> **Status:** early and deliberately small. The viewer logic is covered by unit
> tests that drive the real `update()` function: opening a file or a folder,
> unreadable images and empty folders, navigation, zoom, natural sort order and
> the status line. CI runs them on Linux, Windows and macOS and on the Rust 1.88
> floor, with clippy and rustfmt. Drawing, the keyboard and mouse bindings,
> fullscreen and the file pickers have no automated tests. The interface is in
> French only for now.

## Why Shift

- **No import step.** Photo managers keep a catalog of your pictures. Shift
  reads the folder as it is on disk, every time, and stores nothing about it.
- **Natural order.** `shot-2.png` comes before `shot-10.png`, so numbered
  screenshots and camera files stay in sequence.
- **The same viewer everywhere.** One binary per platform for Linux, Windows
  and macOS, with the same keys on all three.
- **Keys for everything.** Open, move through the folder, zoom, fit and go
  fullscreen without reaching for the mouse.

## What it does

- Opens an image from a native file picker, or a whole folder from a native folder picker.
- Lists the other supported images in the same folder and lets you move through them.
- Fit-to-window, which shrinks large images to the window or screen and leaves small ones at their real size.
- Manual zoom, with mouse wheel zoom and a scrollable zoomed view.
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

## Installation

### Via Colony (recommended)

Search for **Shift** in [Colony](https://github.com/Project-Colony/Colony) and
install it. Updates arrive through the launcher.

### Direct binary download

Grab the asset for your platform from the
[latest release](../../releases/latest):

| Platform | Asset |
|---|---|
| Linux | `shift-linux` |
| Windows | `shift-windows.exe` |
| macOS (Apple Silicon) | `shift-macos` |
| macOS (Intel) | `shift-macos-x86` |

Each asset comes with `.sig`, `.meta` and `.meta.sig` files. The
[Code signing policy](#code-signing-policy) below shows how to check them.

```bash
chmod +x shift-linux && ./shift-linux
```

### Build from source

```bash
git clone https://github.com/Project-Colony/Shift
cd Shift
cargo build --release
```

Requires Rust 1.88 or newer.

The binary is built as `target/release/shift` (`shift.exe` on Windows).
`shift --version` prints the version and exits without opening a window.

## Code signing policy

Every release asset is signed by the Project Colony organisation in CI, never on
a developer machine. Next to each asset on the release page:

| File | What it is |
|---|---|
| `<asset>.sig` | an ed25519 signature over the asset, made with the organisation's release key |
| `<asset>.meta` | three lines binding the asset to its file name, its sha256 and the release version |
| `<asset>.meta.sig` | an ed25519 signature over the `.meta` |

The private key is an organisation secret, used only by the shared
[sign-and-publish workflow](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/.github/workflows/sign-and-publish.yml)
in a job that builds nothing; the jobs that compile Shift never see it.
Colony checks all three files before it installs or updates Shift, and
refuses a release older than the one installed. To check a download yourself
with OpenSSL 3 (the same commands work for every asset):

```bash
cat > colony-release.pub <<'EOF'
-----BEGIN PUBLIC KEY-----
MCowBQYDK2VwAyEARNjg3Nn8H6/aBg1unwGjkUTcrdTxERNefVaqU8cFu0s=
-----END PUBLIC KEY-----
EOF
a=shift-linux
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in "$a" -sigfile "$a.sig"
openssl pkeyutl -verify -pubin -inkey colony-release.pub -rawin -in "$a.meta" -sigfile "$a.meta.sig"
cat "$a.meta"     # version=<tag>, asset=<file name>, sha256=<digest>
sha256sum "$a"    # the digest must equal the sha256 line
```

How releases are built, signed and published:
[design/releases.md](https://github.com/Project-Colony/Project-Colony-Resources/blob/main/design/releases.md#5-signing).

## Privacy

Shift sends no telemetry, no analytics and no crash reports.

| Data | Stored or sent | Where, and why |
|---|---|---|
| The image you open | read | to show it, with its size and last modified time in the status line |
| The folder it is in | read | listed to find the other supported images, so you can move through them |
| Settings and history | not kept | Shift writes no files: nothing is remembered between runs |

This program will not transfer any information to other networked systems
unless specifically requested by the user or the person installing or operating
it. Shift has no network code at all: it connects to no server.

## License

GPL-3.0-or-later. See [LICENSE](LICENSE).
