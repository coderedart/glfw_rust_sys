#[cfg(not(feature = "bindings"))]
mod sys;

#[cfg(feature = "bindings")]
#[allow(unused)]
#[allow(non_upper_case_globals)]
#[allow(non_camel_case_types)]
#[allow(non_snake_case)]
#[allow(improper_ctypes)]
mod sys {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub use sys::*;
