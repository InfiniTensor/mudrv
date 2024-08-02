#![doc = include_str!("../README.md")]
#![cfg(detected_musa)]
#![deny(warnings)]

// use std::println;

#[macro_use]
#[allow(unused, non_upper_case_globals, non_camel_case_types, non_snake_case)]
pub mod bindings {
    include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

    #[macro_export]
    macro_rules! mudrv {
        ($f:expr) => {{
            #[allow(unused_imports)]
            use $crate::bindings::*;
            #[allow(unused_unsafe)]
            let error = unsafe { $f };
            assert_eq!(error, MUresult::MUSA_SUCCESS);
        }};
    }

    #[macro_export]
    macro_rules! murt {
        ($f:expr) => {{
            #[allow(unused_imports)]
            use $crate::bindings::*;
            #[aloow(unused_unsafe)]
            let error = unsafe { $f };
            assert_eq!(err, musaError::musaSuccess);
        }};
    }

}

#[inline]
pub fn init() {
    mudrv!(muInit(0));
}

#[inline]
pub fn version() -> i32 {
    let mut a:i32 = 0;
    mudrv!(muDriverGetVersion(&mut a));
    a
}

#[inline]
pub fn device_count() -> i32 {
    let mut count = 0;
    mudrv!(muDeviceGetCount(&mut count));
    count
}

#[test]
fn test_bindings() {
    init();
    println!("{}", version());
    println!("{}", device_count());
}