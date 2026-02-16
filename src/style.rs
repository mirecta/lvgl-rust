//! LVGL Style Management
//!
//! Styles define the appearance of objects (colors, borders, padding, etc.)

use crate::Color;
use core::mem::MaybeUninit;
use lvgl_sys as sys;

/// Style wrapper
///
/// Styles are reusable appearance definitions that can be applied to multiple objects.
pub struct Style {
    raw: sys::lv_style_t,
}

impl Style {
    /// Create a new empty style
    pub fn new() -> Self {
        let mut raw = MaybeUninit::<sys::lv_style_t>::uninit();
        unsafe {
            sys::lv_style_init(raw.as_mut_ptr());
            Self {
                raw: raw.assume_init(),
            }
        }
    }

    /// Get raw style pointer
    pub fn raw(&self) -> *const sys::lv_style_t {
        &self.raw
    }

    /// Get mutable raw style pointer
    pub fn raw_mut(&mut self) -> *mut sys::lv_style_t {
        &mut self.raw
    }

    // ========================================================================
    // Background
    // ========================================================================

    /// Set background color
    pub fn set_bg_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_bg_color(&mut self.raw, color.raw()) }
    }

    /// Set background opacity (0-255)
    pub fn set_bg_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_bg_opa(&mut self.raw, opa) }
    }

    /// Set background gradient color
    pub fn set_bg_grad_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_bg_grad_color(&mut self.raw, color.raw()) }
    }

    /// Set background gradient direction
    pub fn set_bg_grad_dir(&mut self, dir: GradDir) {
        unsafe { sys::lv_style_set_bg_grad_dir(&mut self.raw, dir as u32) }
    }

    // ========================================================================
    // Border
    // ========================================================================

    /// Set border color
    pub fn set_border_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_border_color(&mut self.raw, color.raw()) }
    }

    /// Set border width
    pub fn set_border_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_border_width(&mut self.raw, width) }
    }

    /// Set border opacity
    pub fn set_border_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_border_opa(&mut self.raw, opa) }
    }

    /// Set border side
    pub fn set_border_side(&mut self, side: BorderSide) {
        unsafe { sys::lv_style_set_border_side(&mut self.raw, side.0 as u32) }
    }

    // ========================================================================
    // Outline
    // ========================================================================

    /// Set outline color
    pub fn set_outline_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_outline_color(&mut self.raw, color.raw()) }
    }

    /// Set outline width
    pub fn set_outline_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_outline_width(&mut self.raw, width) }
    }

    /// Set outline opacity
    pub fn set_outline_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_outline_opa(&mut self.raw, opa) }
    }

    // ========================================================================
    // Padding
    // ========================================================================

    /// Set all padding
    pub fn set_pad_all(&mut self, pad: i32) {
        self.set_pad_top(pad);
        self.set_pad_bottom(pad);
        self.set_pad_left(pad);
        self.set_pad_right(pad);
    }

    /// Set top padding
    pub fn set_pad_top(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_top(&mut self.raw, pad) }
    }

    /// Set bottom padding
    pub fn set_pad_bottom(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_bottom(&mut self.raw, pad) }
    }

    /// Set left padding
    pub fn set_pad_left(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_left(&mut self.raw, pad) }
    }

    /// Set right padding
    pub fn set_pad_right(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_right(&mut self.raw, pad) }
    }

    /// Set horizontal padding (left and right)
    pub fn set_pad_hor(&mut self, pad: i32) {
        self.set_pad_left(pad);
        self.set_pad_right(pad);
    }

    /// Set vertical padding (top and bottom)
    pub fn set_pad_ver(&mut self, pad: i32) {
        self.set_pad_top(pad);
        self.set_pad_bottom(pad);
    }

    /// Set gap between rows
    pub fn set_pad_row(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_row(&mut self.raw, pad) }
    }

    /// Set gap between columns
    pub fn set_pad_column(&mut self, pad: i32) {
        unsafe { sys::lv_style_set_pad_column(&mut self.raw, pad) }
    }

    // ========================================================================
    // Size
    // ========================================================================

    /// Set width
    pub fn set_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_width(&mut self.raw, width) }
    }

    /// Set height
    pub fn set_height(&mut self, height: i32) {
        unsafe { sys::lv_style_set_height(&mut self.raw, height) }
    }

    /// Set minimum width
    pub fn set_min_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_min_width(&mut self.raw, width) }
    }

    /// Set minimum height
    pub fn set_min_height(&mut self, height: i32) {
        unsafe { sys::lv_style_set_min_height(&mut self.raw, height) }
    }

    /// Set maximum width
    pub fn set_max_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_max_width(&mut self.raw, width) }
    }

    /// Set maximum height
    pub fn set_max_height(&mut self, height: i32) {
        unsafe { sys::lv_style_set_max_height(&mut self.raw, height) }
    }

    // ========================================================================
    // Appearance
    // ========================================================================

    /// Set radius (corner rounding)
    pub fn set_radius(&mut self, radius: i32) {
        unsafe { sys::lv_style_set_radius(&mut self.raw, radius) }
    }

    /// Set opacity
    pub fn set_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_opa(&mut self.raw, opa) }
    }

    // ========================================================================
    // Text
    // ========================================================================

    /// Set text color
    pub fn set_text_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_text_color(&mut self.raw, color.raw()) }
    }

    /// Set text opacity
    pub fn set_text_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_text_opa(&mut self.raw, opa) }
    }

    /// Set text letter spacing
    pub fn set_text_letter_space(&mut self, space: i32) {
        unsafe { sys::lv_style_set_text_letter_space(&mut self.raw, space) }
    }

    /// Set text line spacing
    pub fn set_text_line_space(&mut self, space: i32) {
        unsafe { sys::lv_style_set_text_line_space(&mut self.raw, space) }
    }

    /// Set text alignment
    pub fn set_text_align(&mut self, align: TextAlign) {
        unsafe { sys::lv_style_set_text_align(&mut self.raw, align as u32) }
    }

    // ========================================================================
    // Shadow
    // ========================================================================

    /// Set shadow color
    pub fn set_shadow_color(&mut self, color: Color) {
        unsafe { sys::lv_style_set_shadow_color(&mut self.raw, color.raw()) }
    }

    /// Set shadow width
    pub fn set_shadow_width(&mut self, width: i32) {
        unsafe { sys::lv_style_set_shadow_width(&mut self.raw, width) }
    }

    /// Set shadow offset X
    pub fn set_shadow_offset_x(&mut self, offset: i32) {
        unsafe { sys::lv_style_set_shadow_offset_x(&mut self.raw, offset) }
    }

    /// Set shadow offset Y
    pub fn set_shadow_offset_y(&mut self, offset: i32) {
        unsafe { sys::lv_style_set_shadow_offset_y(&mut self.raw, offset) }
    }

    /// Set shadow spread
    pub fn set_shadow_spread(&mut self, spread: i32) {
        unsafe { sys::lv_style_set_shadow_spread(&mut self.raw, spread) }
    }

    /// Set shadow opacity
    pub fn set_shadow_opa(&mut self, opa: u8) {
        unsafe { sys::lv_style_set_shadow_opa(&mut self.raw, opa) }
    }
}

