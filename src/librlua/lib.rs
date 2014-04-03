#![crate_id = "rlua#0.3.0"]
#![crate_type = "lib"]
#![license = "MIT"]

#![feature(globs)]
#![feature(macro_rules)]

extern crate collections;

pub use lua::Lua;

#[allow(uppercase_variables, non_camel_case_types)]
pub mod ffi;

pub mod lua;
pub mod state;

pub mod traits;
pub mod types;
pub mod status;

pub mod macros;
