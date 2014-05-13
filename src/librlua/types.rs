use std::fmt;

use libc::c_int;

use ffi;

pub enum LuaType {
    LuaNone,
    LuaNil,
    LuaBoolean,
    LuaLightUserData,
    LuaNumber,
    LuaString,
    LuaTable,
    LuaFunction,
    LuaUserData,
    LuaThread,
    LuaUnknown(int),
}

impl LuaType {
    /// Translate Lua type code into an enum.
    pub fn from_lua(ty: c_int) -> LuaType {
        match ty {
            ffi::LUA_TNONE => LuaNone,
            ffi::LUA_TNIL => LuaNil,
            ffi::LUA_TBOOLEAN => LuaBoolean,
            ffi::LUA_TLIGHTUSERDATA => LuaLightUserData,
            ffi::LUA_TNUMBER => LuaNumber,
            ffi::LUA_TSTRING => LuaString,
            ffi::LUA_TTABLE => LuaTable,
            ffi::LUA_TFUNCTION => LuaFunction,
            ffi::LUA_TUSERDATA => LuaUserData,
            ffi::LUA_TTHREAD => LuaThread,
            ty => LuaUnknown(ty as int),
        }
    }
}

impl fmt::Show for LuaType {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let s = match *self {
            LuaNone => "None".to_owned(),
            LuaNil => "Nil".to_owned(),
            LuaBoolean => "Boolean".to_owned(),
            LuaLightUserData => "Light User data".to_owned(),
            LuaNumber => "Number".to_owned(),
            LuaString => "String".to_owned(),
            LuaTable => "Table".to_owned(),
            LuaFunction => "Function".to_owned(),
            LuaUserData => "User data".to_owned(),
            LuaThread => "Thread".to_owned(),
            LuaUnknown(ty) => format!("Unknown({})", ty),
        };
        s.fmt(f)
    }
}
