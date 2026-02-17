# LVGL Simulator Example

Desktop simulator for developing and testing LVGL UIs without embedded hardware. Uses SDL2 for window rendering and mouse input.

## Prerequisites

Install SDL2 development libraries:

```bash
# Ubuntu/Debian
sudo apt install libsdl2-dev

# Fedora
sudo dnf install SDL2-devel

# macOS
brew install sdl2
```

## Running

```bash
cd examples/simulator
cargo run
```

This opens a 320x240 window (scaled 2x) with mouse input. Close the window or press Ctrl+C to exit.

## What It Demonstrates

The example creates a tabbed UI with three pages showcasing different widget categories:

**Tab 1 — Controls**
- Button with LED toggle indicator
- Slider with live value label (0-100)
- Switch and checkbox
- Arc gauge with percentage display
- Spinner (loading animation)

**Tab 2 — Data**
- Line chart with two data series
- Progress bars (CPU, RAM, Disk)

**Tab 3 — Inputs**
- Dropdown menu
- Roller (scrollable picker)
- Textarea with placeholder text

## Project Structure

```
simulator/
├── Cargo.toml
├── README.md
└── src/
    ├── main.rs                # Demo UI and LVGL event loop
    └── simulator_display.rs   # SDL2 display driver (RGB565 framebuffer)
```

## Key Patterns

- **Display driver**: `SimulatorDisplay` wraps SDL2, exposes a `flush()` method called from the LVGL flush callback
- **Input handling**: Mouse position and button state are polled from SDL2 and fed to LVGL via the input read callback
- **Flexbox layout**: Rows and columns use `lv_obj_set_flex_flow` for responsive positioning
- **Event callbacks**: Closures capture widget pointers and update labels on `ValueChanged` / `Clicked` events
- **Style lifetime**: Styles are `Box::leak`ed to satisfy LVGL's requirement for `'static` style references
