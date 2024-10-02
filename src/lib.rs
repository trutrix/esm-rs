//! Packed struct bindings for Elder Scrolls Mod format.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub use fourcc::FourCC;

include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
