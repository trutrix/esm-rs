//! Fallout 3 definitions.

pub mod version_control;
pub use version_control::*;

pub use fourcc::FourCC;

pub mod group_label;
pub use group_label::*;

include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
