# LVGL Rust

Safe LVGL (Light and Versatile Graphics Library) wrapper for Rust. Works on ESP32 and desktop (simulator).

LVGL C sources (v9.2.2) are **downloaded automatically** during build -- no manual setup needed.

## Using as a Library

Add to your project's `Cargo.toml`:

```toml
[dependencies]
# For desktop simulator:
lvgl = { git = "https://github.com/user/lvgl-rust.git", features = ["simulator"] }
sdl2 = "0.36"

# For ESP32 (or other embedded):
# lvgl = { git = "https://github.com/user/lvgl-rust.git", features = ["std"] }
```

Raw FFI bindings are re-exported as `lvgl::sys`, so you only need the single `lvgl` dependency.

LVGL C sources are fetched automatically during the first build.

To use a local checkout instead of auto-download, set `LVGL_PATH`:
```toml
# .cargo/config.toml
[env]
LVGL_PATH = { relative = true, value = "lvgl" }
```

## Project Structure

```
lvgl-rust/
├── Cargo.toml              # Library crate + workspace root
├── src/
│   ├── lib.rs              # Library root
│   ├── display.rs          # Display management
│   ├── input.rs            # Input device management
│   ├── obj.rs              # Base object wrapper
│   ├── style.rs            # Style management
│   └── widgets.rs          # Widget wrappers
├── lvgl-sys/               # Raw FFI bindings subcrate
└── examples/
    ├── simulator/          # Desktop simulator example
    │   ├── Cargo.toml
    │   └── src/
    │       ├── main.rs
    │       └── simulator_display.rs
    └── esp32/              # ESP32 example project
        ├── Cargo.toml
        ├── build.rs
        ├── sdkconfig.defaults
        ├── .cargo/config.toml
        └── src/
            ├── main.rs
            └── drivers/
```

## Quick Start

### Desktop Simulator

```bash
# Install SDL2
sudo apt install libsdl2-dev        # Ubuntu/Debian
# brew install sdl2                 # macOS

# Run simulator example (LVGL is downloaded automatically)
cd examples/simulator
cargo run
```

### ESP32 Embedded

```bash
# Install ESP Rust toolchain
cargo install espup
espup install
source ~/export-esp.sh

# Install tools
cargo install ldproxy espflash

# Build ESP32 example
cd examples/esp32
cargo +esp build --release --target xtensa-esp32s3-espidf -Z build-std=std,panic_abort

# Flash
espflash flash --monitor target/xtensa-esp32s3-espidf/release/lvgl-esp32-examples
```

## API Overview

```rust
use lvgl::{self, Align, Color, Event, LvglObj};
use lvgl::display::{Display, RenderMode};
use lvgl::input::{InputDevice, InputType};
use lvgl::widgets::{Label, Button, Slider};

// Initialize LVGL
lvgl::init()?;

// Create display and set up buffers
let display = Display::create(320, 240)?;

// Create widgets
let screen = lvgl::screen_active().unwrap();
let label = Label::create(&screen)?;
label.set_text(c"Hello LVGL!");
label.align(Align::Center, 0, 0);

let btn = Button::create(&screen)?;
btn.add_event_cb(Event::Clicked, || {
    println!("Button clicked!");
});

// Main loop
loop {
    lvgl::task_handler();
}
```

## Widget Status

| Widget | Status | Notes |
|--------|--------|-------|
| Label | done | Text display, long mode, static text |
| Button | done | Click events, create with label helper |
| Slider | done | Value, range, animation |
| Switch | done | On/off toggle |
| Checkbox | done | Check/uncheck with text |
| Bar | done | Progress bar with range |
| Arc | done | Gauge with angles, modes |
| Spinner | done | Loading animation |
| Dropdown | done | Options, selection, open/close |
| Textarea | done | Text input, cursor, password mode |
| Roller | done | Scrollable option picker |
| LED | done | On/off/toggle, brightness, color |
| Line | done | Point arrays, Y invert |
| Image | done | Source, rotation, scale, pivot |
| Spinbox | done | Numeric input with inc/dec |
| Scale | done | Gauge with ticks and labels |
| Buttonmatrix | done | Grid of buttons from map |
| Table | done | Rows, columns, cell values |
| Chart | done | Series, types, ranges |
| List | done | Text items and buttons |
| Msgbox | done | Title, text, footer buttons |
| Tabview | done | Tabbed container |
| Tileview | done | Swipeable tile grid |
| Calendar | done | Date picker, highlights |
| Keyboard | done | On-screen keyboard |
| Menu | done | Pages, sections, sidebar |
| Canvas | done | Pixel drawing, fill |
| Win | done | Window with header/content |
| Animimg | -- | Planned later |
| Imagebutton | -- | Planned later |
| Spangroup | -- | Planned later |

All LVGL widgets are accessible via raw FFI bindings at `lvgl::sys` even if not yet wrapped.

## Features

| Feature | Description |
|---------|-------------|
| `std` | Enable std support |
| `simulator` | Desktop simulator (implies `std`, selects simulator `lv_conf.h`) |

The library itself has zero platform dependencies. Display drivers (SDL2 simulator, ESP-IDF hardware drivers) live in the example projects under `examples/`.

## Troubleshooting

**Build fails with bindgen errors:**
- Ensure clang is installed: `sudo apt install clang`
- Install 32-bit headers: `sudo apt install gcc-multilib`

**Simulator build fails with pointer size assertion (4 vs 8):**
- If you previously sourced `export-esp.sh`, unset the ESP clang:
  ```bash
  unset LIBCLANG_PATH
  cd examples/simulator && cargo run
  ```
- Only source `export-esp.sh` when building for ESP32 targets, not for the simulator.

**Display shows garbage/wrong colors:**
- Check `LV_COLOR_16_SWAP` in `lv_conf.h`
- Verify SPI clock speed

**Out of memory on ESP32:**
- Reduce `LV_MEM_SIZE` in `lv_conf.h`
- Reduce buffer lines in your application
