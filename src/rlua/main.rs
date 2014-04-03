#![crate_id = "rlua#0.3.0"]
#![crate_type = "bin"]
#![license = "MIT"]

#![feature(globs)]

extern crate rlua;

use std::os;
use std::io;
use std::str;

use rlua::Lua;

fn main()
{
    let args = os::args();

    let lua = Lua::new();
    lua.state.load_stdlibs();

    let status = if args.len() == 1 {
        let program = io::stdin().read_to_end().unwrap();
        let program = str::from_utf8(program).unwrap();

        lua.exec_str(program)
    } else {
        lua.exec_file(args[1])
    };

    match status {
        rlua::status::LuaOk => {}
        rlua::status::LuaErr(e) => {
            fail!("Lua error: {}", e);
        }
    }
}
