//! ILI9341 Display Driver Example
//!
//! This shows how to implement a display driver for LVGL using the ILI9341
//! with esp-idf-hal. Adapt this for your specific display (ST7789, etc.)

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{OutputPin, PinDriver};
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};

/// ILI9341 Commands
mod cmd {
    pub const SWRESET: u8 = 0x01;
    pub const SLPOUT: u8 = 0x11;
    pub const DISPON: u8 = 0x29;
    pub const CASET: u8 = 0x2A;
    pub const PASET: u8 = 0x2B;
    pub const RAMWR: u8 = 0x2C;
    pub const MADCTL: u8 = 0x36;
    pub const COLMOD: u8 = 0x3A;
}

/// ILI9341 Driver
pub struct Ili9341<'a, DC, RST, BL>
where
    DC: OutputPin,
    RST: OutputPin,
    BL: OutputPin,
{
    spi: SpiDeviceDriver<'a, &'a SpiDriver<'a>>,
    dc: PinDriver<'a, DC, esp_idf_hal::gpio::Output>,
    rst: PinDriver<'a, RST, esp_idf_hal::gpio::Output>,
    bl: PinDriver<'a, BL, esp_idf_hal::gpio::Output>,
    width: u16,
    height: u16,
}

impl<'a, DC, RST, BL> Ili9341<'a, DC, RST, BL>
where
    DC: OutputPin,
    RST: OutputPin,
    BL: OutputPin,
{
    /// Create a new ILI9341 driver
    ///
    /// # Arguments
    /// * `spi` - SPI device driver
    /// * `dc` - Data/Command pin
    /// * `rst` - Reset pin
    /// * `bl` - Backlight pin
    /// * `width` - Display width
    /// * `height` - Display height
    pub fn new(
        spi: SpiDeviceDriver<'a, &'a SpiDriver<'a>>,
        dc: PinDriver<'a, DC, esp_idf_hal::gpio::Output>,
        rst: PinDriver<'a, RST, esp_idf_hal::gpio::Output>,
        bl: PinDriver<'a, BL, esp_idf_hal::gpio::Output>,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            spi,
            dc,
            rst,
            bl,
            width,
            height,
        }
    }

    /// Initialize the display
    pub fn init(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        // Hardware reset
        self.rst.set_low()?;
        Ets::delay_ms(10);
        self.rst.set_high()?;
        Ets::delay_ms(120);

        // Software reset
        self.write_command(cmd::SWRESET)?;
        Ets::delay_ms(120);

        // Sleep out
        self.write_command(cmd::SLPOUT)?;
        Ets::delay_ms(120);

        // Pixel format: 16-bit RGB565
        self.write_command(cmd::COLMOD)?;
        self.write_data(&[0x55])?;

        // Memory access control (rotation)
        self.write_command(cmd::MADCTL)?;
        self.write_data(&[0x48])?; // RGB order, landscape

        // Display on
        self.write_command(cmd::DISPON)?;
        Ets::delay_ms(50);

        // Backlight on
        self.bl.set_high()?;

        Ok(())
    }

    /// Set the drawing window
    pub fn set_window(
        &mut self,
        x0: u16,
        y0: u16,
        x1: u16,
        y1: u16,
    ) -> Result<(), esp_idf_hal::sys::EspError> {
        // Column address set
        self.write_command(cmd::CASET)?;
        self.write_data(&[
            (x0 >> 8) as u8,
            (x0 & 0xFF) as u8,
            (x1 >> 8) as u8,
            (x1 & 0xFF) as u8,
        ])?;

        // Page address set
        self.write_command(cmd::PASET)?;
        self.write_data(&[
            (y0 >> 8) as u8,
            (y0 & 0xFF) as u8,
            (y1 >> 8) as u8,
            (y1 & 0xFF) as u8,
        ])?;

        // Memory write
        self.write_command(cmd::RAMWR)?;

        Ok(())
    }

    /// Write pixel data to the display
    pub fn write_pixels(&mut self, data: &[u8]) -> Result<(), esp_idf_hal::sys::EspError> {
        self.dc.set_high()?;
        self.spi.write(data)?;
        Ok(())
    }

    /// Flush a region to the display (for LVGL)
    pub fn flush(
        &mut self,
        x1: i32,
        y1: i32,
        x2: i32,
        y2: i32,
        data: &[u8],
    ) -> Result<(), esp_idf_hal::sys::EspError> {
        self.set_window(x1 as u16, y1 as u16, x2 as u16, y2 as u16)?;
        self.write_pixels(data)?;
        Ok(())
    }

    /// Write a command byte
    fn write_command(&mut self, cmd: u8) -> Result<(), esp_idf_hal::sys::EspError> {
        self.dc.set_low()?;
        self.spi.write(&[cmd])?;
        Ok(())
    }

    /// Write data bytes
    fn write_data(&mut self, data: &[u8]) -> Result<(), esp_idf_hal::sys::EspError> {
        self.dc.set_high()?;
        self.spi.write(data)?;
        Ok(())
    }
}
