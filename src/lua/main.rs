#[link(
    name = "lua",
    vers = "0.1.0",
    uuid = "2221d732-c740-464d-a470-292ac42d6c52"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "bin"];

#[feature(globs)];

extern mod lua;

use std::os;

use lua::state::State;
use lua::types::*;

fn main()
{
    let state = State::new();
    state.load_stdlibs();

    let args = os::args();
    let filename = args[1].as_slice();
    match state.load_file(filename)
    {
        LuaOk => {
            match state.exec()
            {
                LuaErr(e) => fail!("Error: {}", e),
                _ => {}
            }
        }
        LuaErr(e) => fail!("Error: {}", e),
    }
}
