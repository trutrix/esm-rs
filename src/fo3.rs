//! Fallout 3 definitions.

pub mod timestamp;
pub use timestamp::Timestamp;

pub mod version;
pub use version::VersionControlInfo;

pub use fourcc::FourCC;

include!(concat!(env!("OUT_DIR"), "/fo3.rs"));
