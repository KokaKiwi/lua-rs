#[feature(globs)];

extern mod lua;
extern mod extra;

use lua::state::State;
use lua::status::*;

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

#[bench]
fn bench_load_str(b: &mut extra::test::BenchHarness)
{
    let state = State::new();
    do b.iter
    {
        state.load_str("a = 2");
    }
}
