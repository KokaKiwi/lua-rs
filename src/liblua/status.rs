use std::fmt;

use libc::c_int;

use ffi;

#[deriving(Show, Eq)]
pub enum LuaStatus {
    LuaOk,
    LuaErr(LuaError),
}

#[deriving(Eq)]
pub enum LuaError {
    RuntimeError,
    MemoryError,
    GCError,
    SyntaxError,
    FileError,
    UnknownError,
}

impl LuaStatus {
    pub fn from_lua(code: c_int) -> LuaStatus {
        match code
        {
            ffi::LUA_OK => LuaOk,
            _ => LuaErr(LuaError::from_lua(code)),
        }
    }
}

impl LuaError {
    pub fn from_lua(code: c_int) -> LuaError {
        match code {
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

impl fmt::Show for LuaError {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            RuntimeError => "Runtime error".to_owned(),
            MemoryError => "Memory eror".to_owned(),
            GCError => "GC error".to_owned(),
            SyntaxError => "Syntax error".to_owned(),
            FileError => "File error".to_owned(),
            _ => "".to_owned(),
        };
        s.fmt(f)
    }
}
