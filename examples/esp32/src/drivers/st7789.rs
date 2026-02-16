//! ST7789 Display Driver
//!
//! Driver for ST7789 LCD displays commonly found on:
//! - LILYGO T-Display (135x240)
//! - LILYGO T-Display-S3 (170x320)
//! - ESP32-S3-BOX (320x240)
//! - Generic ST7789 modules (240x240, 240x320)

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Output, OutputPin, PinDriver};
use esp_idf_hal::spi::{SpiDeviceDriver, SpiDriver};

/// ST7789 Commands
#[allow(dead_code)]
mod cmd {
    pub const NOP: u8 = 0x00;
    pub const SWRESET: u8 = 0x01;
    pub const RDDID: u8 = 0x04;
    pub const RDDST: u8 = 0x09;
    pub const SLPIN: u8 = 0x10;
    pub const SLPOUT: u8 = 0x11;
    pub const PTLON: u8 = 0x12;
    pub const NORON: u8 = 0x13;
    pub const INVOFF: u8 = 0x20;
    pub const INVON: u8 = 0x21;
    pub const DISPOFF: u8 = 0x28;
    pub const DISPON: u8 = 0x29;
    pub const CASET: u8 = 0x2A;
    pub const RASET: u8 = 0x2B;
    pub const RAMWR: u8 = 0x2C;
    pub const RAMRD: u8 = 0x2E;
    pub const PTLAR: u8 = 0x30;
    pub const VSCRDEF: u8 = 0x33;
    pub const COLMOD: u8 = 0x3A;
    pub const MADCTL: u8 = 0x36;
    pub const VSCSAD: u8 = 0x37;
    pub const FRMCTR1: u8 = 0xB1;
    pub const FRMCTR2: u8 = 0xB2;
    pub const FRMCTR3: u8 = 0xB3;
    pub const INVCTR: u8 = 0xB4;
    pub const DISSET5: u8 = 0xB6;
    pub const PWCTR1: u8 = 0xC0;
    pub const PWCTR2: u8 = 0xC1;
    pub const PWCTR3: u8 = 0xC2;
    pub const PWCTR4: u8 = 0xC3;
    pub const PWCTR5: u8 = 0xC4;
    pub const VMCTR1: u8 = 0xC5;
    pub const RDID1: u8 = 0xDA;
    pub const RDID2: u8 = 0xDB;
    pub const RDID3: u8 = 0xDC;
    pub const RDID4: u8 = 0xDD;
    pub const PVGAMCTRL: u8 = 0xE0;
    pub const NVGAMCTRL: u8 = 0xE1;
}

/// MADCTL (Memory Access Control) flags
#[allow(dead_code)]
mod madctl {
    pub const MY: u8 = 0x80; // Row address order
    pub const MX: u8 = 0x40; // Column address order
    pub const MV: u8 = 0x20; // Row/Column exchange
    pub const ML: u8 = 0x10; // Vertical refresh order
    pub const RGB: u8 = 0x00; // RGB order
    pub const BGR: u8 = 0x08; // BGR order
    pub const MH: u8 = 0x04; // Horizontal refresh order
}

/// Display orientation
#[derive(Clone, Copy, Debug)]
pub enum Orientation {
    /// Portrait (connector at bottom)
    Portrait,
    /// Landscape (connector at right)
    Landscape,
    /// Portrait inverted (connector at top)
    PortraitInverted,
    /// Landscape inverted (connector at left)
    LandscapeInverted,
}

impl Orientation {
    fn madctl_value(&self) -> u8 {
        match self {
            Orientation::Portrait => madctl::MX | madctl::BGR,
            Orientation::Landscape => madctl::MV | madctl::BGR,
            Orientation::PortraitInverted => madctl::MY | madctl::BGR,
            Orientation::LandscapeInverted => madctl::MX | madctl::MY | madctl::MV | madctl::BGR,
        }
    }
}

/// ST7789 configuration
#[derive(Clone, Copy, Debug)]
pub struct St7789Config {
    /// Display width in pixels
    pub width: u16,
    /// Display height in pixels
    pub height: u16,
    /// Column offset (for displays smaller than 240 pixels)
    pub col_offset: u16,
    /// Row offset (for displays smaller than 320 pixels)
    pub row_offset: u16,
    /// Display orientation
    pub orientation: Orientation,
    /// Invert colors (some displays need this)
    pub invert_colors: bool,
}

impl St7789Config {
    /// Common config for 240x240 square display
    pub fn square_240() -> Self {
        Self {
            width: 240,
            height: 240,
            col_offset: 0,
            row_offset: 0,
            orientation: Orientation::Portrait,
            invert_colors: true,
        }
    }

    /// Common config for 240x320 display
    pub fn rect_240x320() -> Self {
        Self {
            width: 240,
            height: 320,
            col_offset: 0,
            row_offset: 0,
            orientation: Orientation::Portrait,
            invert_colors: true,
        }
    }

