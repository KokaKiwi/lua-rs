#[link(
    name = "lua",
    vers = "0.2.0",
    uuid = "4dcb6eef-1da4-4034-a11c-0bd23d9606cf",
    package_id = "liblua"
)];

#[license = "MIT"];

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
