//! Fallout 3 definitions.

pub mod timestamp;
pub use timestamp::Timestamp;

pub mod version;
pub use version::VersionControlInfo;

pub use fourcc::FourCC;

pub mod label;
pub use label::*;

include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
