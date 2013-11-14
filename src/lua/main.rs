#[link(
    name = "lua",
    vers = "0.1.0",
    uuid = "2221d732-c740-464d-a470-292ac42d6c52"
)];

#[author = "KokaKiwi <kokakiwi@kokakiwi.net>"];
#[license = "MIT"];

#[crate_type = "bin"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod lua;

use std::os;

use lua::Lua;

mod macros;

lua_struct!(
    Totoro
    {
        name: ~str
    }
)

fn main()
{
    let l = Lua::new();
    l.state.load_stdlibs();

    let args = os::args();
    let filename = args[1].as_slice();

    println!("Load result: {:?}", l.exec_file(filename));

    let tot = Totoro::Totoro {
        name: ~"Totoro",
    };

    let get_name = lua_fn_import!(l: get_name| tot: Totoro::Totoro | -> ~str);

    fn transform(name: ~str) -> ~str
    {
        "[" + name + "]"
    }
    lua_fn_export!(lua_transform: transform(name: ~str) -> ~str);
    l.set_global("transform", lua_transform);
    l.set_global("toto", "toto");

    let new_name = get_name(tot);
    println!("New name: {}", new_name);
}
