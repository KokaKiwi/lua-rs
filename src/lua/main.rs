#[license = "MIT"];

#[crate_type = "bin"];
#[crate_id = "lua#0.1.0"];

#[feature(globs)];
#[feature(macro_rules)];

extern mod lua;

use std::os;
use std::hashmap::HashMap;

use lua::Lua;
use lua::status::*;

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

    match l.exec_file(filename)
    {
        LuaErr(e) => {
            let msg: ~str = match l.pop() {
                Some(s) => s,
                None => fail!(),
            };
            println!("{:?}: {:s}", e, msg);
            return;
        }
        _ => {}
    }

    // Test 1
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

    // Test 2
    let mut map: HashMap<~str, int> = HashMap::new();

    for i in range(0, 10)
    {
        map.insert(format!("key{}", i), i);
    }

    l.set_global("t_map", map);

    let test_map = lua_fn_import!(l: test_map||);
    test_map();

    // Test 3
    l.set_global("t_arr", &[1, 2, 3]);

    let test_arr = lua_fn_import!(l: test_arr||);
    test_arr();

    // Test 4
    let test_fn = lua_fn_import!(l: test_fn|a: &[int]| -> int);
    println!("test_fn() -> {}", test_fn(&[1, 2, 3, 4]));
}
