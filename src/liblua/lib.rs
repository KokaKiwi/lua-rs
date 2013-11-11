#[link(
    name = "lua",
    vers = "0.1.0",
    uuid = "4dcb6eef-1da4-4034-a11c-0bd23d9606cf",
    package_id = "liblua"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "lib"];

#[feature(globs)];
#[feature(macro_rules)];

mod ffi;
pub mod state;
pub mod types;
