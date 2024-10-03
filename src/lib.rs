//! Packed struct bindings for Elder Scrolls Mod format.

#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub mod fo3 {
    pub mod timestamp;
    pub use timestamp::Timestamp;

    pub mod version;
    pub use version::VersionControlInfo;
}

pub use fourcc::FourCC;
pub use fo3::*;

include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
