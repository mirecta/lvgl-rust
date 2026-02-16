//! CST816 Capacitive Touch Controller Driver
//!
//! Common touch controller found on:
//! - LILYGO T-Display-S3
//! - Many ESP32 display modules
//!
//! Uses I2C interface.

use esp_idf_hal::delay::Ets;
use esp_idf_hal::gpio::{Input, InputPin, Output, OutputPin, PinDriver};
use esp_idf_hal::i2c::I2cDriver;

/// CST816 I2C address
const CST816_ADDR: u8 = 0x15;

/// CST816 Registers
#[allow(dead_code)]
mod reg {
    pub const GESTURE_ID: u8 = 0x01;
    pub const FINGER_NUM: u8 = 0x02;
    pub const XPOS_H: u8 = 0x03;
    pub const XPOS_L: u8 = 0x04;
    pub const YPOS_H: u8 = 0x05;
    pub const YPOS_L: u8 = 0x06;
    pub const CHIP_ID: u8 = 0xA7;
    pub const PROJ_ID: u8 = 0xA8;
    pub const FW_VERSION: u8 = 0xA9;
    pub const MOTION_MASK: u8 = 0xEC;
    pub const IRQ_PULSE_WIDTH: u8 = 0xED;
    pub const NOR_SCAN_PER: u8 = 0xEE;
    pub const MOTION_S1_ANGLE: u8 = 0xEF;
    pub const LP_SCAN_RAW1_H: u8 = 0xF0;
    pub const LP_SCAN_RAW1_L: u8 = 0xF1;
    pub const LP_SCAN_RAW2_H: u8 = 0xF2;
    pub const LP_SCAN_RAW2_L: u8 = 0xF3;
    pub const LP_AUTO_WAKEUP_TIME: u8 = 0xF4;
    pub const LP_SCAN_TH: u8 = 0xF5;
    pub const LP_SCAN_WIN: u8 = 0xF6;
    pub const LP_SCAN_FREQ: u8 = 0xF7;
    pub const LP_SCAN_I_DAC: u8 = 0xF8;
    pub const AUTO_SLEEP_TIME: u8 = 0xF9;
    pub const IRQ_CTL: u8 = 0xFA;
    pub const AUTO_RESET: u8 = 0xFB;
    pub const LONG_PRESS_TIME: u8 = 0xFC;
    pub const IO_CTL: u8 = 0xFD;
    pub const DIS_AUTO_SLEEP: u8 = 0xFE;
}

/// Gesture types
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub enum Gesture {
    None = 0x00,
    SwipeUp = 0x01,
    SwipeDown = 0x02,
    SwipeLeft = 0x03,
    SwipeRight = 0x04,
    SingleClick = 0x05,
    DoubleClick = 0x0B,
    LongPress = 0x0C,
}

impl From<u8> for Gesture {
    fn from(value: u8) -> Self {
        match value {
            0x01 => Gesture::SwipeUp,
            0x02 => Gesture::SwipeDown,
            0x03 => Gesture::SwipeLeft,
            0x04 => Gesture::SwipeRight,
            0x05 => Gesture::SingleClick,
            0x0B => Gesture::DoubleClick,
            0x0C => Gesture::LongPress,
            _ => Gesture::None,
        }
    }
}

/// Touch event data
#[derive(Clone, Copy, Debug, Default)]
pub struct TouchData {
    /// X coordinate (0 = left)
    pub x: u16,
    /// Y coordinate (0 = top)
    pub y: u16,
    /// True if finger is touching
    pub pressed: bool,
    /// Detected gesture
    pub gesture: Option<Gesture>,
}

/// CST816 Touch Controller Driver
pub struct Cst816<'a, RST, INT>
where
    RST: OutputPin,
    INT: InputPin,
{
    i2c: I2cDriver<'a>,
    rst: Option<PinDriver<'a, RST, Output>>,
    int: Option<PinDriver<'a, INT, Input>>,
    /// Display width (for coordinate mapping)
    width: u16,
    /// Display height (for coordinate mapping)
    height: u16,
    /// Swap X/Y coordinates
    swap_xy: bool,
    /// Invert X coordinate
    invert_x: bool,
    /// Invert Y coordinate
    invert_y: bool,
}

