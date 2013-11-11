use std::libc::*;
use std::cast::transmute;

use ffi;
use types::*;

pub struct State
{
    L: *ffi::lua_State,
}

impl State
{
    #[fixed_stack_segment]
    pub fn new() -> State
    {
        unsafe
        {
            State
            {
                L: ffi::luaL_newstate(),
            }
        }
    }

    #[fixed_stack_segment]
    pub fn close(&mut self)
    {
        unsafe
        {
            ffi::lua_close(self.L);
        }
    }

    #[fixed_stack_segment]
    pub fn load_stdlibs(&self)
    {
        unsafe
        {
            ffi::luaL_openlibs(self.L);
        }
    }

    // Push functions (Rust -> stack)
    #[fixed_stack_segment]
    pub fn push_bool(&self, b: bool)
    {
        unsafe
        {
            ffi::lua_pushboolean(self.L, if b { 0 } else { 1 })
        }
    }

    #[fixed_stack_segment]
    pub fn push_int(&self, i: int)
    {
        unsafe
        {
            ffi::lua_pushinteger(self.L, i as ffi::lua_Integer)
        }
    }

    // Load functions
    #[fixed_stack_segment]
    pub fn load_file(&self, filename: &str) -> LuaStatus
    {
        unsafe
        {
            let filename = filename.to_c_str().unwrap();
            let ret = LuaStatus::from_lua(ffi::luaL_loadfile(self.L, filename));
            free(transmute(filename));

            ret
        }
    }

    #[fixed_stack_segment]
    pub fn load_str(&self, src: &str) -> LuaStatus
    {
        unsafe
        {
            let src = src.to_c_str().unwrap();
            let ret = LuaStatus::from_lua(ffi::luaL_loadstring(self.L, src));
            free(transmute(src));

            ret
        }
    }

    // Call functions
    pub fn pcall(&self, nargs: int, nresults: int, errfunc: int) -> LuaStatus
    {
        unsafe
        {
            LuaStatus::from_lua(ffi::lua_pcall(self.L, nargs as c_int, nresults as c_int, errfunc as c_int))
        }
    }

    pub fn exec(&self) -> LuaStatus
    {
        self.pcall(0, ffi::LUA_MULTRET as int, 0)
    }
}

impl Drop for State
{
    fn drop(&mut self)
    {
        self.close();
    }
}
