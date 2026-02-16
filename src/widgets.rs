//! LVGL Widget Wrappers
//!
//! Safe wrappers for commonly used LVGL widgets.

use crate::obj::{LvglObj, Obj};
use crate::{Color, LvglError, Result};
use core::ffi::CStr;
use core::marker::PhantomData;
use lvgl_sys as sys;

// ============================================================================
// Label
// ============================================================================

/// Label widget for displaying text
pub struct Label {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Label {
    /// Create a new label on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_label_create(parent.raw());
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

    /// Set the label text
    pub fn set_text(&self, text: &CStr) {
        unsafe { sys::lv_label_set_text(self.raw, text.as_ptr()) }
    }

    /// Set text using a static string (more efficient, no copy)
    pub fn set_text_static(&self, text: &'static CStr) {
        unsafe { sys::lv_label_set_text_static(self.raw, text.as_ptr()) }
    }

    /// Set text with format (like printf)
    ///
    /// # Safety
    /// Format string must match arguments
    pub unsafe fn set_text_fmt(&self, fmt: &CStr) {
        sys::lv_label_set_text_fmt(self.raw, fmt.as_ptr())
    }

    /// Enable/disable long text mode (scrolling, wrapping, etc.)
    pub fn set_long_mode(&self, mode: LabelLongMode) {
        unsafe { sys::lv_label_set_long_mode(self.raw, mode as u32) }
    }

    /// Set text color
    pub fn set_text_color(&self, color: Color) {
        self.set_style_text_color(color, 0);
    }
}

impl LvglObj for Label {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Label long text mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum LabelLongMode {
    Wrap = sys::LV_LABEL_LONG_WRAP as u8,
    Dot = sys::LV_LABEL_LONG_DOT as u8,
    Scroll = sys::LV_LABEL_LONG_SCROLL as u8,
    ScrollCircular = sys::LV_LABEL_LONG_SCROLL_CIRCULAR as u8,
    Clip = sys::LV_LABEL_LONG_CLIP as u8,
}

// ============================================================================
// Button
// ============================================================================

/// Button widget
pub struct Button {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Button {
    /// Create a new button on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_button_create(parent.raw());
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

    /// Create a button with a label
    pub fn create_with_label(parent: &impl LvglObj, text: &CStr) -> Result<Self> {
        let btn = Self::create(parent)?;
        let label = Label::create(&btn)?;
        label.set_text(text);
        label.center();
        Ok(btn)
    }
}

impl LvglObj for Button {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Slider
// ============================================================================

/// Slider widget for selecting a value from a range
pub struct Slider {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Slider {
    /// Create a new slider on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_slider_create(parent.raw());
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

    /// Set the slider value
    pub fn set_value(&self, value: i32, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_slider_set_value(self.raw, value, anim_flag) }
    }

    /// Get the slider value
    pub fn get_value(&self) -> i32 {
        unsafe { sys::lv_slider_get_value(self.raw) }
    }

    /// Set the range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { sys::lv_slider_set_range(self.raw, min, max) }
    }
}

impl LvglObj for Slider {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Switch
// ============================================================================

/// On/Off switch widget
pub struct Switch {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Switch {
    /// Create a new switch on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_switch_create(parent.raw());
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

    /// Check if the switch is on
    pub fn is_checked(&self) -> bool {
        self.has_state(crate::State::CHECKED)
    }

    /// Set the switch state
    pub fn set_checked(&self, checked: bool) {
        if checked {
            self.add_state(crate::State::CHECKED);
        } else {
            self.remove_state(crate::State::CHECKED);
        }
    }
}

impl LvglObj for Switch {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Checkbox
// ============================================================================

/// Checkbox widget
pub struct Checkbox {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Checkbox {
    /// Create a new checkbox on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_checkbox_create(parent.raw());
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

    /// Set the checkbox text
    pub fn set_text(&self, text: &CStr) {
        unsafe { sys::lv_checkbox_set_text(self.raw, text.as_ptr()) }
    }

    /// Check if checked
    pub fn is_checked(&self) -> bool {
        self.has_state(crate::State::CHECKED)
    }

    /// Set checked state
    pub fn set_checked(&self, checked: bool) {
        if checked {
            self.add_state(crate::State::CHECKED);
        } else {
            self.remove_state(crate::State::CHECKED);
        }
    }
}

impl LvglObj for Checkbox {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Bar (Progress bar)
// ============================================================================

/// Progress bar widget
pub struct Bar {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Bar {
    /// Create a new bar on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_bar_create(parent.raw());
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

    /// Set the bar value
    pub fn set_value(&self, value: i32, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_bar_set_value(self.raw, value, anim_flag) }
    }

    /// Get the bar value
    pub fn get_value(&self) -> i32 {
        unsafe { sys::lv_bar_get_value(self.raw) }
    }

    /// Set the range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { sys::lv_bar_set_range(self.raw, min, max) }
    }
}

impl LvglObj for Bar {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Arc
// ============================================================================

/// Arc/Gauge widget
pub struct Arc {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Arc {
    /// Create a new arc on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_arc_create(parent.raw());
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

    /// Set the arc value
    pub fn set_value(&self, value: i32) {
        unsafe { sys::lv_arc_set_value(self.raw, value) }
    }

    /// Get the arc value
    pub fn get_value(&self) -> i32 {
        unsafe { sys::lv_arc_get_value(self.raw) }
    }

    /// Set the range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { sys::lv_arc_set_range(self.raw, min, max) }
    }

    /// Set background angles
    pub fn set_bg_angles(&self, start: u32, end: u32) {
        unsafe { sys::lv_arc_set_bg_angles(self.raw, start as i32, end as i32) }
    }

    /// Set rotation
    pub fn set_rotation(&self, rotation: i32) {
        unsafe { sys::lv_arc_set_rotation(self.raw, rotation) }
    }

    /// Set arc mode
    pub fn set_mode(&self, mode: ArcMode) {
        unsafe { sys::lv_arc_set_mode(self.raw, mode as u32) }
    }
}

impl LvglObj for Arc {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Arc display mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ArcMode {
    Normal = sys::LV_ARC_MODE_NORMAL as u8,
    Symmetrical = sys::LV_ARC_MODE_SYMMETRICAL as u8,
    Reverse = sys::LV_ARC_MODE_REVERSE as u8,
}

// ============================================================================
// Spinner
// ============================================================================

/// Loading spinner widget
pub struct Spinner {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Spinner {
    /// Create a new spinner on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_spinner_create(parent.raw());
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

    /// Set animation parameters
    pub fn set_anim_params(&self, time_ms: u32, arc_length: u32) {
        unsafe { sys::lv_spinner_set_anim_params(self.raw, time_ms, arc_length) }
    }
}

impl LvglObj for Spinner {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}
