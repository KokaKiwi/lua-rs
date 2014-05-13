use std::mem::transmute;
use std::hash::Hash;
use collections::hashmap::HashMap;

use libc::c_void;

use state::State;
use types::*;
use lua::Lua;

/**
 *  A generic trait for converting a Lua value to a Rust value.
 *
 *  Return None when the types didn't corresponds.
 */
pub trait FromLua {
    fn from_lua(state: &State, idx: int) -> Option<Self>;
}

/// A generic trait for converting a Rust value to a Lua value.
pub trait ToLua {
    fn to_lua(&self, state: &State);
}

// float
impl FromLua for f64 {
    fn from_lua(state: &State, idx: int) -> Option<f64> {
        match state.get_type(idx) {
            LuaNumber => Some(state.get_float(idx)),
            _ => None,
        }
    }
}

impl ToLua for f64 {
    fn to_lua(&self, state: &State) {
        state.push_float(*self);
    }
}

// int
impl FromLua for int {
    fn from_lua(state: &State, idx: int) -> Option<int> {
        match state.get_type(idx) {
            LuaNumber => Some(state.get_int(idx)),
            _ => None,
        }
    }
}

impl ToLua for int {
    fn to_lua(&self, state: &State) {
        state.push_int(*self);
    }
}

// uint
impl FromLua for uint {
    fn from_lua(state: &State, idx: int) -> Option<uint> {
        match state.get_type(idx) {
            LuaNumber => Some(state.get_uint(idx)),
            _ => None,
        }
    }
}

impl ToLua for uint {
    fn to_lua(&self, state: &State) {
        state.push_uint(*self);
    }
}

// &str
impl<'a> ToLua for &'a str {
    fn to_lua(&self, state: &State) {
        state.push_str(*self);
    }
}

// ~str
impl FromLua for ~str {
    fn from_lua(state: &State, idx: int) -> Option<~str> {
        match state.get_type(idx) {
            LuaString => Some(state.get_str(idx)),
            _ => None,
        }
    }
}

impl ToLua for ~str {
    fn to_lua(&self, state: &State) {
        state.push_str(*self);
    }
}

// bool
impl FromLua for bool {
    fn from_lua(state: &State, idx: int) -> Option<bool> {
        match state.get_type(idx) {
            LuaBoolean => Some(state.get_bool(idx)),
            _ => None,
        }
    }
}

impl ToLua for bool {
    fn to_lua(&self, state: &State) {
        state.push_bool(*self);
    }
}

// Pointer
impl<T> FromLua for *T {
    fn from_lua(state: &State, idx: int) -> Option<*T> {
        match state.get_type(idx) {
            LuaUserData | LuaLightUserData => Some(state.get_userdata(idx)),
            _ => None,
        }
    }
}

impl<T> ToLua for *T {
    fn to_lua(&self, state: &State) {
        state.push_userdata(*self);
    }
}

// Function
impl FromLua for fn(l: &Lua) -> int {
    fn from_lua(state: &State, idx: int) -> Option<fn(l: &Lua) -> int> {
        match state.get_type(idx) {
            LuaUserData | LuaLightUserData => {
                let ptr: fn(&Lua) -> int = unsafe {
                    transmute(state.get_userdata::<*c_void>(idx))
                };

                Some(ptr)
            }
            _ => None,
        }
    }
}

impl ToLua for fn(l: &Lua) -> int {
    fn to_lua(&self, state: &State) {
        state.push_function(*self);
    }
}

// HashMap
impl<K: ToLua + Hash + TotalEq, V: ToLua> ToLua for HashMap<K, V> {
    fn to_lua(&self, state: &State) {
        state.new_table();

        for (ref key, ref val) in self.iter() {
            key.to_lua(state);
            val.to_lua(state);
            state.raw_set(-3);
        }
    }
}

impl<K: FromLua + Hash + TotalEq, V: FromLua> FromLua for HashMap<K, V> {
    fn from_lua(state: &State, idx: int) -> Option<HashMap<K, V>> {
        let mut map: HashMap<K, V> = HashMap::new();

        state.push_nil();
        while state.next(idx - 1) {
            let key: K = match FromLua::from_lua(state, -2) {
                Some(k) => k,
                None => {
                    return None;
                }
            };
            let val: V = match FromLua::from_lua(state, -1) {
                Some(v) => v,
                None => {
                    return None;
                }
            };
            map.insert(key, val);

            state.pop(1);
        }

        Some(map)
    }
}

// Array
impl<'a, T: ToLua> ToLua for &'a [T] {
    fn to_lua(&self, state: &State) {
        state.new_table();

        for (i, ref val) in self.iter().enumerate() {
            state.push_uint(i + 1);
            val.to_lua(state);
            state.raw_set(-3);
        }
    }
}

impl<T: FromLua> FromLua for Vec<T> {
    fn from_lua(state: &State, idx: int) -> Option<Vec<T>> {
        let mut v = Vec::new();
        let length = state.len(idx);

        let mut i = 1;
        while i <= length {
            state.push_int(i);

            let val: T = match FromLua::from_lua(state, -1) {
                Some(val) => val,
                None => {
                    return None;
                }
            };
            v.push(val);

            i += 1;
        }

        Some(v)
    }
}
