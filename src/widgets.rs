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

// ============================================================================
// Dropdown
// ============================================================================

/// Dropdown (combo box) widget
pub struct Dropdown {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Dropdown {
    /// Create a new dropdown on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_dropdown_create(parent.raw());
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

    /// Set options as newline-separated string (e.g. c"Option 1\nOption 2\nOption 3")
    pub fn set_options(&self, options: &CStr) {
        unsafe { sys::lv_dropdown_set_options(self.raw, options.as_ptr()) }
    }

    /// Set options using a static string (no copy)
    pub fn set_options_static(&self, options: &'static CStr) {
        unsafe { sys::lv_dropdown_set_options_static(self.raw, options.as_ptr()) }
    }

    /// Add an option at a given position
    pub fn add_option(&self, option: &CStr, pos: u32) {
        unsafe { sys::lv_dropdown_add_option(self.raw, option.as_ptr(), pos) }
    }

    /// Remove all options
    pub fn clear_options(&self) {
        unsafe { sys::lv_dropdown_clear_options(self.raw) }
    }

    /// Set the selected option index
    pub fn set_selected(&self, index: u32) {
        unsafe { sys::lv_dropdown_set_selected(self.raw, index) }
    }

    /// Get the selected option index
    pub fn get_selected(&self) -> u32 {
        unsafe { sys::lv_dropdown_get_selected(self.raw) }
    }

    /// Get the number of options
    pub fn get_option_count(&self) -> u32 {
        unsafe { sys::lv_dropdown_get_option_count(self.raw) }
    }

    /// Set the direction the dropdown list opens
    pub fn set_dir(&self, dir: Dir) {
        unsafe { sys::lv_dropdown_set_dir(self.raw, dir.0) }
    }

    /// Set the static text shown on the button (overrides selected option display)
    pub fn set_text(&self, text: &CStr) {
        unsafe { sys::lv_dropdown_set_text(self.raw, text.as_ptr()) }
    }

    /// Open the dropdown list
    pub fn open(&self) {
        unsafe { sys::lv_dropdown_open(self.raw) }
    }

    /// Close the dropdown list
    pub fn close(&self) {
        unsafe { sys::lv_dropdown_close(self.raw) }
    }

    /// Check if the dropdown list is open
    pub fn is_open(&self) -> bool {
        unsafe { sys::lv_dropdown_is_open(self.raw) }
    }

    /// Enable/disable highlighting the selected option
    pub fn set_selected_highlight(&self, en: bool) {
        unsafe { sys::lv_dropdown_set_selected_highlight(self.raw, en) }
    }
}

impl LvglObj for Dropdown {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Direction flags (used by Dropdown, Tileview, etc.)
#[derive(Clone, Copy, Debug)]
pub struct Dir(pub u32);

impl Dir {
    pub const NONE: Self = Self(sys::LV_DIR_NONE);
    pub const LEFT: Self = Self(sys::LV_DIR_LEFT);
    pub const RIGHT: Self = Self(sys::LV_DIR_RIGHT);
    pub const TOP: Self = Self(sys::LV_DIR_TOP);
    pub const BOTTOM: Self = Self(sys::LV_DIR_BOTTOM);
    pub const HOR: Self = Self(sys::LV_DIR_HOR);
    pub const VER: Self = Self(sys::LV_DIR_VER);
    pub const ALL: Self = Self(sys::LV_DIR_ALL);
}

// ============================================================================
// Textarea
// ============================================================================

/// Text area widget for text input
pub struct Textarea {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Textarea {
    /// Create a new textarea on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_textarea_create(parent.raw());
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

    /// Set the text content
    pub fn set_text(&self, text: &CStr) {
        unsafe { sys::lv_textarea_set_text(self.raw, text.as_ptr()) }
    }

    /// Get the current text as a C string pointer
    ///
    /// # Safety
    /// The returned pointer is valid only while the textarea exists and text is not modified.
    pub fn get_text(&self) -> *const core::ffi::c_char {
        unsafe { sys::lv_textarea_get_text(self.raw) }
    }

    /// Append text at the cursor position
    pub fn add_text(&self, text: &CStr) {
        unsafe { sys::lv_textarea_add_text(self.raw, text.as_ptr()) }
    }

    /// Insert a character at the cursor position
    pub fn add_char(&self, c: u32) {
        unsafe { sys::lv_textarea_add_char(self.raw, c) }
    }

    /// Delete character before the cursor
    pub fn delete_char(&self) {
        unsafe { sys::lv_textarea_delete_char(self.raw) }
    }

    /// Delete character after the cursor
    pub fn delete_char_forward(&self) {
        unsafe { sys::lv_textarea_delete_char_forward(self.raw) }
    }

    /// Set placeholder text
    pub fn set_placeholder_text(&self, text: &CStr) {
        unsafe { sys::lv_textarea_set_placeholder_text(self.raw, text.as_ptr()) }
    }

    /// Set cursor position
    pub fn set_cursor_pos(&self, pos: i32) {
        unsafe { sys::lv_textarea_set_cursor_pos(self.raw, pos) }
    }

    /// Get cursor position
    pub fn get_cursor_pos(&self) -> u32 {
        unsafe { sys::lv_textarea_get_cursor_pos(self.raw) }
    }

