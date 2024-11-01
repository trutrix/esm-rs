//! Fallout 3 definitions.

pub mod record_flags;
pub use record_flags::*;


pub mod version_control;
pub use version_control::*;

pub use fourcc::FourCC;

pub mod group_label;
pub use group_label::*;



pub mod headers;
pub use headers::*;


include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
