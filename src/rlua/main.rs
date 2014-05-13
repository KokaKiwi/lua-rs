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
        let program = str::from_utf8(program.as_slice()).unwrap();

        lua.exec_str(program)
    } else {
        lua.exec_file(args.get(1).as_slice())
    };

    match status {
        rlua::status::LuaOk => {}
        rlua::status::LuaErr(e) => {
            let msg: ~str = lua.get(-1).unwrap();

            fail!("Lua {}: {}", e, msg);
        }
    }
}