    /// Config for LILYGO T-Display (135x240)
    pub fn t_display() -> Self {
        Self {
            width: 135,
            height: 240,
            col_offset: 52,
            row_offset: 40,
            orientation: Orientation::Portrait,
            invert_colors: true,
        }
    }

    /// Config for LILYGO T-Display-S3 (170x320)
    pub fn t_display_s3() -> Self {
        Self {
            width: 170,
            height: 320,
            col_offset: 35,
            row_offset: 0,
            orientation: Orientation::Portrait,
            invert_colors: true,
        }
    }

    /// Config for ESP32-S3-BOX (320x240, landscape)
    pub fn esp32_s3_box() -> Self {
        Self {
            width: 320,
            height: 240,
            col_offset: 0,
            row_offset: 0,
            orientation: Orientation::Landscape,
            invert_colors: true,
        }
    }

    /// Get effective width after rotation
    pub fn effective_width(&self) -> u16 {
        match self.orientation {
            Orientation::Portrait | Orientation::PortraitInverted => self.width,
            Orientation::Landscape | Orientation::LandscapeInverted => self.height,
        }
    }

    /// Get effective height after rotation
    pub fn effective_height(&self) -> u16 {
        match self.orientation {
            Orientation::Portrait | Orientation::PortraitInverted => self.height,
            Orientation::Landscape | Orientation::LandscapeInverted => self.width,
        }
    }
}

/// ST7789 Display Driver
pub struct St7789<'a, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    spi: SpiDeviceDriver<'a, &'a SpiDriver<'a>>,
    dc: PinDriver<'a, DC, Output>,
    rst: Option<PinDriver<'a, RST, Output>>,
    config: St7789Config,
}