    /// Enable/disable password mode (hides characters)
    pub fn set_password_mode(&self, en: bool) {
        unsafe { sys::lv_textarea_set_password_mode(self.raw, en) }
    }

    /// Check if password mode is enabled
    pub fn get_password_mode(&self) -> bool {
        unsafe { sys::lv_textarea_get_password_mode(self.raw) }
    }

    /// Set password show time in ms (how long to show typed char before hiding)
    pub fn set_password_show_time(&self, time: u32) {
        unsafe { sys::lv_textarea_set_password_show_time(self.raw, time) }
    }

    /// Enable/disable one-line mode
    pub fn set_one_line(&self, en: bool) {
        unsafe { sys::lv_textarea_set_one_line(self.raw, en) }
    }

    /// Check if one-line mode is enabled
    pub fn get_one_line(&self) -> bool {
        unsafe { sys::lv_textarea_get_one_line(self.raw) }
    }

    /// Set maximum text length (0 = no limit)
    pub fn set_max_length(&self, num: u32) {
        unsafe { sys::lv_textarea_set_max_length(self.raw, num) }
    }

    /// Get maximum text length
    pub fn get_max_length(&self) -> u32 {
        unsafe { sys::lv_textarea_get_max_length(self.raw) }
    }

    /// Set accepted characters (only these can be typed)
    pub fn set_accepted_chars(&self, chars: &CStr) {
        unsafe { sys::lv_textarea_set_accepted_chars(self.raw, chars.as_ptr()) }
    }

    /// Enable/disable text selection
    pub fn set_text_selection(&self, en: bool) {
        unsafe { sys::lv_textarea_set_text_selection(self.raw, en) }
    }

    /// Move cursor right
    pub fn cursor_right(&self) {
        unsafe { sys::lv_textarea_cursor_right(self.raw) }
    }

    /// Move cursor left
    pub fn cursor_left(&self) {
        unsafe { sys::lv_textarea_cursor_left(self.raw) }
    }

    /// Move cursor up
    pub fn cursor_up(&self) {
        unsafe { sys::lv_textarea_cursor_up(self.raw) }
    }

    /// Move cursor down
    pub fn cursor_down(&self) {
        unsafe { sys::lv_textarea_cursor_down(self.raw) }
    }
}

impl LvglObj for Textarea {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Roller
// ============================================================================

/// Roller (scroll wheel selector) widget
pub struct Roller {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Roller {
    /// Create a new roller on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_roller_create(parent.raw());
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

    /// Set options as newline-separated string
    pub fn set_options(&self, options: &CStr, mode: RollerMode) {
        unsafe { sys::lv_roller_set_options(self.raw, options.as_ptr(), mode as u32) }
    }

    /// Get the number of options
    pub fn get_option_count(&self) -> u32 {
        unsafe { sys::lv_roller_get_option_count(self.raw) }
    }

    /// Set the selected option
    pub fn set_selected(&self, index: u32, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_roller_set_selected(self.raw, index, anim_flag) }
    }

    /// Get the selected option index
    pub fn get_selected(&self) -> u32 {
        unsafe { sys::lv_roller_get_selected(self.raw) }
    }

    /// Set the number of visible rows
    pub fn set_visible_row_count(&self, count: u32) {
        unsafe { sys::lv_roller_set_visible_row_count(self.raw, count) }
    }
}

impl LvglObj for Roller {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Roller mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum RollerMode {
    Normal = sys::LV_ROLLER_MODE_NORMAL as u8,
    Infinite = sys::LV_ROLLER_MODE_INFINITE as u8,
}

// ============================================================================
// LED
// ============================================================================

/// LED indicator widget
pub struct Led {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Led {
    /// Create a new LED on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_led_create(parent.raw());
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

    /// Set the LED color
    pub fn set_color(&self, color: Color) {
        unsafe { sys::lv_led_set_color(self.raw, color.raw()) }
    }

    /// Set brightness (0-255)
    pub fn set_brightness(&self, bright: u8) {
        unsafe { sys::lv_led_set_brightness(self.raw, bright) }
    }

    /// Get brightness
    pub fn get_brightness(&self) -> u8 {
        unsafe { sys::lv_led_get_brightness(self.raw) }
    }

    /// Turn on (max brightness)
    pub fn on(&self) {
        unsafe { sys::lv_led_on(self.raw) }
    }

    /// Turn off (min brightness)
    pub fn off(&self) {
        unsafe { sys::lv_led_off(self.raw) }
    }

    /// Toggle on/off
    pub fn toggle(&self) {
        unsafe { sys::lv_led_toggle(self.raw) }
    }
}

impl LvglObj for Led {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Line
// ============================================================================

/// Line drawing widget
pub struct Line {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Line {
    /// Create a new line on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_line_create(parent.raw());
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

    /// Set the line points
    ///
    /// # Safety
    /// The `points` slice must remain valid for the lifetime of the line object.
    pub unsafe fn set_points(&self, points: &[sys::lv_point_precise_t]) {
        sys::lv_line_set_points(self.raw, points.as_ptr(), points.len() as u32)
    }