impl Default for Style {
    fn default() -> Self {
        Self::new()
    }
}

impl Drop for Style {
    fn drop(&mut self) {
        unsafe { sys::lv_style_reset(&mut self.raw) }
    }
}

/// Gradient direction
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum GradDir {
    None = sys::LV_GRAD_DIR_NONE as u8,
    Vertical = sys::LV_GRAD_DIR_VER as u8,
    Horizontal = sys::LV_GRAD_DIR_HOR as u8,
}

/// Border side flags
#[derive(Clone, Copy, Debug)]
pub struct BorderSide(pub u8);

impl BorderSide {
    pub const NONE: Self = Self(sys::LV_BORDER_SIDE_NONE as u8);
    pub const BOTTOM: Self = Self(sys::LV_BORDER_SIDE_BOTTOM as u8);
    pub const TOP: Self = Self(sys::LV_BORDER_SIDE_TOP as u8);
    pub const LEFT: Self = Self(sys::LV_BORDER_SIDE_LEFT as u8);
    pub const RIGHT: Self = Self(sys::LV_BORDER_SIDE_RIGHT as u8);
    pub const FULL: Self = Self(sys::LV_BORDER_SIDE_FULL as u8);
}

/// Text alignment
#[derive(Clone, Copy, Debug)]
#[repr(u8)]
pub enum TextAlign {
    Auto = sys::LV_TEXT_ALIGN_AUTO as u8,
    Left = sys::LV_TEXT_ALIGN_LEFT as u8,
    Center = sys::LV_TEXT_ALIGN_CENTER as u8,
    Right = sys::LV_TEXT_ALIGN_RIGHT as u8,
}
