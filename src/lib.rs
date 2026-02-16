//! Safe LVGL wrapper for Rust
//!
//! Platform-independent safe Rust abstractions over the raw LVGL FFI bindings.
//! Works on ESP32, desktop (simulator), and any platform with a C compiler.
//!
//! The design philosophy is "minimal but safe" - we don't wrap everything,
//! just the commonly used parts.

#![cfg_attr(not(feature = "std"), no_std)]

extern crate alloc;

pub mod display;
pub mod input;
mod obj;
pub mod style;
pub mod widgets;

pub use display::Display;
pub use obj::{LvglObj, Obj};
pub use style::Style;
pub use widgets::*;

/// Re-export raw FFI bindings so users don't need a separate `lvgl-sys` dependency.
pub use lvgl_sys as sys;

/// Global LVGL state. LVGL is not thread-safe, so we use a RefCell
/// to enforce single-threaded access at runtime.
static mut LVGL_INITIALIZED: bool = false;

/// Error type for LVGL operations
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum LvglError {
    NotInitialized,
    AlreadyInitialized,
    NullPointer,
    InvalidParameter,
    OutOfMemory,
    DisplayError,
}

impl core::fmt::Display for LvglError {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        match self {
            Self::NotInitialized => write!(f, "LVGL not initialized"),
            Self::AlreadyInitialized => write!(f, "LVGL already initialized"),
            Self::NullPointer => write!(f, "null pointer"),
            Self::InvalidParameter => write!(f, "invalid parameter"),
            Self::OutOfMemory => write!(f, "out of memory"),
            Self::DisplayError => write!(f, "display error"),
        }
    }
}

#[cfg(feature = "std")]
impl std::error::Error for LvglError {}

pub type Result<T> = core::result::Result<T, LvglError>;

/// Initialize LVGL. Must be called before any other LVGL function.
///
/// # Safety
/// This function is safe to call, but LVGL itself is not thread-safe.
/// Ensure all LVGL operations happen on the same thread.
pub fn init() -> Result<()> {
    unsafe {
        if LVGL_INITIALIZED {
            return Err(LvglError::AlreadyInitialized);
        }
        sys::lv_init();
        LVGL_INITIALIZED = true;
    }
    Ok(())
}

/// Check if LVGL is initialized
pub fn is_initialized() -> bool {
    unsafe { LVGL_INITIALIZED }
}

/// Run LVGL task handler. Call this periodically (e.g., every 5-10ms).
///
/// Returns the time in milliseconds until it wants to be called again.
pub fn task_handler() -> u32 {
    unsafe { sys::lv_timer_handler() }
}

/// Tick LVGL's internal clock. Call this from a timer interrupt or task.
///
/// With ESP-IDF, this is handled automatically via `lv_conf.h` using
/// `esp_timer_get_time()`, so you typically don't need to call this.
/// For the simulator, call this manually with elapsed milliseconds.
pub fn tick_inc(period_ms: u32) {
    unsafe { sys::lv_tick_inc(period_ms) }
}

/// Get the currently active screen
pub fn screen_active() -> Option<Obj> {
    unsafe {
        let screen = sys::lv_screen_active();
        if screen.is_null() {
            None
        } else {
            Some(Obj::from_raw(screen))
        }
    }
}

/// Load a screen (make it active)
pub fn screen_load(screen: &Obj) {
    unsafe {
        sys::lv_screen_load(screen.raw());
    }
}

/// Create a new screen
pub fn screen_create() -> Result<Obj> {
    unsafe {
        let screen = sys::lv_obj_create(core::ptr::null_mut());
        if screen.is_null() {
            Err(LvglError::OutOfMemory)
        } else {
            Ok(Obj::from_raw(screen))
        }
    }
}

/// LVGL color (RGB565 or RGB888 depending on config)
#[derive(Clone, Copy, Debug, Default)]
#[repr(transparent)]
pub struct Color(sys::lv_color_t);

impl Color {
    /// Create color from RGB values (0-255 each)
    pub fn rgb(r: u8, g: u8, b: u8) -> Self {
        unsafe { Self(sys::lv_color_make(r, g, b)) }
    }

    /// Create color from hex value (0xRRGGBB)
    pub fn hex(hex: u32) -> Self {
        unsafe { Self(sys::lv_color_hex(hex)) }
    }

    /// Create color from hex3 value (0xRGB)
    pub fn hex3(hex: u16) -> Self {
        unsafe { Self(sys::lv_color_hex3(hex as u32)) }
    }

    /// White
    pub fn white() -> Self {
        Self::hex(0xFFFFFF)
    }

    /// Black
    pub fn black() -> Self {
        Self::hex(0x000000)
    }

    /// Get raw LVGL color
    pub fn raw(&self) -> sys::lv_color_t {
        self.0
    }
}

/// Alignment options for positioning objects
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum Align {
    Default = sys::LV_ALIGN_DEFAULT as u8,
    TopLeft = sys::LV_ALIGN_TOP_LEFT as u8,
    TopMid = sys::LV_ALIGN_TOP_MID as u8,
    TopRight = sys::LV_ALIGN_TOP_RIGHT as u8,
    BottomLeft = sys::LV_ALIGN_BOTTOM_LEFT as u8,
    BottomMid = sys::LV_ALIGN_BOTTOM_MID as u8,
    BottomRight = sys::LV_ALIGN_BOTTOM_RIGHT as u8,
    LeftMid = sys::LV_ALIGN_LEFT_MID as u8,
    RightMid = sys::LV_ALIGN_RIGHT_MID as u8,
    Center = sys::LV_ALIGN_CENTER as u8,
}

/// Object state flags
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct State(pub u16);

impl State {
    pub const DEFAULT: Self = Self(sys::LV_STATE_DEFAULT as u16);
    pub const CHECKED: Self = Self(sys::LV_STATE_CHECKED as u16);
    pub const FOCUSED: Self = Self(sys::LV_STATE_FOCUSED as u16);
    pub const PRESSED: Self = Self(sys::LV_STATE_PRESSED as u16);
    pub const DISABLED: Self = Self(sys::LV_STATE_DISABLED as u16);
}

/// Object part (for styling)
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub struct Part(pub u32);

impl Part {
    pub const MAIN: Self = Self(sys::LV_PART_MAIN);
    pub const SCROLLBAR: Self = Self(sys::LV_PART_SCROLLBAR);
    pub const INDICATOR: Self = Self(sys::LV_PART_INDICATOR);
    pub const KNOB: Self = Self(sys::LV_PART_KNOB);
    pub const SELECTED: Self = Self(sys::LV_PART_SELECTED);
    pub const ITEMS: Self = Self(sys::LV_PART_ITEMS);
    pub const CURSOR: Self = Self(sys::LV_PART_CURSOR);
}

/// Event codes
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u32)]
pub enum Event {
    Pressed = sys::LV_EVENT_PRESSED,
    Pressing = sys::LV_EVENT_PRESSING,
    Released = sys::LV_EVENT_RELEASED,
    Clicked = sys::LV_EVENT_CLICKED,
    LongPressed = sys::LV_EVENT_LONG_PRESSED,
    ValueChanged = sys::LV_EVENT_VALUE_CHANGED,
    Focused = sys::LV_EVENT_FOCUSED,
    Defocused = sys::LV_EVENT_DEFOCUSED,
}
