//! Bethesda Softworks Archive definitions.

pub use fourcc::FourCC;

include!(concat!(env!("OUT_DIR"), "/bsa.rs"));
