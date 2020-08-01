#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]

pub(crate) mod ffi {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
}

pub mod grid;

pub use ffi::{initialize, uninitialize};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialize() {
        unsafe {
            super::initialize();
            super::uninitialize();
        }
    }
}
