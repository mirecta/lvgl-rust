//! LVGL Display Management
//!
//! Provides safe wrappers for creating and managing LVGL displays.

use crate::{LvglError, Result};
use core::marker::PhantomData;
use core::ptr;
use lvgl_sys as sys;

/// Type alias for the flush callback function
pub type FlushCb = unsafe extern "C" fn(
    disp: *mut sys::lv_display_t,
    area: *const sys::lv_area_t,
    px_map: *mut u8,
);

/// Display wrapper
pub struct Display {
    raw: *mut sys::lv_display_t,
    _marker: PhantomData<*mut ()>,
}

impl Display {
    /// Create a new display with the given resolution
    ///
    /// # Arguments
    /// * `width` - Horizontal resolution in pixels
    /// * `height` - Vertical resolution in pixels
    pub fn create(width: u32, height: u32) -> Result<Self> {
        unsafe {
            let raw = sys::lv_display_create(width as i32, height as i32);
            if raw.is_null() {
                Err(LvglError::DisplayError)
            } else {
                Ok(Self {
                    raw,
                    _marker: PhantomData,
                })
            }
        }
    }

    /// Set the flush callback and buffers
    ///
    /// # Arguments
    /// * `buf1` - Primary draw buffer
    /// * `buf2` - Optional secondary buffer for double-buffering (can be None)
    /// * `flush_cb` - Callback function to flush pixels to the display
    /// * `render_mode` - How rendering should work
    ///
    /// # Safety
    /// Buffers must remain valid for the lifetime of the display.
    pub unsafe fn set_buffers(
        &self,
        buf1: &'static mut [u8],
        buf2: Option<&'static mut [u8]>,
        render_mode: RenderMode,
    ) {
        let buf2_ptr = buf2
            .map(|b| b.as_mut_ptr() as *mut _)
            .unwrap_or(ptr::null_mut());

        sys::lv_display_set_buffers(
            self.raw,
            buf1.as_mut_ptr() as *mut _,
            buf2_ptr,
            buf1.len() as u32,
            render_mode as u32,
        );
    }

    /// Set the flush callback
    pub fn set_flush_cb(&self, flush_cb: FlushCb) {
        unsafe {
            sys::lv_display_set_flush_cb(self.raw, Some(flush_cb));
        }
    }

    /// Signal that flushing is complete
    ///
    /// Call this from your flush callback when the transfer is done.
    pub fn flush_ready(&self) {
        unsafe {
            sys::lv_display_flush_ready(self.raw);
        }
    }

    /// Get raw display pointer (for use in flush callbacks)
    pub fn raw(&self) -> *mut sys::lv_display_t {
        self.raw
    }

    /// Get the horizontal resolution
    pub fn get_hor_res(&self) -> i32 {
        unsafe { sys::lv_display_get_horizontal_resolution(self.raw) }
    }

    /// Get the vertical resolution
    pub fn get_ver_res(&self) -> i32 {
        unsafe { sys::lv_display_get_vertical_resolution(self.raw) }
    }

    /// Set display rotation
    pub fn set_rotation(&self, rotation: DisplayRotation) {
        unsafe { sys::lv_display_set_rotation(self.raw, rotation as u32) }
    }
}

/// Render mode for the display
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum RenderMode {
    /// Partial rendering - only changed areas are rendered
    Partial = sys::LV_DISPLAY_RENDER_MODE_PARTIAL,
    /// Full refresh - entire screen is rendered each frame
    Full = sys::LV_DISPLAY_RENDER_MODE_FULL,
    /// Direct mode - draw buffer is the frame buffer
    Direct = sys::LV_DISPLAY_RENDER_MODE_DIRECT,
}

/// Display rotation
#[derive(Clone, Copy, Debug)]
#[repr(u32)]
pub enum DisplayRotation {
    None = sys::LV_DISPLAY_ROTATION_0,
    Rotate90 = sys::LV_DISPLAY_ROTATION_90,
    Rotate180 = sys::LV_DISPLAY_ROTATION_180,
    Rotate270 = sys::LV_DISPLAY_ROTATION_270,
}

/// Helper to convert an area to coordinates
pub fn area_to_coords(area: &sys::lv_area_t) -> (i32, i32, i32, i32) {
    (area.x1, area.y1, area.x2, area.y2)
}

/// Calculate buffer size needed for a given resolution and color depth
///
/// For partial rendering, a buffer of 1/10th the screen is common.
pub const fn calc_buf_size(width: u32, height: u32, lines: u32) -> usize {
    // RGB565 = 2 bytes per pixel
    (width * lines * 2) as usize
}

/// Macro to create a static display buffer
#[macro_export]
macro_rules! display_buffer {
    ($name:ident, $width:expr, $height:expr, $lines:expr) => {
        static mut $name: [u8; $crate::display::calc_buf_size($width, $height, $lines)] =
            [0u8; $crate::display::calc_buf_size($width, $height, $lines)];
    };
}
