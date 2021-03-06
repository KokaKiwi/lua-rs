#![crate_id = "lua#0.3.0"]
#![crate_type = "rlib"]
#![crate_type = "dylib"]
#![license = "MIT"]

#![feature(globs)]
#![feature(macro_rules)]

extern crate collections;
extern crate libc;

pub use lua::Lua;

#[allow(uppercase_variables, non_camel_case_types)]
pub mod ffi;

pub mod lua;
pub mod state;

pub mod traits;
pub mod types;
pub mod status;

pub mod macros;