    /// Get the number of points
    pub fn get_point_count(&self) -> u32 {
        unsafe { sys::lv_line_get_point_count(self.raw) }
    }

    /// Enable/disable Y axis inversion (0 at top vs bottom)
    pub fn set_y_invert(&self, en: bool) {
        unsafe { sys::lv_line_set_y_invert(self.raw, en) }
    }

    /// Check if Y axis is inverted
    pub fn get_y_invert(&self) -> bool {
        unsafe { sys::lv_line_get_y_invert(self.raw) }
    }
}

impl LvglObj for Line {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Image
// ============================================================================

/// Image widget
pub struct Image {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Image {
    /// Create a new image on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_image_create(parent.raw());
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

    /// Set the image source (pointer to lv_image_dsc_t or a path string)
    ///
    /// # Safety
    /// The source must remain valid for the lifetime of the image object.
    pub unsafe fn set_src(&self, src: *const core::ffi::c_void) {
        sys::lv_image_set_src(self.raw, src)
    }

    /// Set rotation in 0.1 degree units (e.g. 900 = 90 degrees)
    pub fn set_rotation(&self, angle: i32) {
        unsafe { sys::lv_image_set_rotation(self.raw, angle) }
    }

    /// Get rotation
    pub fn get_rotation(&self) -> i32 {
        unsafe { sys::lv_image_get_rotation(self.raw) }
    }

    /// Set the pivot point for rotation
    pub fn set_pivot(&self, x: i32, y: i32) {
        unsafe { sys::lv_image_set_pivot(self.raw, x, y) }
    }

    /// Set scale (256 = 100%, 512 = 200%, 128 = 50%)
    pub fn set_scale(&self, zoom: u32) {
        unsafe { sys::lv_image_set_scale(self.raw, zoom) }
    }

    /// Set X scale separately
    pub fn set_scale_x(&self, zoom: u32) {
        unsafe { sys::lv_image_set_scale_x(self.raw, zoom) }
    }

    /// Set Y scale separately
    pub fn set_scale_y(&self, zoom: u32) {
        unsafe { sys::lv_image_set_scale_y(self.raw, zoom) }
    }

    /// Get scale
    pub fn get_scale(&self) -> i32 {
        unsafe { sys::lv_image_get_scale(self.raw) }
    }

    /// Set X offset
    pub fn set_offset_x(&self, x: i32) {
        unsafe { sys::lv_image_set_offset_x(self.raw, x) }
    }

    /// Set Y offset
    pub fn set_offset_y(&self, y: i32) {
        unsafe { sys::lv_image_set_offset_y(self.raw, y) }
    }

    /// Enable/disable anti-aliasing for transformations
    pub fn set_antialias(&self, en: bool) {
        unsafe { sys::lv_image_set_antialias(self.raw, en) }
    }

    /// Set inner alignment
    pub fn set_inner_align(&self, align: ImageAlign) {
        unsafe { sys::lv_image_set_inner_align(self.raw, align as u32) }
    }
}

impl LvglObj for Image {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Image inner alignment
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ImageAlign {
    Default = sys::LV_IMAGE_ALIGN_DEFAULT as u8,
    TopLeft = sys::LV_IMAGE_ALIGN_TOP_LEFT as u8,
    TopMid = sys::LV_IMAGE_ALIGN_TOP_MID as u8,
    TopRight = sys::LV_IMAGE_ALIGN_TOP_RIGHT as u8,
    BottomLeft = sys::LV_IMAGE_ALIGN_BOTTOM_LEFT as u8,
    BottomMid = sys::LV_IMAGE_ALIGN_BOTTOM_MID as u8,
    BottomRight = sys::LV_IMAGE_ALIGN_BOTTOM_RIGHT as u8,
    LeftMid = sys::LV_IMAGE_ALIGN_LEFT_MID as u8,
    RightMid = sys::LV_IMAGE_ALIGN_RIGHT_MID as u8,
    Center = sys::LV_IMAGE_ALIGN_CENTER as u8,
    Stretch = sys::LV_IMAGE_ALIGN_STRETCH as u8,
    Tile = sys::LV_IMAGE_ALIGN_TILE as u8,
}

// ============================================================================
// Spinbox
// ============================================================================

/// Numeric spinbox widget
pub struct Spinbox {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Spinbox {
    /// Create a new spinbox on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_spinbox_create(parent.raw());
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

    /// Set the value
    pub fn set_value(&self, value: i32) {
        unsafe { sys::lv_spinbox_set_value(self.raw, value) }
    }

    /// Get the value
    pub fn get_value(&self) -> i32 {
        unsafe { sys::lv_spinbox_get_value(self.raw) }
    }

    /// Set the range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { sys::lv_spinbox_set_range(self.raw, min, max) }
    }

    /// Set digit format (total digits, separator position from right)
    pub fn set_digit_format(&self, digit_count: u32, separator_pos: u32) {
        unsafe { sys::lv_spinbox_set_digit_format(self.raw, digit_count, separator_pos) }
    }

    /// Set step size
    pub fn set_step(&self, step: u32) {
        unsafe { sys::lv_spinbox_set_step(self.raw, step) }
    }

