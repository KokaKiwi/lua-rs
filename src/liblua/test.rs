#[feature(globs)];
#[feature(macro_rules)];

extern mod lua;
extern mod extra;

use std::hashmap::HashMap;

use lua::Lua;
use lua::state::State;
use lua::status::*;

#[path = "../lua/macros.rs"]
mod macros;

#[test]
fn test_file_loading()
{
    let state = State::new();
    match state.load_file("sample.lua")
    {
        LuaOk => {}
        LuaErr(_) => fail!("Failed to load file: sample.lua"),
    }

    match state.load_file("inexistant.lua")
    {
        LuaOk => fail!("Successed to load file: inexistant.lua, but it shouldn't as the file shouldn't exists."),
        LuaErr(_) => {},
    }
}

#[test]
fn test_str_loading()
{
    let state = State::new();
    match state.load_str("a = 2")
    {
        LuaOk => {}
        LuaErr(_) => fail!(),
    }

    match state.load_str("fail fail fail fail")
    {
        LuaOk => fail!(),
        LuaErr(_) => {}
    }
}

#[test]
fn test_export_struct()
{
    let lua = Lua::new();
    lua_struct!(
        Player
        {
            name: ~str
        }
    )

    let player = Player::Player {
        name: ~"Player"
    };
    lua.set_global("player", player.clone());

    lua.exec_str("
        name = player.name
    ");

    let name: ~str = match lua.get_global("name") {
        Some(s) => s,
        None => fail!(),
    };

    assert_eq!(name, player.name);
}

#[test]
fn test_export_fn()
{
    let lua = Lua::new();

    fn sample(s: ~str) -> ~str
    {
        "[" + s + "]"
    }
    lua_fn_export!(lua_sample: sample(name: ~str) -> ~str);
    lua.set_global("sample", lua_sample);

    lua.exec_str("
        result = sample('sample')
    ");

    let result: ~str = match lua.get_global("result") {
        Some(s) => s,
        None => fail!(),
    };

    assert_eq!(result, ~"[sample]");
}

#[test]
fn test_import_fn()
{
    let lua = Lua::new();
    lua.exec_str("
        function sample(s)
            return '[' .. s .. ']'
        end
    ");

    let sample = lua_fn_import!(lua: sample|s: ~str| -> ~str);
    let s = ~"sample";

    assert_eq!(sample(s.clone()), "[" + s + "]");
}

#[test]
fn test_array_len()
{
    let lua = Lua::new();
    lua.state.load_stdlibs();
    lua.set_global("arr", &[1, 2, 3]);

    lua.exec_str("
        result = 0
        for _ in pairs(arr) do
            result = result + 1
        end
    ");

    let result: int = match lua.get_global("result") {
        Some(n) => n,
        None => fail!(),
    };

    assert_eq!(result, 3);
}

#[test]
fn test_array_eq()
{
    let lua = Lua::new();

    lua.exec_str("
        result = {1, 2, 3}
    ");

    let result: ~[int] = match lua.get_global("result") {
        Some(a) => a,
        None => fail!(),
    };

    assert_eq!(result, ~[1, 2, 3]);
}

#[test]
fn test_hashmap_to_lua()
{
    let lua = Lua::new();

    let mut map: HashMap<~str, ~str> = HashMap::new();
    map.insert(~"name", ~"Test");

    lua.set_global("map", map);

    lua.exec_str("
        result = map.name
    ");

    let result: ~str = match lua.get_global("result") {
        Some(s) => s,
        None => fail!(),
    };

    assert_eq!(result, ~"Test");
}

#[test]
fn test_hashmap_from_lua()
{
    let lua = Lua::new();

    lua.exec_str("
        result = {}
        result.name = 'Test'
    ");

    let result: HashMap<~str, ~str> = match lua.get_global("result") {
        Some(m) => m,
        None => fail!(),
    };

    assert_eq!(result.get(&~"name"), &~"Test");
}

#[test]
fn test_module()
{
    let lua = Lua::new();

    fn test_mul(n: int, m: int) -> int
    {
        n * m
    }
    lua_fn_export!(lua_mul: test_mul(n: int, m: int) -> int);

    let ops = ~[
        ("mul", lua_mul),
    ];

    let mut module = HashMap::new();
    for &(name, method) in ops.iter()
    {
        module.insert(name, method);
    }

    lua.set_global("test", module);

    lua.exec_str("
        result = test.mul(2, 3)
    ");

    let result: int = match lua.get_global("result") {
        Some(n) => n,
        None => fail!(),
    };

    assert_eq!(result, 6);
}

#[bench]
fn bench_load_str(b: &mut extra::test::BenchHarness)
{
    let state = State::new();
    do b.iter
    {
        state.load_str("a = 2");
    }
}
