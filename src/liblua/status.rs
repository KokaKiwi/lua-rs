use std::libc::*;

use ffi;

pub enum LuaStatus
{
    LuaOk,
    LuaErr(LuaError),
}

pub enum LuaError
{
    RuntimeError,
    MemoryError,
    GCError,
    SyntaxError,
    FileError,
    UnknownError,
}

impl LuaStatus {
    pub fn from_lua(code: c_int) -> LuaStatus
    {
        match code
        {
            ffi::LUA_OK => LuaOk,
            _ => LuaErr(LuaError::from_lua(code)),
        }
    }
}

impl LuaError
{
    pub fn from_lua(code: c_int) -> LuaError
    {
        match code
        {
            ffi::LUA_OK => fail!("Not an error."),
            ffi::LUA_ERRRUN => RuntimeError,
            ffi::LUA_ERRMEM => MemoryError,
            ffi::LUA_ERRGCMM => GCError,
            ffi::LUA_ERRSYNTAX => SyntaxError,
            ffi::LUA_ERRFILE => FileError,
            _ => UnknownError,
        }
    }
}

impl ToStr for LuaError
{
    fn to_str(&self) -> ~str
    {
        match *self
        {
            _ => ~""
        }
    }
}