    /// Get current step size
    pub fn get_step(&self) -> i32 {
        unsafe { sys::lv_spinbox_get_step(self.raw) }
    }

    /// Increment value by current step
    pub fn increment(&self) {
        unsafe { sys::lv_spinbox_increment(self.raw) }
    }

    /// Decrement value by current step
    pub fn decrement(&self) {
        unsafe { sys::lv_spinbox_decrement(self.raw) }
    }

    /// Enable/disable rollover (wrap around at min/max)
    pub fn set_rollover(&self, en: bool) {
        unsafe { sys::lv_spinbox_set_rollover(self.raw, en) }
    }

    /// Check if rollover is enabled
    pub fn get_rollover(&self) -> bool {
        unsafe { sys::lv_spinbox_get_rollover(self.raw) }
    }
}

impl LvglObj for Spinbox {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Scale
// ============================================================================

/// Scale (ruler/gauge marks) widget
pub struct Scale {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Scale {
    /// Create a new scale on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_scale_create(parent.raw());
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

    /// Set the scale mode (horizontal, vertical, or round)
    pub fn set_mode(&self, mode: ScaleMode) {
        unsafe { sys::lv_scale_set_mode(self.raw, mode as u32) }
    }

    /// Get the scale mode
    pub fn get_mode(&self) -> u32 {
        unsafe { sys::lv_scale_get_mode(self.raw) }
    }

    /// Set the value range
    pub fn set_range(&self, min: i32, max: i32) {
        unsafe { sys::lv_scale_set_range(self.raw, min, max) }
    }

    /// Set total number of ticks
    pub fn set_total_tick_count(&self, count: u32) {
        unsafe { sys::lv_scale_set_total_tick_count(self.raw, count) }
    }

    /// Set how often a major tick appears
    pub fn set_major_tick_every(&self, nth: u32) {
        unsafe { sys::lv_scale_set_major_tick_every(self.raw, nth) }
    }

    /// Show/hide labels on major ticks
    pub fn set_label_show(&self, show: bool) {
        unsafe { sys::lv_scale_set_label_show(self.raw, show) }
    }

    /// Set angle range for round mode (in degrees)
    pub fn set_angle_range(&self, angle: u32) {
        unsafe { sys::lv_scale_set_angle_range(self.raw, angle) }
    }

    /// Set rotation offset for round mode
    pub fn set_rotation(&self, rotation: i32) {
        unsafe { sys::lv_scale_set_rotation(self.raw, rotation) }
    }
}

impl LvglObj for Scale {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Scale mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ScaleMode {
    HorizontalTop = sys::LV_SCALE_MODE_HORIZONTAL_TOP as u8,
    HorizontalBottom = sys::LV_SCALE_MODE_HORIZONTAL_BOTTOM as u8,
    VerticalLeft = sys::LV_SCALE_MODE_VERTICAL_LEFT as u8,
    VerticalRight = sys::LV_SCALE_MODE_VERTICAL_RIGHT as u8,
    RoundInner = sys::LV_SCALE_MODE_ROUND_INNER as u8,
    RoundOuter = sys::LV_SCALE_MODE_ROUND_OUTER as u8,
}

// ============================================================================
// Buttonmatrix
// ============================================================================

/// Button matrix widget
pub struct Buttonmatrix {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Buttonmatrix {
    /// Create a new button matrix on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_buttonmatrix_create(parent.raw());
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

    /// Set the button map (null-terminated array of C strings, "" for newline)
    ///
    /// # Safety
    /// The map must remain valid for the lifetime of the buttonmatrix.
    pub unsafe fn set_map(&self, map: &[*const core::ffi::c_char]) {
        sys::lv_buttonmatrix_set_map(self.raw, map.as_ptr())
    }

    /// Get the selected button index (LV_BUTTONMATRIX_BUTTON_NONE if none)
    pub fn get_selected_button(&self) -> u32 {
        unsafe { sys::lv_buttonmatrix_get_selected_button(self.raw) }
    }

    /// Set the selected button
    pub fn set_selected_button(&self, btn_id: u32) {
        unsafe { sys::lv_buttonmatrix_set_selected_button(self.raw, btn_id) }
    }

    /// Set control flags for a button
    pub fn set_button_ctrl(&self, btn_id: u32, ctrl: u32) {
        unsafe {
            sys::lv_buttonmatrix_set_button_ctrl(
                self.raw,
                btn_id,
                ctrl as sys::lv_buttonmatrix_ctrl_t,
            )
        }
    }

    /// Clear control flags for a button
    pub fn clear_button_ctrl(&self, btn_id: u32, ctrl: u32) {
        unsafe {
            sys::lv_buttonmatrix_clear_button_ctrl(
                self.raw,
                btn_id,
                ctrl as sys::lv_buttonmatrix_ctrl_t,
            )
        }
    }

    /// Set control flags for all buttons
    pub fn set_button_ctrl_all(&self, ctrl: u32) {
        unsafe {
            sys::lv_buttonmatrix_set_button_ctrl_all(self.raw, ctrl as sys::lv_buttonmatrix_ctrl_t)
        }
    }

    /// Enable/disable "one checked" mode (radio button behavior)
    pub fn set_one_checked(&self, en: bool) {
        unsafe { sys::lv_buttonmatrix_set_one_checked(self.raw, en) }
    }

