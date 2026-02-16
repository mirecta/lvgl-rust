//! LVGL Input Device Management
//!
//! Handles touch screens, buttons, encoders, and other input devices.

use crate::{LvglError, Result};
use core::marker::PhantomData;
use lvgl_sys as sys;

/// Input device type
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InputType {
    /// Touchpad or mouse
    Pointer = sys::LV_INDEV_TYPE_POINTER as u8,
    /// Keypad or keyboard
    Keypad = sys::LV_INDEV_TYPE_KEYPAD as u8,
    /// Encoder with rotation and button
    Encoder = sys::LV_INDEV_TYPE_ENCODER as u8,
    /// Physical button mapped to coordinates
    Button = sys::LV_INDEV_TYPE_BUTTON as u8,
}

/// Input state
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum InputState {
    Released = sys::LV_INDEV_STATE_RELEASED as u8,
    Pressed = sys::LV_INDEV_STATE_PRESSED as u8,
}

/// Type alias for the input read callback
pub type ReadCb =
    unsafe extern "C" fn(indev: *mut sys::lv_indev_t, data: *mut sys::lv_indev_data_t);

/// Input device wrapper
pub struct InputDevice {
    raw: *mut sys::lv_indev_t,
    _marker: PhantomData<*mut ()>,
}

impl InputDevice {
    /// Create a new input device
    pub fn create() -> Result<Self> {
        unsafe {
            let raw = sys::lv_indev_create();
            if raw.is_null() {
                Err(LvglError::OutOfMemory)
            } else {
                Ok(Self {
                    raw,
                    _marker: PhantomData,
                })
            }
        }
    }

    /// Set the input device type
    pub fn set_type(&self, indev_type: InputType) {
        unsafe { sys::lv_indev_set_type(self.raw, indev_type as u32) }
    }

    /// Set the read callback
    pub fn set_read_cb(&self, read_cb: ReadCb) {
        unsafe { sys::lv_indev_set_read_cb(self.raw, Some(read_cb)) }
    }

    /// Get raw pointer
    pub fn raw(&self) -> *mut sys::lv_indev_t {
        self.raw
    }
}

/// Touch point data for use in read callbacks
#[derive(Clone, Copy, Debug, Default)]
pub struct TouchPoint {
    pub x: i32,
    pub y: i32,
    pub pressed: bool,
}

impl TouchPoint {
    pub fn new(x: i32, y: i32, pressed: bool) -> Self {
        Self { x, y, pressed }
    }

    /// Write this touch point to LVGL input data
    ///
    /// # Safety
    /// The data pointer must be valid.
    pub unsafe fn write_to(&self, data: *mut sys::lv_indev_data_t) {
        (*data).point.x = self.x;
        (*data).point.y = self.y;
        (*data).state = if self.pressed {
            InputState::Pressed as u32
        } else {
            InputState::Released as u32
        };
    }
}

/// Macro to create a touch input device with a closure
///
/// # Example
/// ```ignore
/// let mut last_touch = TouchPoint::default();
///
/// create_touch_input!(|data| {
///     // Read from your touch controller here
///     let (x, y, pressed) = read_touch();
///     last_touch = TouchPoint::new(x, y, pressed);
///     last_touch.write_to(data);
/// });
/// ```
#[macro_export]
macro_rules! create_touch_input {
    ($read_fn:expr) => {{
        // Store the closure in a static to ensure it lives long enough
        static mut TOUCH_READ_FN: Option<fn(*mut lvgl_sys::lv_indev_data_t)> = None;

        unsafe extern "C" fn touch_read_cb(
            _indev: *mut lvgl_sys::lv_indev_t,
            data: *mut lvgl_sys::lv_indev_data_t,
        ) {
            if let Some(f) = TOUCH_READ_FN {
                f(data);
            }
        }

        unsafe {
            TOUCH_READ_FN = Some($read_fn);
        }

        let indev = $crate::input::InputDevice::create()?;
        indev.set_type($crate::input::InputType::Pointer);
        indev.set_read_cb(touch_read_cb);
        indev
    }};
}
