# LVGL ESP32 Example

LVGL running on real hardware — configured for the LILYGO T-Display-S3 (170x320 ST7789 TFT) with capacitive touch input. Easily adaptable to other ESP32 boards and displays.

## Prerequisites

1. Install the Rust ESP32 toolchain:

```bash
cargo install espup espflash
espup install
```

2. Source the environment (add to your shell profile):

```bash
. $HOME/export-esp.sh
```

## Building and Flashing

```bash
cd examples/esp32
cargo run --target xtensa-esp32s3-espidf --release
```

This builds, flashes, and opens a serial monitor. Use `cargo build` if you only want to compile.

For other ESP32 variants, change the target:
- ESP32: `xtensa-esp32-espidf`
- ESP32-S2: `xtensa-esp32s2-espidf`
- ESP32-C3: `riscv32imc-esp-espidf`

## Pin Configuration (T-Display-S3)

| Function | GPIO |
|----------|------|
| MOSI     | 11   |
| SCLK     | 12   |
| CS       | 10   |
| DC       | 13   |
| RST      | 9    |
| Backlight| 14   |

Edit the pin assignments in `src/main.rs` to match your board.

## What It Demonstrates

A scrollable vertical layout optimized for the tall narrow screen:

- Title label with cyan accent color
- Button with LED toggle indicator
- Slider with live value display
- Switch and checkbox
- Dropdown menu (baud rate selection)
- Progress bars (CPU, RAM)
- Arc gauge with percentage label
- Spinner animation

Uses a dark theme (0x1a1a2e background) with cyan and green accents.

## Project Structure

```
esp32/
├── Cargo.toml
├── build.rs                  # ESP-IDF build integration
├── sdkconfig.defaults        # ESP-IDF settings (stack size, SPI, flash)
├── partitions.csv
├── .cargo/config.toml        # Target, linker, and cross-compiler config
├── README.md
└── src/
    ├── main.rs               # Demo UI and LVGL event loop
    └── drivers/
        ├── mod.rs
        ├── st7789.rs         # ST7789 SPI display driver
        ├── ili9341.rs        # ILI9341 display driver (alternative)
        └── cst816.rs         # CST816 capacitive touch driver
```

## Adapting to Your Board

1. **Different display**: Swap `St7789` for the `Ili9341` driver (included) or write your own implementing the same `flush()` pattern
2. **Different pins**: Update the GPIO numbers in `main()`
3. **Different resolution**: Change `DISPLAY_WIDTH` and `DISPLAY_HEIGHT` constants
4. **PSRAM**: Uncomment the SPIRAM lines in `sdkconfig.defaults` if your board has PSRAM
5. **Touch controller**: The `cst816` driver is included for boards with a CST816 touch panel
