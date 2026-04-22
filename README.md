# Shift Private

**A fast, local image viewer built in Rust.**

Shift Private is a modern image viewer designed to be lightweight, responsive, keyboard-friendly, and visually clean, with a stronger identity than the usual utility-style viewers.

Originally inspired by the speed and simplicity of tools like `imv`, Shift Private is **not Linux-only** and is intended to grow as a real cross-platform desktop viewer for:

- Linux
- Windows
- macOS (Apple Silicon)
- macOS (Intel)

Built in the Colony spirit, but useful well beyond it.

[![Rust](https://img.shields.io/badge/Rust-2024-black?logo=rust)](#build-from-source)
[![UI](https://img.shields.io/badge/UI-Iced-7c3aed)](#stack)
[![Platforms](https://img.shields.io/badge/platforms-Linux%20%7C%20Windows%20%7C%20macOS-lightgrey)](#status)
[![Status](https://img.shields.io/badge/status-prototype-8b5cf6)](#status)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Why Shift Private?

Most image viewers lean too far in one direction.

They are often either:

- extremely fast, but visually bare,
- comfortable, but too heavy,
- or built like generic file browsers instead of tools you actually want to keep open.

Shift Private aims for a better balance:

- **fast startup**
- **smooth navigation**
- **deep zoom**
- **clean minimal UI**
- **local-first workflow**
- **strong cross-platform foundation**
- **a sharper visual identity**

Not a bloated asset manager.
Not a generic gallery app.
A serious local viewer, with taste.

---

## Current prototype

The current prototype already supports:

- opening an image from a native file picker,
- opening a folder directly from a native folder picker,
- displaying the selected image,
- indexing sibling images in the same directory,
- navigating through the folder with buttons and keyboard shortcuts,
- basic zoom controls with fit/manual modes,
- mouse wheel zoom support,
- manual mode with a scrollable zoomed viewer,
- a cleaner viewer status line with current file metadata,
- compiling cleanly on the current Rust + Iced stack.

This is the foundation, not the finished experience.

---

## Project goals

### Core viewer experience

Shift Private is being built around a few priorities:

- **instant feeling** when opening images,
- **strong keyboard flow**,
- **precise zoom and framing**,
- **fit-to-window done right**,
- **low memory footprint**,
- **an interface that stays out of the way**.

### Long-term direction

Once the viewer core feels excellent, the project can grow into a richer local gallery:

- thumbnails / filmstrip
- folder browsing
- favorites
- tags
- collections
- wallpaper helpers
- better format support
- personal gallery features

The rule is simple: **nail the viewer first, expand second.**

---

## Status

Shift Private is currently in **prototype stage**.

Implemented today:

- [x] Rust desktop foundation
- [x] Iced application shell
- [x] Native image picker
- [x] Native folder picker
- [x] Local folder indexing
- [x] Previous / next navigation
- [x] Keyboard-first navigation shortcuts
- [x] Basic fit / zoom controls
- [x] Mouse wheel zoom
- [x] Scrollable manual zoom mode
- [x] Quick shortcut help panel
- [x] Cleaner status bar
- [x] Visual polish on the empty state

Planned next:

- [ ] pan / image movement
- [ ] smarter fit-to-viewport behavior
- [ ] smoother zoom feel
- [ ] richer visual polish
- [ ] metadata layout cleanup
- [ ] large-folder handling

---

## Stack

- **Rust** for speed, safety, and maintainability
- **Iced** for the desktop UI
- **rfd** for native dialogs
- **image** for image handling and decoding support

---

## Design philosophy

Shift Private is intentionally being built with restraint.

The goal is not to over-engineer a huge gallery platform on day one.
The goal is to build a viewer that feels good immediately, then grow from a solid core.

That means:

- simple architecture first,
- clear responsibilities,
- minimal unnecessary state,
- performance-sensitive decisions,
- and product polish that matters in daily use.

---

## Build from source

```bash
git clone https://github.com/MotherSphere/Shift_Private.git
cd Shift_Private
cargo run
```

Optimized build:

```bash
cargo run --release
```

---

## Position in the Colony ecosystem

Shift Private is part of the broader Colony spirit:

small, focused desktop tools built with care.

It is meant to feel personal, local, elegant, and fast, while also standing on its own as a cross-platform image viewer.

---

## Roadmap

### Viewer MVP

- [x] Open image
- [x] Open folder directly
- [x] Detect other images in current folder
- [x] Navigate between images
- [x] Fit / manual display modes
- [x] Zoom controls
- [x] Keyboard-first navigation
- [x] Scrollable manual exploration
- [x] Quick in-app help panel
- [ ] Pan controls
- [ ] Better layout and visual hierarchy

### After MVP

- [ ] Filmstrip / thumbnails
- [ ] Smart preloading
- [ ] Large-folder handling
- [ ] Favorites
- [ ] Local metadata / tags
- [ ] Stronger Colony visual language

---

## Name

**Shift Private** is meant to sound personal and discreet.

Not a loud media suite.
Not a cluttered library manager.
Just a fast, intimate viewer that feels good to keep nearby.
