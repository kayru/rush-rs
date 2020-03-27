#![allow(non_camel_case_types)] // todo: switch to rust default style in RushC.h 

#[macro_use]
extern crate const_cstr;

include!("rush_ffi.rs");

const_cstr! {
    DEFAULT_APP_NAME = "RushApp";
}

impl rush_app_config {
    pub fn new() -> rush_app_config {
        rush_app_config {
            name: DEFAULT_APP_NAME.as_ptr(),
            vsync: 1,
            width: 640,
            height: 480,
            ..Default::default()
        }
    }
}
