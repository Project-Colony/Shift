# Shift Private

**A fast, local image viewer for Linux, built in Rust.**

Shift Private is a modern alternative to `imv`: lightweight, responsive, keyboard-friendly, and designed to feel sharper, cleaner, and more premium.

Built for the Colony ecosystem, but useful on its own.

[![Rust](https://img.shields.io/badge/Rust-2024-black?logo=rust)](#build-from-source)
[![UI](https://img.shields.io/badge/UI-Iced-7c3aed)](#stack)
[![Platform](https://img.shields.io/badge/platform-Linux-1793d1?logo=linux)](#status)
[![Status](https://img.shields.io/badge/status-prototype-8b5cf6)](#status)
[![License](https://img.shields.io/badge/license-MIT-blue.svg)](LICENSE)

---

## Why Shift Private?

Most image viewers are either:

- extremely fast, but visually bare,
- comfortable, but too heavy,
- or built like generic file tools instead of something you actually want to keep open.

Shift Private aims for a better balance:

- **fast startup**
- **smooth navigation**
- **deep zoom**
- **clean minimal UI**
- **local-first workflow**
- **a stronger visual identity**

Not a bloated asset manager.
Not a generic gallery app.
A serious local viewer, with taste.

---

## Current prototype

The current prototype already supports:

- opening an image from a native file picker,
- displaying the selected image,
- indexing sibling images in the same directory,
- navigating previous / next within the folder,
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
- [x] Local folder indexing
- [x] Previous / next navigation

Planned next:

- [ ] real folder opening
- [ ] fit-to-window
- [ ] zoom in / zoom out
- [ ] pan / image movement
- [ ] keyboard shortcuts
- [ ] cleaner status bar
- [ ] visual polish

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

It is meant to feel personal, local, elegant, and fast, while staying useful as a standalone Linux image viewer.

---

## Roadmap

### Viewer MVP

- [x] Open image
- [x] Detect other images in current folder
- [x] Navigate between images
- [ ] Open folder directly
- [ ] Fit image to viewport
- [ ] Zoom controls
- [ ] Pan controls
- [ ] Keyboard-first navigation
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
