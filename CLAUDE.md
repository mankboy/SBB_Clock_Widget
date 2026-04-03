# CLAUDE.md

This file provides guidance to Claude Code (claude.ai/code) when working with code in this repository.

## Project Overview

SBB (Swiss Federal Railways) Clock Widget — a browser-based and standalone desktop analog clock that replicates the iconic Mondaine SBB station clock design. Dual-mode: runs as a single `index.html` in a browser/Plash, or as a native desktop widget via Tauri v2. No external JS/CSS dependencies.

## Running

### Browser / Plash mode
Open `index.html` directly in a browser. No server required. Use URL query parameters to configure: `?variant=dark&shadow=heavy`. Designed for use as a desktop widget via [Plash](https://github.com/nicklockwood/Plash).

### Standalone desktop app (Tauri)
```bash
npm install              # first time only
npm run dev              # development mode with hot reload
npm run build            # production build with installer
```
Requires: Node.js (LTS), Rust toolchain (`rustup`). On Linux also: `libwebkit2gtk-4.1-dev`, `libappindicator3-dev`, `librsvg2-dev`.

## Files

### Root (browser/Plash version)
- `index.html` — standalone browser clock (CSS + HTML + JS, single file)
- `sbb-cff-ffs.svg` — SBB CFF FFS logo displayed on the clock face
- `CLAUDE.md` — this file
- Reference images (`*.jpg`, `*.webp`) — Mondaine product photos for design reference
- `sbb-cff-ffs.xcf` — GIMP project file for the SVG logo

### Tauri desktop app
- `src/index.html` — adapted version with Tauri drag region, event listeners, store-based persistence
- `src/sbb-cff-ffs.svg` — logo copy for Tauri frontend
- `src-tauri/tauri.conf.json` — window config (frameless, transparent, alwaysOnBottom, skipTaskbar), bundle settings, app identity
- `src-tauri/src/lib.rs` — Rust entry point: system tray menu (variants, shadows, bring-to-front, quit), tray event handling
- `src-tauri/src/main.rs` — Rust main (calls `lib::run()`)
- `src-tauri/Cargo.toml` — Rust dependencies (tauri with tray-icon feature, tauri-plugin-store)
- `src-tauri/icons/` — app icons for all platforms (png, icns, ico)
- `src-tauri/capabilities/default.json` — Tauri v2 permission grants
- `package.json` — Node.js metadata, `@tauri-apps/cli` + `@tauri-apps/api` + `@tauri-apps/plugin-store`
- `.github/workflows/release.yml` — CI/CD: builds macOS (ARM64 + x86_64), Windows, Linux on tag push

## Architecture

### Browser version (`index.html`)
Single-file application with three embedded sections:

- **CSS (lines 7–213):** Clock styling, hand shapes with `clip-path: polygon()` tapers, `drop-shadow` filter on hands, variant picker UI. Uses CSS custom properties (`--clock-bg`, `--clock-border`, `--hand-color`, `--sec-color`, `--clock-shadow`, `--hand-shadow-opacity`) for theming. Clock is 300px diameter with 18px border, absolute-positioned hands.
- **HTML (lines 215–233):** Clock wrapper containing: clock container (dial, logo, shadow hands [hidden], real hands, center dot), variant picker (hidden by default), and URL hint.
- **JavaScript (lines 235–415):** Dial mark generation, variant/shadow system, picker UI construction, URL parameter handling, and `updateClock()` animation loop via `requestAnimationFrame`.

### Tauri desktop version (`src/index.html` + `src-tauri/`)
- **Frontend** (`src/index.html`): Same clock code adapted with `data-tauri-drag-region` on wrapper for window dragging, `@tauri-apps/plugin-store` for settings persistence, and Tauri event listeners (`set-variant`, `set-shadow`) for tray menu communication. Falls back to URL params when not in Tauri.
- **Backend** (`src-tauri/src/lib.rs`): Rust code sets up system tray with submenus for variant selection (6 options) and shadow selection (4 options), plus "Bring to Front" / "Send to Desktop" layer controls and "Quit". Communicates with frontend via `app.emit()` events.
- **Window** (`src-tauri/tauri.conf.json`): 340x340px frameless, transparent, `alwaysOnBottom`, `skipTaskbar`, non-resizable, no shadow — true desktop widget behavior.

## SBB Clock Timing Logic

The clock implements the distinctive SBB timing behavior in `updateClock()`:

- **Second hand:** Completes full 360° rotation in 58.5 seconds (not 60), then pauses at 12 o'clock for 1.5 seconds before the next cycle
- **Minute hand:** Discrete jumps only (no smooth interpolation between minutes) — moves when the second hand resets
- **Hour hand:** Smooth continuous movement based on hours + fractional minutes
- **Backward-spin prevention:** CSS transition is temporarily removed (`style.transition = 'none'`) when the second hand resets from 360° to 0° to avoid a visible reverse animation
- **Resume bounce flick:** When the second hand resumes after the pause, a 0.6s cubic-bezier(0.2, 2.5, 0.4, 0.8) transition gives a characteristic snap/bounce before returning to smooth sweep

## Color Variants

Six variants matching the Mondaine SBB commerce range, selectable via tray menu or `?variant=` URL param:

| Key | Name | Border | Background | Hands | Second |
|---|---|---|---|---|---|
| `classic` | Classic | Silver `#C0C0C0` | White | Black | SBB Red `#EB0000` |
| `white` | White | Light grey `#E8E8E8` | White | Black | SBB Red `#EB0000` |
| `black` | Black | Dark `#222222` | White | Black | SBB Red `#EB0000` |
| `dark` | Dark | Dark `#222222` | Black `#111111` | White | SBB Red `#EB0000` |
| `red` | Red | Red `#EB0000` | White | Black | SBB Red `#EB0000` |
| `gold` | Gold | Gold `#C5A55A` | White | Black | SBB Red `#EB0000` |

Default: `black`. Applied via `applyVariant()` which sets CSS custom properties on `:root`. The SBB Red (`#EB0000`) is the official SBB CI color (Pantone 485 C equivalent).

## Shadow System

Four shadow presets, selectable via tray menu or `?shadow=` URL param:

| Key | Clock shadow | Hand shadow opacity |
|---|---|---|
| `light` | `0 0 12px rgba(0,0,0,0.1)` | 0.1 |
| `medium` | `0 0 24px rgba(0,0,0,0.15)` | 0.2 |
| `heavy` | `0 0 36px rgba(0,0,0,0.25)` | 0.3 |
| `none` | none | 0 |

Default: `medium`. Hand shadows use CSS `filter: drop-shadow()` on `.hand` elements (centered shadow, no offset). Second hand has `filter: none` to avoid rendering artifacts during continuous animation.

## Key Implementation Details

- **Dial marks:** Generated programmatically in a loop (0–59), classified as `.hour` or `.minute` via `i % 5 === 0`. Marks have `top: 8px` gap from edge with `transform-origin: 50% 142px` (150px center minus 8px offset) to keep rotation centered.
- **Hand rotation:** CSS `transform: rotate(Ndeg)` updated each animation frame via `requestAnimationFrame`
- **Tapered hands:** Hour and minute hands use `clip-path: polygon(...)` — hour: `10%/90%` at tip, `100%` at base; minute: `12%/88%` at tip, `100%` at base
- **Hand tails:** Both hour and minute hands extend below center via offset `bottom` and `transform-origin` values
- **Clock frame:** 18px solid border with `border-radius: 50%`, `box-sizing: content-box`
- **Minute snap:** CSS `transition` with bounce easing (`cubic-bezier(0.4, 2.08, 0.55, 0.44)`) for the characteristic snap effect
- **Second hand tip:** `::after` pseudo-element creates the iconic 26px red circle; hand extends past center via `transform-origin: 50% 73%`
- **Second hand bounce:** 0.6s `cubic-bezier(0.2, 2.5, 0.4, 0.8)` transition applied when resuming from pause, cleared after animation completes
- **Logo:** `sbb-cff-ffs.svg` positioned at 33% from top, 100px wide, centered horizontally, with `pointer-events: none`. Auto-inverts via `filter: invert(1)` CSS class when dark variant is active (keyed off `v.hand === '#ffffff'`).
- **SVG logo format:** Cropped viewBox (`0 84 192.756 26`) showing only the SBB CFF FFS logotype strip. Red background `#b74243` with white Swiss cross, black text paths for "SBB CFF FFS".
- **URL persistence (browser):** `history.replaceState()` updates the URL without reload; initial state read from `URLSearchParams`
- **Store persistence (Tauri):** `@tauri-apps/plugin-store` saves variant/shadow to `settings.json`; loaded on app start via `initStore()`
- **Tray-to-frontend communication (Tauri):** Rust emits `set-variant` and `set-shadow` events via `app.emit()`; JS listens via `@tauri-apps/api/event` `listen()` function
- **Dragging (Tauri):** `data-tauri-drag-region` attribute on `.clock-wrapper` enables native OS window dragging without title bar

## CI/CD & Distribution

- **GitHub Actions** (`.github/workflows/release.yml`): Triggered on `v*` tags. Builds in parallel for macOS (ARM64 + x86_64), Ubuntu 22.04, and Windows. Uses `tauri-apps/tauri-action@v0`.
- **Installers:** macOS `.dmg`, Windows `.msi`/`.exe` (NSIS), Linux `.deb`/`.AppImage`
- **Code signing:** macOS via Apple Developer ID + notarization (env vars: `APPLE_CERTIFICATE`, `APPLE_SIGNING_IDENTITY`, `APPLE_ID`, `APPLE_PASSWORD`, `APPLE_TEAM_ID`). Windows via Azure Trusted Signing. Linux via GPG.
- **WebView2 (Windows):** Bootstrapper embedded for Windows 10 compatibility (`webviewInstallMode: "embedBootstrapper"`)
