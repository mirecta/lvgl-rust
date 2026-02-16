//! Base LVGL object wrapper
//!
//! All LVGL widgets inherit from lv_obj, so this provides common functionality.

use crate::{Align, Color, LvglError, Part, Result, State, Style};
use alloc::boxed::Box;
use core::ffi::c_void;
use core::marker::PhantomData;
use lvgl_sys as sys;

/// Trait for types that wrap LVGL objects
pub trait LvglObj {
    /// Get the raw LVGL object pointer
    fn raw(&self) -> *mut sys::lv_obj_t;

    /// Set position
    fn set_pos(&self, x: i32, y: i32) {
        unsafe { sys::lv_obj_set_pos(self.raw(), x, y) }
    }

    /// Set size
    fn set_size(&self, width: i32, height: i32) {
        unsafe { sys::lv_obj_set_size(self.raw(), width, height) }
    }

    /// Set width
    fn set_width(&self, width: i32) {
        unsafe { sys::lv_obj_set_width(self.raw(), width) }
    }

    /// Set height
    fn set_height(&self, height: i32) {
        unsafe { sys::lv_obj_set_height(self.raw(), height) }
    }

    /// Align object relative to parent
    fn align(&self, align: Align, x_ofs: i32, y_ofs: i32) {
        unsafe { sys::lv_obj_align(self.raw(), align as u32, x_ofs, y_ofs) }
    }

    /// Center the object in its parent
    fn center(&self) {
        unsafe { sys::lv_obj_center(self.raw()) }
    }

    /// Set alignment to center
    fn set_align(&self, align: Align) {
        unsafe { sys::lv_obj_set_align(self.raw(), align as u32) }
    }

    /// Add style to the object
    fn add_style(&self, style: &Style, selector: u32) {
        unsafe { sys::lv_obj_add_style(self.raw(), style.raw() as *mut _, selector) }
    }

    /// Set background color
    fn set_style_bg_color(&self, color: Color, selector: u32) {
        unsafe { sys::lv_obj_set_style_bg_color(self.raw(), color.raw(), selector) }
    }

    /// Set background opacity (0-255)
    fn set_style_bg_opa(&self, opa: u8, selector: u32) {
        unsafe { sys::lv_obj_set_style_bg_opa(self.raw(), opa, selector) }
    }

    /// Set text color
    fn set_style_text_color(&self, color: Color, selector: u32) {
        unsafe { sys::lv_obj_set_style_text_color(self.raw(), color.raw(), selector) }
    }

    /// Set border width
    fn set_style_border_width(&self, width: i32, selector: u32) {
        unsafe { sys::lv_obj_set_style_border_width(self.raw(), width, selector) }
    }

    /// Set border color
    fn set_style_border_color(&self, color: Color, selector: u32) {
        unsafe { sys::lv_obj_set_style_border_color(self.raw(), color.raw(), selector) }
    }

    /// Set radius
    fn set_style_radius(&self, radius: i32, selector: u32) {
        unsafe { sys::lv_obj_set_style_radius(self.raw(), radius, selector) }
    }

    /// Set padding
    fn set_style_pad_all(&self, pad: i32, selector: u32) {
        unsafe {
            sys::lv_obj_set_style_pad_top(self.raw(), pad, selector);
            sys::lv_obj_set_style_pad_bottom(self.raw(), pad, selector);
            sys::lv_obj_set_style_pad_left(self.raw(), pad, selector);
            sys::lv_obj_set_style_pad_right(self.raw(), pad, selector);
        }
    }

    /// Add a state flag
    fn add_state(&self, state: State) {
        unsafe { sys::lv_obj_add_state(self.raw(), state.0) }
    }

    /// Remove a state flag
    fn remove_state(&self, state: State) {
        unsafe { sys::lv_obj_remove_state(self.raw(), state.0) }
    }

    /// Check if object has a state
    fn has_state(&self, state: State) -> bool {
        unsafe { sys::lv_obj_has_state(self.raw(), state.0) }
    }

    /// Add an event callback
    ///
    /// # Safety
    /// The callback must remain valid for the lifetime of the object.
    /// User data must remain valid for the lifetime of the object.
    fn add_event_cb<F>(&self, event: crate::Event, callback: F)
    where
        F: FnMut() + 'static,
    {
        // Box the closure and leak it (we can't easily clean this up)
        let boxed: Box<Box<dyn FnMut()>> = Box::new(Box::new(callback));
        let user_data = Box::into_raw(boxed) as *mut c_void;

        unsafe {
            sys::lv_obj_add_event_cb(
                self.raw(),
                Some(event_callback_trampoline),
                event as u32,
                user_data,
            );
        }
    }

    /// Delete the object
    fn delete(&self) {
        unsafe { sys::lv_obj_delete(self.raw()) }
    }

    /// Set object as hidden
    fn set_hidden(&self, hidden: bool) {
        if hidden {
            unsafe { sys::lv_obj_add_flag(self.raw(), sys::LV_OBJ_FLAG_HIDDEN) }
        } else {
            unsafe { sys::lv_obj_remove_flag(self.raw(), sys::LV_OBJ_FLAG_HIDDEN) }
        }
    }

    /// Set object as clickable
    fn set_clickable(&self, clickable: bool) {
        if clickable {
            unsafe { sys::lv_obj_add_flag(self.raw(), sys::LV_OBJ_FLAG_CLICKABLE) }
        } else {
            unsafe { sys::lv_obj_remove_flag(self.raw(), sys::LV_OBJ_FLAG_CLICKABLE) }
        }
    }

    /// Invalidate (redraw) the object
    fn invalidate(&self) {
        unsafe { sys::lv_obj_invalidate(self.raw()) }
    }
}

/// Trampoline function for event callbacks
unsafe extern "C" fn event_callback_trampoline(e: *mut sys::lv_event_t) {
    let user_data = sys::lv_event_get_user_data(e);
    if !user_data.is_null() {
        let callback = &mut *(user_data as *mut Box<dyn FnMut()>);
        callback();
    }
}

/// Generic LVGL object wrapper
///
/// This is the base type for all LVGL objects. Specific widgets like Button,
/// Label, etc. wrap this with additional functionality.
pub struct Obj {
    raw: *mut sys::lv_obj_t,
    /// Prevent Send/Sync - LVGL is not thread-safe
    _marker: PhantomData<*mut ()>,
}

impl Obj {
    /// Create a new object with a parent
    pub fn create(parent: &impl LvglObj) -> Result<Self> {
        unsafe {
            let raw = sys::lv_obj_create(parent.raw());
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

    /// Create from raw pointer (unsafe - caller must ensure validity)
    pub(crate) unsafe fn from_raw(raw: *mut sys::lv_obj_t) -> Self {
        Self {
            raw,
            _marker: PhantomData,
        }
    }

    /// Get child by index
    pub fn get_child(&self, index: i32) -> Option<Obj> {
        unsafe {
            let child = sys::lv_obj_get_child(self.raw, index);
            if child.is_null() {
                None
            } else {
                Some(Obj::from_raw(child))
            }
        }
    }

    /// Get child count
    pub fn get_child_count(&self) -> u32 {
        unsafe { sys::lv_obj_get_child_count(self.raw) }
    }
}

impl LvglObj for Obj {
    fn raw(&self) -> *mut sys::lv_obj_t {
        self.raw
    }
}

// Note: We intentionally don't implement Drop. LVGL manages object lifetimes
// through its internal tree structure. Deleting an object also deletes
// its children. Users should call delete() explicitly if needed.
