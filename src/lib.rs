#![doc = include_str!("../README.md")]

pub use sys::*;
#[cfg(not(feature = "bindings"))]
mod sys {
    /// if bindings feature is not enabled, we use manually maintained bindings
    /// for native handles stuff.
    mod manual;
    /// if bindings is not enabled, we use pre-generated bindings
    mod pregenerated;
    pub use manual::*;
    pub use pregenerated::*;
}
#[cfg(feature = "bindings")]
mod sys {
    #![allow(
        unused,
        non_upper_case_globals,
        non_camel_case_types,
        non_snake_case,
        improper_ctypes,
        reason = "This is a bindgen generated file from C headers and can't be fixed manually"
    )]
    // if bindings is enabled, we use bindgen to generate bindings and include them here
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}
