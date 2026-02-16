//! Raw FFI bindings to LVGL
//! 
//! This crate provides unsafe bindings to LVGL. Use the parent `lvgl` crate
//! for safe wrappers.

#![cfg_attr(not(feature = "simulator"), no_std)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
#![allow(dead_code)]
#![allow(clippy::all)]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
