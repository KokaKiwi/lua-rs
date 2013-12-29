#[license = "MIT"];

#[crate_id = "github.com/KokaKiwi/lua#0.2.1"];
#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

pub use lua::Lua;

pub mod ffi;

pub mod lua;
pub mod state;

pub mod traits;
pub mod types;
pub mod status;