    /// Get button text by index
    pub fn get_button_text(&self, btn_id: u32) -> *const core::ffi::c_char {
        unsafe { sys::lv_buttonmatrix_get_button_text(self.raw, btn_id) }
    }

    /// Set button width
    pub fn set_button_width(&self, btn_id: u32, width: u32) {
        unsafe { sys::lv_buttonmatrix_set_button_width(self.raw, btn_id, width) }
    }
}

impl LvglObj for Buttonmatrix {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Table
// ============================================================================

/// Table widget
pub struct Table {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Table {
    /// Create a new table on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_table_create(parent.raw());
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

    /// Set the number of rows
    pub fn set_row_count(&self, count: u32) {
        unsafe { sys::lv_table_set_row_count(self.raw, count) }
    }

    /// Get the number of rows
    pub fn get_row_count(&self) -> u32 {
        unsafe { sys::lv_table_get_row_count(self.raw) }
    }

    /// Set the number of columns
    pub fn set_column_count(&self, count: u32) {
        unsafe { sys::lv_table_set_column_count(self.raw, count) }
    }

    /// Get the number of columns
    pub fn get_column_count(&self) -> u32 {
        unsafe { sys::lv_table_get_column_count(self.raw) }
    }

    /// Set the width of a column
    pub fn set_column_width(&self, col: u32, width: i32) {
        unsafe { sys::lv_table_set_column_width(self.raw, col, width) }
    }

    /// Get the width of a column
    pub fn get_column_width(&self, col: u32) -> i32 {
        unsafe { sys::lv_table_get_column_width(self.raw, col) }
    }

    /// Set cell value (text)
    pub fn set_cell_value(&self, row: u32, col: u32, text: &CStr) {
        unsafe { sys::lv_table_set_cell_value(self.raw, row, col, text.as_ptr()) }
    }

    /// Get cell value as C string pointer
    pub fn get_cell_value(&self, row: u32, col: u32) -> *const core::ffi::c_char {
        unsafe { sys::lv_table_get_cell_value(self.raw, row, col) }
    }

    /// Get the selected cell (row, col)
    pub fn get_selected_cell(&self) -> (u32, u32) {
        let mut row = 0u32;
        let mut col = 0u32;
        unsafe { sys::lv_table_get_selected_cell(self.raw, &mut row, &mut col) }
        (row, col)
    }

    /// Set selected cell
    pub fn set_selected_cell(&self, row: u16, col: u16) {
        unsafe { sys::lv_table_set_selected_cell(self.raw, row, col) }
    }
}

impl LvglObj for Table {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Chart
// ============================================================================

/// Chart widget for data visualization
pub struct Chart {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

/// Opaque wrapper for a chart data series
pub struct ChartSeries {
    raw: *mut sys::lv_chart_series_t,
}

impl Chart {
    /// Create a new chart on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_chart_create(parent.raw());
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

    /// Set the chart type
    pub fn set_type(&self, chart_type: ChartType) {
        unsafe { sys::lv_chart_set_type(self.raw, chart_type as u32) }
    }

    /// Set the number of data points per series
    pub fn set_point_count(&self, count: u32) {
        unsafe { sys::lv_chart_set_point_count(self.raw, count) }
    }

    /// Get the number of data points
    pub fn get_point_count(&self) -> u32 {
        unsafe { sys::lv_chart_get_point_count(self.raw) }
    }

    /// Set the value range for an axis
    pub fn set_range(&self, axis: ChartAxis, min: i32, max: i32) {
        unsafe { sys::lv_chart_set_range(self.raw, axis as u32, min, max) }
    }

    /// Set the number of horizontal and vertical division lines
    pub fn set_div_line_count(&self, hdiv: u8, vdiv: u8) {
        unsafe { sys::lv_chart_set_div_line_count(self.raw, hdiv, vdiv) }
    }

    /// Add a data series to the chart
    pub fn add_series(&self, color: Color, axis: ChartAxis) -> ChartSeries {
        unsafe {
            let raw = sys::lv_chart_add_series(self.raw, color.raw(), axis as u32);
            ChartSeries { raw }
        }
    }

    /// Remove a data series
    pub fn remove_series(&self, series: &ChartSeries) {
        unsafe { sys::lv_chart_remove_series(self.raw, series.raw) }
    }

    /// Hide/show a series
    pub fn hide_series(&self, series: &ChartSeries, hide: bool) {
        unsafe { sys::lv_chart_hide_series(self.raw, series.raw, hide) }
    }

    /// Add the next value to a series (circular buffer)
    pub fn set_next_value(&self, series: &ChartSeries, value: i32) {
        unsafe { sys::lv_chart_set_next_value(self.raw, series.raw, value) }
    }

    /// Set all values of a series
    pub fn set_all_value(&self, series: &ChartSeries, value: i32) {
        unsafe { sys::lv_chart_set_all_value(self.raw, series.raw, value) }
    }

    /// Set a specific value by index
    pub fn set_value_by_id(&self, series: &ChartSeries, id: u32, value: i32) {
        unsafe { sys::lv_chart_set_value_by_id(self.raw, series.raw, id, value) }
    }