impl<'a, RST, INT> Cst816<'a, RST, INT>
where
    RST: OutputPin,
    INT: InputPin,
{
    /// Create a new CST816 driver
    ///
    /// # Arguments
    /// * `i2c` - I2C driver
    /// * `rst` - Reset pin (optional)
    /// * `int` - Interrupt pin (optional, for detecting touch events)
    /// * `width` - Display width
    /// * `height` - Display height
    pub fn new(
        i2c: I2cDriver<'a>,
        rst: Option<PinDriver<'a, RST, Output>>,
        int: Option<PinDriver<'a, INT, Input>>,
        width: u16,
        height: u16,
    ) -> Self {
        Self {
            i2c,
            rst,
            int,
            width,
            height,
            swap_xy: false,
            invert_x: false,
            invert_y: false,
        }
    }

    /// Configure coordinate transformation
    pub fn set_transform(&mut self, swap_xy: bool, invert_x: bool, invert_y: bool) {
        self.swap_xy = swap_xy;
        self.invert_x = invert_x;
        self.invert_y = invert_y;
    }

    /// Initialize the touch controller
    pub fn init(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        // Hardware reset if we have a reset pin
        if let Some(ref mut rst) = self.rst {
            rst.set_low()?;
            Ets::delay_ms(10);
            rst.set_high()?;
            Ets::delay_ms(50);
        }

        // Read chip ID to verify communication
        let chip_id = self.read_reg(reg::CHIP_ID)?;
        log::info!("CST816 Chip ID: 0x{:02X}", chip_id);

        // Disable auto sleep (keep touch active)
        self.write_reg(reg::DIS_AUTO_SLEEP, 0x01)?;

        // Configure IRQ mode (trigger on touch)
        self.write_reg(reg::IRQ_CTL, 0x41)?;

        Ok(())
    }

    /// Read touch data
    pub fn read(&mut self) -> Result<TouchData, esp_idf_hal::sys::EspError> {
        let mut data = [0u8; 6];

        // Read gesture and touch data in one transaction
        self.i2c
            .write_read(CST816_ADDR, &[reg::GESTURE_ID], &mut data, 100)?;

        let gesture_id = data[0];
        let finger_num = data[1];
        let x_raw = ((data[2] as u16 & 0x0F) << 8) | data[3] as u16;
        let y_raw = ((data[4] as u16 & 0x0F) << 8) | data[5] as u16;

        let pressed = finger_num > 0;

        // Apply coordinate transformations
        let (mut x, mut y) = if self.swap_xy {
            (y_raw, x_raw)
        } else {
            (x_raw, y_raw)
        };

        if self.invert_x {
            x = self.width.saturating_sub(x);
        }

        if self.invert_y {
            y = self.height.saturating_sub(y);
        }

        // Clamp to display bounds
        x = x.min(self.width.saturating_sub(1));
        y = y.min(self.height.saturating_sub(1));

        let gesture = if gesture_id != 0 {
            Some(Gesture::from(gesture_id))
        } else {
            None
        };

        Ok(TouchData {
            x,
            y,
            pressed,
            gesture,
        })
    }

    /// Check if touch interrupt is active (if INT pin is connected)
    pub fn is_touched(&self) -> bool {
        if let Some(ref int) = self.int {
            int.is_low()
        } else {
            // If no INT pin, we can't tell without reading
            true
        }
    }

    /// Put controller into sleep mode
    pub fn sleep(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        self.write_reg(reg::DIS_AUTO_SLEEP, 0x00)?;
        Ok(())
    }

    /// Wake controller from sleep
    pub fn wake(&mut self) -> Result<(), esp_idf_hal::sys::EspError> {
        // Toggle reset to wake
        if let Some(ref mut rst) = self.rst {
            rst.set_low()?;
            Ets::delay_ms(10);
            rst.set_high()?;
            Ets::delay_ms(50);
        }
        self.write_reg(reg::DIS_AUTO_SLEEP, 0x01)?;
        Ok(())
    }

    /// Read a register
    fn read_reg(&mut self, reg: u8) -> Result<u8, esp_idf_hal::sys::EspError> {
        let mut buf = [0u8; 1];
        self.i2c.write_read(CST816_ADDR, &[reg], &mut buf, 100)?;
        Ok(buf[0])
    }

    /// Write a register
    fn write_reg(&mut self, reg: u8, value: u8) -> Result<(), esp_idf_hal::sys::EspError> {
        self.i2c.write(CST816_ADDR, &[reg, value], 100)?;
        Ok(())
    }
}

/// Helper to convert TouchData to LVGL TouchPoint
impl From<TouchData> for lvgl::input::TouchPoint {
    fn from(data: TouchData) -> Self {
        Self {
            x: data.x as i32,
            y: data.y as i32,
            pressed: data.pressed,
        }
    }
}