impl<'a, DC, RST> St7789<'a, DC, RST>
where
    DC: OutputPin,
    RST: OutputPin,
{
    /// Create a new ST7789 driver
    ///
    /// # Arguments
    /// * `spi` - SPI device driver
    /// * `dc` - Data/Command pin
    /// * `rst` - Reset pin (optional, use None if tied to EN)
    /// * `config` - Display configuration
    pub fn new(
        spi: SpiDeviceDriver<'a, &'a SpiDriver<'a>>,
        dc: PinDriver<'a, DC, Output>,
        rst: Option<PinDriver<'a, RST, Output>>,
        config: St7789Config,
    ) -> Self {
        Self {
            spi,
            dc,
            rst,
            config,
        }
    }

    /// Initialize the display
    pub fn init(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        // Hardware reset if we have a reset pin
        if let Some(ref mut rst) = self.rst {
            rst.set_high()?;
            Ets::delay_ms(10);
            rst.set_low()?;
            Ets::delay_ms(10);
            rst.set_high()?;
            Ets::delay_ms(120);
        }

        // Software reset
        self.write_command(cmd::SWRESET)?;
        Ets::delay_ms(150);

        // Sleep out
        self.write_command(cmd::SLPOUT)?;
        Ets::delay_ms(50);

        // Pixel format: 16-bit RGB565
        self.write_command(cmd::COLMOD)?;
        self.write_data(&[0x55])?; // 16-bit color
        Ets::delay_ms(10);

        // Memory access control (orientation)
        self.write_command(cmd::MADCTL)?;
        self.write_data(&[self.config.orientation.madctl_value()])?;

        // Inversion (most ST7789 displays need this)
        if self.config.invert_colors {
            self.write_command(cmd::INVON)?;
        } else {
            self.write_command(cmd::INVOFF)?;
        }
        Ets::delay_ms(10);

        // Normal display mode
        self.write_command(cmd::NORON)?;
        Ets::delay_ms(10);

        // Display on
        self.write_command(cmd::DISPON)?;
        Ets::delay_ms(50);

        // Clear screen to black
        self.clear(0x0000)?;

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
        // Apply offsets based on orientation
        let (col_start, col_end, row_start, row_end) = match self.config.orientation {
            Orientation::Portrait => (
                x0 + self.config.col_offset,
                x1 + self.config.col_offset,
                y0 + self.config.row_offset,
                y1 + self.config.row_offset,
            ),
            Orientation::Landscape => (
                x0 + self.config.row_offset,
                x1 + self.config.row_offset,
                y0 + self.config.col_offset,
                y1 + self.config.col_offset,
            ),
            Orientation::PortraitInverted => (
                x0 + self.config.col_offset,
                x1 + self.config.col_offset,
                y0 + self.config.row_offset,
                y1 + self.config.row_offset,
            ),
            Orientation::LandscapeInverted => (
                x0 + self.config.row_offset,
                x1 + self.config.row_offset,
                y0 + self.config.col_offset,
                y1 + self.config.col_offset,
            ),
        };

        // Column address set
        self.write_command(cmd::CASET)?;
        self.write_data(&[
            (col_start >> 8) as u8,
            (col_start & 0xFF) as u8,
            (col_end >> 8) as u8,
            (col_end & 0xFF) as u8,
        ])?;

        // Row address set
        self.write_command(cmd::RASET)?;
        self.write_data(&[
            (row_start >> 8) as u8,
            (row_start & 0xFF) as u8,
            (row_end >> 8) as u8,
            (row_end & 0xFF) as u8,
        ])?;

        // Memory write
        self.write_command(cmd::RAMWR)?;

        Ok(())
    }

    /// Write pixel data to the display
    pub fn write_pixels(&mut self, data: &[u8]) -> Result<(), esp_idf_hal::sys::EspError> {
        self.dc.set_high()?;

        // Write in chunks to avoid SPI buffer overflow
        const CHUNK_SIZE: usize = 4096;
        for chunk in data.chunks(CHUNK_SIZE) {
            self.spi.write(chunk)?;
        }

        Ok(())
    }

    /// Flush a region to the display (for LVGL integration)
    ///
    /// This is the main function you'll call from the LVGL flush callback.
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

    /// Clear the screen with a color
    pub fn clear(&mut self, color: u16) -> Result<(), esp_idf_hal::sys::EspError> {
        let width = self.config.effective_width();
        let height = self.config.effective_height();

        self.set_window(0, 0, width - 1, height - 1)?;

        let color_bytes = [(color >> 8) as u8, (color & 0xFF) as u8];

        // Write in lines to save memory
        let line_bytes = (width as usize) * 2;
        let mut line_buf = vec![0u8; line_bytes];
        for i in 0..width as usize {
            line_buf[i * 2] = color_bytes[0];
            line_buf[i * 2 + 1] = color_bytes[1];
        }

        self.dc.set_high()?;
        for _ in 0..height {
            self.spi.write(&line_buf)?;
        }

        Ok(())
    }

    /// Fill a rectangle with a color
    pub fn fill_rect(
        &mut self,
        x: u16,
        y: u16,
        width: u16,
        height: u16,
        color: u16,
    ) -> Result<(), esp_idf_hal::sys::EspError> {
        self.set_window(x, y, x + width - 1, y + height - 1)?;

        let color_bytes = [(color >> 8) as u8, (color & 0xFF) as u8];
        let total_pixels = (width as usize) * (height as usize);

        self.dc.set_high()?;

        // Write in chunks
        const CHUNK_PIXELS: usize = 1024;
        let mut buf = [0u8; CHUNK_PIXELS * 2];
        for i in 0..CHUNK_PIXELS {
            buf[i * 2] = color_bytes[0];
            buf[i * 2 + 1] = color_bytes[1];
        }

        let full_chunks = total_pixels / CHUNK_PIXELS;
        let remainder = total_pixels % CHUNK_PIXELS;

        for _ in 0..full_chunks {
            self.spi.write(&buf)?;
        }

        if remainder > 0 {
            self.spi.write(&buf[..remainder * 2])?;
        }

        Ok(())
    }

    /// Set display orientation
    pub fn set_orientation(
        &mut self,
        orientation: Orientation,
    ) -> Result<(), esp_idf_hal::sys::EspError> {
        self.config.orientation = orientation;
        self.write_command(cmd::MADCTL)?;
        self.write_data(&[orientation.madctl_value()])?;
        Ok(())
    }

    /// Turn display on
    pub fn display_on(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        self.write_command(cmd::DISPON)
    }

    /// Turn display off
    pub fn display_off(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        self.write_command(cmd::DISPOFF)
    }

    /// Enter sleep mode
    pub fn sleep(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        self.write_command(cmd::SLPIN)?;
        Ets::delay_ms(5);
        Ok(())
    }

    /// Exit sleep mode
    pub fn wake(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        self.write_command(cmd::SLPOUT)?;
        Ets::delay_ms(120);
        Ok(())
    }

    /// Get display width (accounting for orientation)
    pub fn width(&self) -> u16 {
        self.config.effective_width()
    }

    /// Get display height (accounting for orientation)
    pub fn height(&self) -> u16 {
        self.config.effective_height()
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

/// RGB565 color helper
pub mod color {
    /// Convert RGB888 to RGB565
    pub const fn rgb565(r: u8, g: u8, b: u8) -> u16 {
        ((r as u16 & 0xF8) << 8) | ((g as u16 & 0xFC) << 3) | (b as u16 >> 3)
    }

    pub const BLACK: u16 = 0x0000;
    pub const WHITE: u16 = 0xFFFF;
    pub const RED: u16 = rgb565(255, 0, 0);
    pub const GREEN: u16 = rgb565(0, 255, 0);
    pub const BLUE: u16 = rgb565(0, 0, 255);
    pub const YELLOW: u16 = rgb565(255, 255, 0);
    pub const CYAN: u16 = rgb565(0, 255, 255);
    pub const MAGENTA: u16 = rgb565(255, 0, 255);
    pub const ORANGE: u16 = rgb565(255, 165, 0);
    pub const GRAY: u16 = rgb565(128, 128, 128);
}