    /// Set the update mode
    pub fn set_update_mode(&self, mode: ChartUpdateMode) {
        unsafe { sys::lv_chart_set_update_mode(self.raw, mode as u32) }
    }

    /// Refresh the chart (call after modifying data externally)
    pub fn refresh(&self) {
        unsafe { sys::lv_chart_refresh(self.raw) }
    }

    /// Get the index of the pressed point
    pub fn get_pressed_point(&self) -> u32 {
        unsafe { sys::lv_chart_get_pressed_point(self.raw) }
    }
}

impl LvglObj for Chart {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Chart type
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ChartType {
    None = sys::LV_CHART_TYPE_NONE,
    Line = sys::LV_CHART_TYPE_LINE,
    Bar = sys::LV_CHART_TYPE_BAR,
    Scatter = sys::LV_CHART_TYPE_SCATTER,
}

/// Chart axis
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum ChartAxis {
    PrimaryY = sys::LV_CHART_AXIS_PRIMARY_Y,
    SecondaryY = sys::LV_CHART_AXIS_SECONDARY_Y,
    PrimaryX = sys::LV_CHART_AXIS_PRIMARY_X,
    SecondaryX = sys::LV_CHART_AXIS_SECONDARY_X,
}

/// Chart update mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum ChartUpdateMode {
    Shift = sys::LV_CHART_UPDATE_MODE_SHIFT as u8,
    Circular = sys::LV_CHART_UPDATE_MODE_CIRCULAR as u8,
}

// ============================================================================
// List
// ============================================================================

/// List widget (scrollable list of items)
pub struct List {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl List {
    /// Create a new list on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_list_create(parent.raw());
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

