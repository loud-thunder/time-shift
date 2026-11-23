# Time-Shift

<div align="center">

**A high-performance procedural animation studio built in Rust for crafting seamless, infinitely tileable, pure CSS-animated vector backgrounds and geometric patterns.**

[![Rust](https://img.shields.io/badge/rust-2021%20edition-orange.svg?style=flat-square&logo=rust)](https://www.rust-lang.org/)
[![egui](https://img.shields.io/badge/GUI-egui%200.31-blue.svg?style=flat-square)](https://github.com/emilk/egui)
[![Tests](https://img.shields.io/badge/tests-7%2F7%20passing-brightgreen.svg?style=flat-square)](https://github.com/loud-thunder/time-shift)
[![Zero JS](https://img.shields.io/badge/output-zero--runtime%20CSS%20SVG-informational.svg?style=flat-square)]()
[![License](https://img.shields.io/badge/license-MIT-green.svg?style=flat-square)](#license)

[Features](#features) • [Quickstart](#quickstart) • [Generators](#procedural-generators) • [Presets](#curated-presets) • [Architecture](#architecture--deep-dive) • [Web Integration](#web-integration-guide)

</div>

---

## Overview

Modern web applications and landing pages require dynamic, high-fidelity visual backgrounds. Traditionally, teams rely on heavy video loops, Lottie runtimes, or client-side Canvas/WebGL scripts that introduce noticeable layout shifts, runtime bloat, and battery drain.

**Time-Shift** solves this by providing a native desktop CAD-like creative studio written in Rust. It enables designers and developers to procedurally synthesize geometric patterns, choreograph multi-track keyframe animations with spatial wave propagation, preview them in real-time at 60+ FPS, and export them as **lightweight, standalone SVGs with pure CSS `@keyframes`**.

The resulting assets run natively inside any web browser with **zero external JavaScript dependencies**, full GPU compositing, and seamless toroidal edge continuity across infinite 2D planes.

```
┌───────────────────────────┐      ┌─────────────────────────────┐      ┌──────────────────────────────┐
│   Procedural Generators   │      │    Multi-Track Keyframes    │      │    Output & Integration      │
│  • Polka / Halftone       │      │  • Transforms (Scale/Rot)   │      │  • Standalone W3C SVG        │
│  • Sine Ribbon Waves      │ ───► │  • Color / Opacity Cycles   │ ───► │  • Pure CSS @keyframes       │
│  • Truchet Labyrinths     │      │  • Path Morphing            │      │  • 60+ FPS GPU Acceleration  │
│  • Isometric Hex Meshes   │      │  • 2D Spatial Phase Waves   │      │  • Zero JavaScript Runtime   │
│  • Concentric Ripples     │      └─────────────────────────────┘      └──────────────────────────────┘
│  • Vector Flow Fields     │
└───────────────────────────┘
```

---

## Features

- **Standalone CSS-Animated SVG Export**: Exports pure, self-contained SVG files containing native `<style>` sheets and CSS `@keyframes`. Uses modern `transform-box: fill-box; transform-origin: center;` for hardware-accelerated rendering around local element centroids.
- **Toroidal Tiling & Infinite Continuity**: Every procedural algorithm and phase calculation strictly enforces periodic boundary conditions ($C^0$ continuity) across $X$ and $Y$ tile boundaries, eliminating visible seams when repeated as backgrounds.
- **2D Spatial Phase Propagation**: Propagate animation delays across UV coordinates ($\Delta\phi = (u \cdot f_x + v \cdot f_y) \pmod 1$) to generate mesmerizing cascading wave fronts, radial ripples, and synchronized ripples across repeating tiles.
- **Multi-Track Animation Engine**: Independent layer control over:
  - **Transform Track**: Centroid scale, rotation, skew, and translation.
  - **Color & Opacity Track**: Hue shifting, alpha transitions, and strobe effects.
  - **Path Morph Track**: Interpolated vector geometry modifications.
- **Immediate-Mode Studio GUI**: Powered by `egui` and `eframe` for sub-millisecond frame updates, featuring:
  - Interactive infinite pan and zoom canvas.
  - Real-time multi-tile grid preview (1×1 up to 6×6) with toggleable tile boundary overlays.
  - Transport bar with Play/Pause, speed scaling (0.1× to 4.0×), and bidirectional timeline scrubbing (0–30s).
  - Built-in SVG markup inspector and one-click clipboard copy.
- **Browser-Parity sRGB Compositor**: Implements exact sRGB premultiplied alpha math (`GL_ONE`, `GL_ONE_MINUS_SRC_ALPHA`) ensuring pixel-for-pixel color and opacity parity between the native preview canvas and browser SVG rendering engines.
- **Designer Curated Palettes & Presets**: Ships with 8 calibrated light palettes and 6 production-grade starter presets for instant design inspiration.

---

## Procedural Generators

Time-Shift ships with 6 procedural generation engines, each exposing deep parameterization for architectural and graphic patterns:

| Generator | Engine Characteristics | Configurable Parameters |
|:---|:---|:---|
| **Polka Dots & Halftone** | Regular and staggered matrix distributions with dynamic sizing | Columns/Rows, Staggering, Motif Shapes (*Circle, Square, Cross, Star, Diamond*), Size Modulation Amplitude & 2D Frequency |
| **Sine Ribbon Waves** | Continuous harmonic oscillating bands with guaranteed tile continuity | Band count, Orientation (*Horizontal, Vertical, Diagonal*), Amplitude, Harmonic ratio, Ribbon thickness, Sample resolution |
| **Truchet Tiles** | Dual-arc and geometric maze networks with topological connectivity | Columns/Rows, Variants (*Quarter Circles, Diagonal Lines, Triangles*), Layout modes (*Hash Seed, Checkerboard, Diagonal Wave*), Arc Radii |
| **Isometric / Hex Grid** | 2.5D axonometric projection and honeycomb tessellations | Tessellation Modes (*Isometric Cubes, Hex Honeycomb, Triangular Mesh*), Inset ratios, Facet shading |
| **Concentric Ripples** | Multi-center circular interference fields | Center grid distribution, Rings per center, Maximum radius ratio, Exponential ring spacing, Seeded jitter |
| **Flow Field Dashes** | Vector velocity fields with smooth angle transitions | Columns/Rows, Dash length ratio, Spatial frequencies ($f_x, f_y$), Angle amplitude, Curl phase, Arrowheads, Cap styling |

---

## Curated Presets

Jumpstart your designs with built-in, meticulously tuned animations accessible from the top toolbar:

- **Cyberpunk Isometric Matrix**: Pulsing 3D isometric cubes paired with directional flow field vector arrows.
- **Synthwave Ocean Ripples**: Morphing sine ribbons with spatial phase propagation over a halftone sun horizon.
- **Kinetic Truchet Labyrinth**: Color-cycling quarter-circle geometric tiles layered with counter-rotating cross motifs.
- **Bioluminescent Ripple Pond**: Toroidal concentric water rings expanding rhythmically over a luminescent dot lattice.
- **Magnetic Vector Field**: Oscillating directional vector dashes over an architectural hexagonal mesh.
- **Retro Halftone Spin**: Staggered star motifs executing wave-propagated rotation and scale pulses.

---

## Quickstart

### Prerequisites

Ensure you have the Rust toolchain installed (Rust 1.75+ recommended):

```bash
curl --proto '=https' --tlsv1.2 -sSf https://sh.rustup.rs | sh
```

### Build & Run

Clone the repository and launch the native desktop application in release mode:

```bash
git clone https://github.com/loud-thunder/time-shift.git
cd time-shift

# Run with release optimizations for 60+ FPS immediate-mode canvas
cargo run --release
```

### Run Automated Tests

Time-Shift includes automated test coverage validating generator outputs, color premultiplication, keyframe continuity, and SVG compliance:

```bash
cargo test
```

---

## Web Integration Guide

The exported SVGs are 100% self-contained and require no client-side JavaScript. Here is how to integrate them into modern web stacks:

### 1. CSS Background Pattern (Infinite Seamless Tiling)

Save your pattern as `background.svg` in your static assets folder:

```css
.hero-section {
  width: 100%;
  min-height: 100vh;
  background-color: #ffffff;
  background-image: url('/assets/background.svg');
  background-repeat: repeat;
  background-size: 240px 240px; /* Match exported tile dimensions */
}
```

### 2. Tailwind CSS

```html
<div class="w-full min-h-screen bg-repeat bg-[url('/assets/background.svg')] bg-[length:240px_240px]">
  <div class="container mx-auto px-6 py-24 backdrop-blur-sm bg-white/70 rounded-2xl shadow-xl">
    <h1 class="text-4xl font-extrabold text-slate-900">Modern SaaS Product</h1>
    <p class="text-slate-600 mt-4">Powered by seamless, hardware-accelerated vector animations.</p>
  </div>
</div>
```

### 3. Next.js / React Component

```tsx
import Image from 'next/image';

export function AnimatedBackground() {
  return (
    <div
      className="fixed inset-0 -z-10 bg-repeat opacity-90 pointer-events-none"
      style={{
        backgroundImage: "url('/patterns/time-shift-pattern.svg')",
        backgroundSize: '240px 240px',
      }}
    />
  );
}
```

---

## Architecture & Deep Dive

Time-Shift is engineered with clean separation between data models, procedural evaluation, immediate-mode rendering, and SVG compilation:

```
src/
├── main.rs                   # Application entry point & native eframe runtime options
├── presets.rs                # Curated palettes & production starter presets
├── model/                    # Pure declarative state definitions
│   ├── mod.rs
│   ├── project.rs            # Project, layer stack, and global tile parameters
│   ├── generators.rs         # Generator configurations, motifs, and enum variants
│   └── animation.rs          # Keyframe tracks, easing curves, and interpolation types
├── engine/                   # High-performance procedural & animation engines
│   ├── mod.rs
│   ├── primitives.rs         # Abstract vector path representation (Move, Line, Cubic, Close)
│   ├── generator_eval.rs     # Mathematical generators producing geometry primitives
│   └── animation_eval.rs     # Spatial phase offsets, easing functions, and stop lerping
├── ui/                       # egui immediate-mode interface
│   ├── mod.rs
│   ├── app.rs                # Main application loop, transport bar, top menu
│   ├── canvas.rs             # Multi-tile 2D canvas, pan/zoom, sRGB premultiplied rendering
│   ├── layer_panel.rs        # Layer management & procedural parameter controls
│   └── animation_panel.rs    # Keyframe timeline editor, inspector & file exporter
└── export/                   # W3C-compliant exporter
    ├── mod.rs
    └── svg_exporter.rs       # Pure CSS @keyframes generator and SVG document builder
```

### Mathematical & Engineering Highlights

#### 1. Toroidal UV Spatial Phase Delay
To achieve smooth wave propagation without breaking tile boundaries, phase offsets are calculated across normalized UV coordinates $(u, v) \in [0, 1]^2$ with integer spatial frequencies:

$$\Delta\phi(u, v) = \left( u \cdot f_x + v \cdot f_y \right) \bmod 1$$

Because $f_x, f_y \in \mathbb{Z}$, the phase at $u = 0$ is identical to $u = 1$, ensuring continuous seamless loops across all adjacent tiles in an infinite tiling plane.

#### 2. Hardware-Accelerated CSS `@keyframes` Generation
Rather than recalculating animated SVG matrices on every frame with JavaScript, Time-Shift generates W3C-compliant CSS animations using:

```css
.l1-tf {
  transform-box: fill-box;
  transform-origin: center;
  animation: l1-tf 4.00s ease-in-out infinite normal;
}
```

Each element receives an inline `animation-delay` based on its spatial position, offloading the entire rendering and interpolation burden directly to the browser's GPU compositor.

#### 3. Exact sRGB Premultiplied Alpha Parity
In graphics engines using non-linear sRGB, blending translucent elements over light or dark backgrounds can cause subtle discoloration (e.g. cyan shifts or darkened fringes). Time-Shift implements exact sRGB premultiplication:

```rust
let pr = ((r as u32 * a as u32 + 127) / 255) as u8;
let pg = ((g as u32 * a as u32 + 127) / 255) as u8;
let pb = ((b as u32 * a as u32 + 127) / 255) as u8;
```

This guarantees visual consistency between the native `egui` preview and Chromium / WebKit SVG compositors.

---

## Tech Stack

- **Language**: [Rust](https://www.rust-lang.org/) (2021 Edition) — Type safety, memory safety, zero-cost abstractions.
- **GUI Framework**: [egui](https://github.com/emilk/egui) / [eframe](https://docs.rs/eframe/latest/eframe/) (v0.31) — Immediate mode GUI with immediate repaint scheduling.
- **File Dialogs**: [rfd](https://github.com/PolyMeilex/rfd) (v0.15) — Native asynchronous OS file pickers.
- **Vector Output**: Clean W3C SVG 1.1 / CSS Animations Level 1 markup.

---

## License

This project is licensed under the [MIT License](LICENSE).
