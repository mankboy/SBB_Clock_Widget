# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SBB (Swiss Federal Railways) Clock Widget — a browser-based analog clock that replicates the iconic SBB station clock design (Mondaine wall clock). The entire application is a single `index.html` file with embedded CSS and JavaScript. No build system, no dependencies, no package manager.

## Running

Open `index.html` directly in a browser. No server required. Use URL query parameters to configure: `?variant=dark&shadow=heavy`. Designed for use as a desktop widget via [Plash](https://github.com/nicklockwood/Plash).

## Files

- `index.html` — the entire application (CSS + HTML + JS)
- `sbb-cff-ffs.svg` — SBB CFF FFS logo displayed on the clock face
- Reference images (`*.jpg`, `*.webp`) — Mondaine product photos for design reference

## Architecture

Single-file application (`index.html`) with three embedded sections:

- **CSS (lines 7–218):** Clock styling, hand shapes, shadow elements, variant picker UI. Uses CSS custom properties (`--clock-bg`, `--clock-border`, `--hand-color`, `--sec-color`, `--clock-shadow`, `--hand-shadow-opacity`) for theming. Clock is 300px diameter with absolute-positioned hands.
- **HTML (lines 220–237):** Clock wrapper containing: clock container (dial, logo, shadow hands, real hands, center dot), variant picker, and URL hint.
- **JavaScript (lines 239–416):** Dial mark generation, variant/shadow system, picker UI construction, URL parameter handling, and `updateClock()` animation loop via `requestAnimationFrame`.

## SBB Clock Timing Logic

The clock implements the distinctive SBB timing behavior in `updateClock()`:

- **Second hand:** Completes full 360° rotation in 58.5 seconds (not 60), then pauses at 12 o'clock for 1.5 seconds before the next cycle
- **Minute hand:** Discrete jumps only (no smooth interpolation between minutes) — moves when the second hand resets
- **Hour hand:** Smooth continuous movement based on hours + fractional minutes
- **Backward-spin prevention:** CSS transition is temporarily removed (`style.transition = 'none'`) when the second hand resets from 360° to 0° to avoid a visible reverse animation
- **Resume bounce flick:** When the second hand resumes after the pause, a brief 0.3s cubic-bezier transition gives a characteristic snap before returning to smooth sweep

## Color Variants

Six variants matching the Mondaine SBB commerce range, selectable via picker buttons or `?variant=` URL param:

| Key | Name | Border | Background | Hands | Second |
|---|---|---|---|---|---|
| `classic` | Classic | Silver `#C0C0C0` | White | Black | Red |
| `white` | White | Light grey `#E8E8E8` | White | Black | Red |
| `black` | Black | Dark `#222222` | White | Black | Red |
| `dark` | Dark | Dark `#222222` | Black `#111111` | White | Red |
| `red` | Red | Red `#EB0000` | White | Black | Red |
| `gold` | Gold | Gold `#C5A55A` | White | Black | Red |

Default: `classic`. Applied via `applyVariant()` which sets CSS custom properties on `:root`.

## Shadow System

Four shadow presets, cycled via a toggle button or `?shadow=` URL param:

| Key | Clock shadow | Hand shadow opacity |
|---|---|---|
| `light` | `0 4px 12px rgba(0,0,0,0.1)` | 0.1 |
| `medium` | `0 8px 24px rgba(0,0,0,0.15)` | 0.2 |
| `heavy` | `0 12px 36px rgba(0,0,0,0.25)` | 0.3 |
| `none` | none | 0 |

Default: `medium`. Hand shadows are separate DOM elements (`.hand.shadow`) offset by 1px right and 1px down, not CSS filters.

## Key Implementation Details

- **Dial marks:** Generated programmatically in a loop (0–59), classified as `.hour` or `.minute` via `i % 5 === 0`
- **Hand rotation:** CSS `transform: rotate(Ndeg)` updated each animation frame
- **Tapered hands:** Hour and minute hands use `clip-path: polygon(...)` to create a bold taper — wider at the base, narrower at the tip
- **Hand tails:** Both hour and minute hands extend below center (tail portion) via offset `bottom` and `transform-origin` values
- **Minute snap:** CSS `transition` with bounce easing (`cubic-bezier(0.4, 2.08, 0.55, 0.44)`) for the characteristic snap effect
- **Second hand tip:** `::after` pseudo-element creates the iconic red circle; hand extends past center via `transform-origin: 50% 74.2%`
- **Logo:** `sbb-cff-ffs.svg` positioned at 35% from top (matching Mondaine reference), 80px wide, centered horizontally, with `pointer-events: none`
- **URL persistence:** `history.replaceState()` updates the URL without reload when variant/shadow changes; initial state read from `URLSearchParams`
- **Variant picker:** Circular swatch buttons with a nested `.swatch-inner` showing the background color; active state indicated by red border. Shadow button cycles through presets on click.