    /// Add a text separator item
    pub fn add_text(&self, text: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_list_add_text(self.raw, text.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Add a button item (with optional icon and text)
    ///
    /// Pass `ptr::null()` for icon to create a text-only button.
    pub fn add_button(&self, icon: *const core::ffi::c_void, text: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_list_add_button(self.raw, icon, text.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Get the text of a list button
    pub fn get_button_text(&self, btn: &impl LvglObj) -> *const core::ffi::c_char {
        unsafe { sys::lv_list_get_button_text(self.raw, btn.raw()) }
    }
}

impl LvglObj for List {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Msgbox (Message Box)
// ============================================================================

/// Message box widget
pub struct Msgbox {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Msgbox {
    /// Create a new message box on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_msgbox_create(parent.raw());
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

    /// Add a title
    pub fn add_title(&self, title: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_msgbox_add_title(self.raw, title.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Add body text
    pub fn add_text(&self, text: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_msgbox_add_text(self.raw, text.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Add a footer button
    pub fn add_footer_button(&self, text: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_msgbox_add_footer_button(self.raw, text.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Add a close button to the header
    pub fn add_close_button(&self) -> Obj {
        unsafe {
            let raw = sys::lv_msgbox_add_close_button(self.raw);
            Obj::from_raw(raw)
        }
    }

    /// Get the content area (for adding custom widgets)
    pub fn get_content(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_msgbox_get_content(self.raw)) }
    }

    /// Get the header
    pub fn get_header(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_msgbox_get_header(self.raw)) }
    }

    /// Get the footer
    pub fn get_footer(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_msgbox_get_footer(self.raw)) }
    }

    /// Close the message box
    pub fn close(&self) {
        unsafe { sys::lv_msgbox_close(self.raw) }
    }

    /// Close the message box asynchronously (safe to call from event callbacks)
    pub fn close_async(&self) {
        unsafe { sys::lv_msgbox_close_async(self.raw) }
    }
}

impl LvglObj for Msgbox {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Tabview
// ============================================================================

/// Tabview widget (container with switchable tabs)
pub struct Tabview {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Tabview {
    /// Create a new tabview on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_tabview_create(parent.raw());
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

    /// Add a tab and return its content area
    pub fn add_tab(&self, name: &CStr) -> Obj {
        unsafe {
            let raw = sys::lv_tabview_add_tab(self.raw, name.as_ptr());
            Obj::from_raw(raw)
        }
    }

    /// Rename a tab by index
    pub fn rename_tab(&self, index: u32, name: &CStr) {
        unsafe { sys::lv_tabview_rename_tab(self.raw, index, name.as_ptr()) }
    }

    /// Set the active tab
    pub fn set_active(&self, index: u32, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_tabview_set_active(self.raw, index, anim_flag) }
    }

    /// Get the active tab index
    pub fn get_tab_active(&self) -> u32 {
        unsafe { sys::lv_tabview_get_tab_active(self.raw) }
    }

    /// Get the number of tabs
    pub fn get_tab_count(&self) -> u32 {
        unsafe { sys::lv_tabview_get_tab_count(self.raw) }
    }

    /// Get the content container
    pub fn get_content(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_tabview_get_content(self.raw)) }
    }

    /// Get the tab bar
    pub fn get_tab_bar(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_tabview_get_tab_bar(self.raw)) }
    }

    /// Set the tab bar position
    pub fn set_tab_bar_position(&self, dir: Dir) {
        unsafe { sys::lv_tabview_set_tab_bar_position(self.raw, dir.0) }
    }

    /// Set the tab bar size (height if top/bottom, width if left/right)
    pub fn set_tab_bar_size(&self, size: i32) {
        unsafe { sys::lv_tabview_set_tab_bar_size(self.raw, size) }
    }
}

impl LvglObj for Tabview {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Tileview
// ============================================================================

/// Tileview widget (swipeable page grid)
pub struct Tileview {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Tileview {
    /// Create a new tileview on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_tileview_create(parent.raw());
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

    /// Add a tile at grid position (col, row) with allowed swipe directions
    pub fn add_tile(&self, col: u8, row: u8, dir: Dir) -> Obj {
        unsafe {
            let raw = sys::lv_tileview_add_tile(self.raw, col, row, dir.0);
            Obj::from_raw(raw)
        }
    }

    /// Set the active tile by object
    pub fn set_tile(&self, tile: &impl LvglObj, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_tileview_set_tile(self.raw, tile.raw(), anim_flag) }
    }

    /// Set the active tile by grid index
    pub fn set_tile_by_index(&self, col: u32, row: u32, anim: bool) {
        let anim_flag = if anim {
            sys::LV_ANIM_ON
        } else {
            sys::LV_ANIM_OFF
        };
        unsafe { sys::lv_tileview_set_tile_by_index(self.raw, col, row, anim_flag) }
    }

    /// Get the currently active tile
    pub fn get_tile_active(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_tileview_get_tile_active(self.raw)) }
    }
}

impl LvglObj for Tileview {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Calendar
// ============================================================================

/// Calendar widget
pub struct Calendar {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Calendar {
    /// Create a new calendar on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_calendar_create(parent.raw());
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

    /// Set today's date
    pub fn set_today_date(&self, year: u32, month: u32, day: u32) {
        unsafe { sys::lv_calendar_set_today_date(self.raw, year, month, day) }
    }

    /// Set the currently shown month/year
    pub fn set_showed_date(&self, year: u32, month: u32) {
        unsafe { sys::lv_calendar_set_showed_date(self.raw, year, month) }
    }

    /// Set highlighted dates
    ///
    /// # Safety
    /// The dates slice must remain valid for the lifetime of the calendar.
    pub unsafe fn set_highlighted_dates(&self, dates: &mut [sys::lv_calendar_date_t]) {
        sys::lv_calendar_set_highlighted_dates(self.raw, dates.as_mut_ptr(), dates.len())
    }

    /// Get the pressed date (returns None if no date pressed)
    pub fn get_pressed_date(&self) -> Option<(u32, u32, u32)> {
        let mut date = sys::lv_calendar_date_t {
            year: 0,
            month: 0,
            day: 0,
        };
        let res = unsafe { sys::lv_calendar_get_pressed_date(self.raw, &mut date) };
        if res == sys::LV_RESULT_OK {
            Some((date.year as u32, date.month as u32, date.day as u32))
        } else {
            None
        }
    }

    /// Add a header with arrow navigation
    pub fn add_header_arrow(parent: &impl LvglObj) -> Obj {
        unsafe { Obj::from_raw(sys::lv_calendar_header_arrow_create(parent.raw())) }
    }

    /// Add a header with dropdown navigation
    pub fn add_header_dropdown(parent: &impl LvglObj) -> Obj {
        unsafe { Obj::from_raw(sys::lv_calendar_header_dropdown_create(parent.raw())) }
    }
}

impl LvglObj for Calendar {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Keyboard
// ============================================================================

/// On-screen keyboard widget
pub struct Keyboard {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Keyboard {
    /// Create a new keyboard on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_keyboard_create(parent.raw());
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

    /// Link a textarea to receive keyboard input
    pub fn set_textarea(&self, ta: &Textarea) {
        unsafe { sys::lv_keyboard_set_textarea(self.raw, ta.raw()) }
    }

    /// Get the linked textarea (may be null)
    pub fn get_textarea_raw(&self) -> *mut sys::lv_obj_t {
        unsafe { sys::lv_keyboard_get_textarea(self.raw) }
    }

    /// Set the keyboard mode
    pub fn set_mode(&self, mode: KeyboardMode) {
        unsafe { sys::lv_keyboard_set_mode(self.raw, mode as u32) }
    }

    /// Get the keyboard mode
    pub fn get_mode(&self) -> u32 {
        unsafe { sys::lv_keyboard_get_mode(self.raw) }
    }

    /// Enable/disable key popovers
    pub fn set_popovers(&self, en: bool) {
        unsafe { sys::lv_keyboard_set_popovers(self.raw, en) }
    }
}

impl LvglObj for Keyboard {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Keyboard mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum KeyboardMode {
    TextLower = sys::LV_KEYBOARD_MODE_TEXT_LOWER as u8,
    TextUpper = sys::LV_KEYBOARD_MODE_TEXT_UPPER as u8,
    Special = sys::LV_KEYBOARD_MODE_SPECIAL as u8,
    Number = sys::LV_KEYBOARD_MODE_NUMBER as u8,
}

// ============================================================================
// Menu
// ============================================================================

/// Menu widget (hierarchical navigation)
pub struct Menu {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Menu {
    /// Create a new menu on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_menu_create(parent.raw());
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

    /// Create a menu page
    pub fn page_create(&self, title: &CStr) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_page_create(self.raw, title.as_ptr())) }
    }

    /// Create a menu content container (add items to this)
    pub fn cont_create(&self, parent: &impl LvglObj) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_cont_create(parent.raw())) }
    }

    /// Create a section (visual grouping)
    pub fn section_create(&self, parent: &impl LvglObj) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_section_create(parent.raw())) }
    }

    /// Create a separator line
    pub fn separator_create(&self, parent: &impl LvglObj) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_separator_create(parent.raw())) }
    }

    /// Set the main page
    pub fn set_page(&self, page: &impl LvglObj) {
        unsafe { sys::lv_menu_set_page(self.raw, page.raw()) }
    }

    /// Set the sidebar page
    pub fn set_sidebar_page(&self, page: &impl LvglObj) {
        unsafe { sys::lv_menu_set_sidebar_page(self.raw, page.raw()) }
    }

    /// Set header mode
    pub fn set_mode_header(&self, mode: MenuModeHeader) {
        unsafe { sys::lv_menu_set_mode_header(self.raw, mode as u32) }
    }

    /// Set root back button mode
    pub fn set_mode_root_back_button(&self, mode: MenuModeRootBackButton) {
        unsafe { sys::lv_menu_set_mode_root_back_button(self.raw, mode as u32) }
    }

    /// Set a load-page event on an object (clicking it navigates to a page)
    pub fn set_load_page_event(&self, obj: &impl LvglObj, page: &impl LvglObj) {
        unsafe { sys::lv_menu_set_load_page_event(self.raw, obj.raw(), page.raw()) }
    }

    /// Clear navigation history
    pub fn clear_history(&self) {
        unsafe { sys::lv_menu_clear_history(self.raw) }
    }

    /// Get the current main page
    pub fn get_cur_main_page(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_get_cur_main_page(self.raw)) }
    }

    /// Get the main header
    pub fn get_main_header(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_menu_get_main_header(self.raw)) }
    }
}

impl LvglObj for Menu {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

/// Menu header mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MenuModeHeader {
    TopFixed = sys::LV_MENU_HEADER_TOP_FIXED as u8,
    TopUnfixed = sys::LV_MENU_HEADER_TOP_UNFIXED as u8,
    BottomFixed = sys::LV_MENU_HEADER_BOTTOM_FIXED as u8,
}

/// Menu root back button mode
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum MenuModeRootBackButton {
    Disabled = sys::LV_MENU_ROOT_BACK_BUTTON_DISABLED as u8,
    Enabled = sys::LV_MENU_ROOT_BACK_BUTTON_ENABLED as u8,
}

// ============================================================================
// Canvas (requires LV_USE_CANVAS — disabled on ESP32 by default)
// ============================================================================

/// Canvas widget for pixel-level drawing
///
/// Only available with the `simulator` feature (or when `LV_USE_CANVAS = 1`
/// in your `lv_conf.h`). Requires a large pixel buffer.
#[cfg(feature = "simulator")]
pub struct Canvas {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

#[cfg(feature = "simulator")]
impl Canvas {
    /// Create a new canvas on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_canvas_create(parent.raw());
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

    /// Set the canvas buffer
    ///
    /// # Safety
    /// The buffer must remain valid and large enough for the given dimensions and color format.
    pub unsafe fn set_buffer(&self, buf: *mut core::ffi::c_void, w: i32, h: i32, cf: u32) {
        sys::lv_canvas_set_buffer(self.raw, buf, w, h, cf)
    }

    /// Set a pixel color
    pub fn set_px(&self, x: i32, y: i32, color: Color, opa: u8) {
        unsafe { sys::lv_canvas_set_px(self.raw, x, y, color.raw(), opa) }
    }

    /// Fill the entire canvas with a color
    pub fn fill_bg(&self, color: Color, opa: u8) {
        unsafe { sys::lv_canvas_fill_bg(self.raw, color.raw(), opa) }
    }
}

#[cfg(feature = "simulator")]
impl LvglObj for Canvas {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// ============================================================================
// Win (Window)
// ============================================================================

/// Window widget (title bar + content area)
pub struct Win {
    raw: *mut sys::lv_obj_t,
    _marker: PhantomData<*mut ()>,
}

impl Win {
    /// Create a new window on the given parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_win_create(parent.raw());
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

    /// Add a title to the header
    pub fn add_title(&self, text: &CStr) -> Obj {
        unsafe { Obj::from_raw(sys::lv_win_add_title(self.raw, text.as_ptr())) }
    }

    /// Add a button to the header
    ///
    /// Pass an icon source (or `ptr::null()`) and button width.
    pub fn add_button(&self, icon: *const core::ffi::c_void, btn_w: i32) -> Obj {
        unsafe { Obj::from_raw(sys::lv_win_add_button(self.raw, icon, btn_w)) }
    }

    /// Get the header
    pub fn get_header(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_win_get_header(self.raw)) }
    }

    /// Get the content area
    pub fn get_content(&self) -> Obj {
        unsafe { Obj::from_raw(sys::lv_win_get_content(self.raw)) }
    }
}

impl LvglObj for Win {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}
