use std::cast::transmute;
use std::str::raw;

use libc::c_int;

use ffi;
use types::LuaType;
use status::LuaStatus;
use lua::Lua;

/**
 *  Proxy to Lua state FFI.
 *
 *  The goal is to provide some functions which don't need to be called in unsafe block everytime we want to call them.
 */
#[allow(uppercase_variables)]
pub struct State<'a> {
    raw: *ffi::lua_State,
    managed: bool,
}

impl<'a> State<'a> {
    /**
     *  Create a new Lua state, which is managed by this struct.
     *
     *  When this struct is dropped, the state is closed, and is not usable anymore.
     */
    pub fn new() -> State {
        let raw = unsafe {
            ffi::luaL_newstate()
        };

        match raw.is_null() {
            true => fail!("Can't create new Lua state!"),
            false => State {
                raw: raw,
                managed: true,
            },
        }
    }

    /**
     *  Create a new state from an existing Lua state.
     *
     *  Marked as unsafe as we know nothing about this Lua state lifetime.
     */
    pub unsafe fn from_ffi(raw: *ffi::lua_State) -> State {
        State {
            raw: raw,
            managed: false,
        }
    }

    /**
     *  Close the Lua state.
     *
     *  Marked as unsafe as the pointer is not usable anymore after this function call.
     */
    pub unsafe fn close(&mut self) {
        ffi::lua_close(self.raw);
    }

    /// Load lua standard libraries.
    pub fn load_stdlibs(&self) {
        unsafe {
            ffi::luaL_openlibs(self.raw);
        }
    }

    /// Get type of a stack element at specified index.
    pub fn get_type(&self, idx: int) -> LuaType {
        let ty = unsafe {
            ffi::lua_type(self.raw, idx as c_int)
        };
        LuaType::from_lua(ty)
    }

    /// Create a new Lua thread.
    pub fn new_thread(&self) -> State {
        unsafe {
            State::from_ffi(ffi::lua_newthread(self.raw))
        }
    }

    /// Get Lua version.
    pub fn version(&self) -> int {
        unsafe {
            ffi::lua_version(self.raw) as int
        }
    }

    /// Pop a stack element specified by his index.
    pub fn pop(&self, idx: int) {
        unsafe {
            ffi::lua_pop(self.raw, idx as c_int);
        }
    }

    /// Get global value. Push to the stack.
    pub fn get_global(&self, name: &str) {
        name.with_c_str(|name| unsafe {
            ffi::lua_getglobal(self.raw, name);
        });
    }

    /// Set global value. Pop from the stack.
    pub fn set_global(&self, name: &str) {
        name.with_c_str(|name| unsafe {
            ffi::lua_setglobal(self.raw, name);
        });
    }

    /// Get field value. Push to the stack.
    pub fn get_field(&self, idx: int, name: &str) {
        name.with_c_str(|name| unsafe {
            ffi::lua_getfield(self.raw, idx as c_int, name);
        });
    }

    /// Set field value. Pop from the stack.
    pub fn set_field(&self, idx: int, name: &str) {
        name.with_c_str(|name| unsafe {
            ffi::lua_setfield(self.raw, idx as c_int, name);
        });
    }

    pub fn load_file(&self, filename: &str) -> LuaStatus {
        let status = filename.with_c_str(|filename| unsafe {
            ffi::luaL_loadfile(self.raw, filename)
        });

        LuaStatus::from_lua(status)
    }

    pub fn load_str(&self, source: &str) -> LuaStatus {
        let status = source.with_c_str(|source| unsafe {
            ffi::luaL_loadstring(self.raw, source)
        });

        LuaStatus::from_lua(status)
    }

    pub fn pcall(&self, nargs: int, nresults: int, errfunc: int) -> LuaStatus {
        let status = unsafe {
            ffi::lua_pcall(self.raw, nargs as c_int, nresults as c_int, errfunc as c_int)
        };

        LuaStatus::from_lua(status)
    }

    pub fn exec(&self) -> LuaStatus {
        self.pcall(0, ffi::LUA_MULTRET as int, 0)
    }

    // Stack functions
    pub fn upvalue_index(i: int) -> int {
        unsafe {
            ffi::lua_upvalueindex(i as c_int) as int
        }
    }

    /// Get index of the stack's top.
    pub fn get_top(&self) -> int {
        unsafe {
            ffi::lua_gettop(self.raw) as int
        }
    }

    /// Set index of the stack's top.
    pub fn set_top(&self, idx: int) {
        unsafe {
            ffi::lua_settop(self.raw, idx as c_int);
        }
    }

    /// Removes the element at the given valid index.
    pub fn remove(&self, idx: int) {
        unsafe {
            ffi::lua_remove(self.raw, idx as c_int);
        }
    }

    // Table functions
    pub fn new_table(&self) {
        unsafe {
            ffi::lua_newtable(self.raw);
        }
    }

    pub fn create_table(&self, narr: int, nrec: int) {
        unsafe {
            ffi::lua_createtable(self.raw, narr as c_int, nrec as c_int);
        }
    }

    pub fn set_table(&self, idx: int) {
        unsafe {
            ffi::lua_settable(self.raw, idx as c_int);
        }
    }

    pub fn raw_set(&self, idx: int) {
        unsafe {
            ffi::lua_rawset(self.raw, idx as c_int);
        }
    }

    pub fn raw_set_index(&self, idx: int, i: int) {
        unsafe {
            ffi::lua_rawseti(self.raw, idx as c_int, i as c_int);
        }
    }

    pub fn next(&self, idx: int) -> bool {
        unsafe {
            ffi::lua_next(self.raw, idx as c_int) != 0
        }
    }

    pub fn len(&self, idx: int) -> int {
        unsafe {
            ffi::lua_len(self.raw, idx as c_int);
        }

        let len = self.get_int(-1);
        self.pop(1);

        len
    }

    pub fn get_metatable(&self, idx: int) -> bool {
        unsafe {
            ffi::lua_getmetatable(self.raw, idx as c_int) != 0
        }
    }

    pub fn set_metatable(&self, idx: int) {
        unsafe {
            ffi::lua_setmetatable(self.raw, idx as c_int);
        }
    }

    // Push functions

    pub fn push_nil(&self) {
        unsafe {
            ffi::lua_pushnil(self.raw);
        }
    }

    pub fn push_float(&self, n: f64) {
        unsafe {
            ffi::lua_pushnumber(self.raw, n as ffi::lua_Number);
        }
    }

    pub fn push_int(&self, n: int) {
        unsafe {
            ffi::lua_pushinteger(self.raw, n as ffi::lua_Integer);
        }
    }

    pub fn push_uint(&self, n: uint) {
        unsafe {
            ffi::lua_pushunsigned(self.raw, n as ffi::lua_Unsigned);
        }
    }

    pub fn push_str(&self, s: &str) {
        s.with_c_str(|s| unsafe {
            ffi::lua_pushstring(self.raw, s);
        });
    }

    pub fn push_bool(&self, b: bool) {
        unsafe {
            let n = if b { 1 } else { 0 };
            ffi::lua_pushboolean(self.raw, n);
        }
    }

    pub fn push_userdata<T>(&self, p: *T) {
        unsafe {
            ffi::lua_pushlightuserdata(self.raw, transmute(p));
        }
    }

    pub fn push_cclosure(&self, f: ffi::lua_CFunction, n: int) {
        unsafe {
            ffi::lua_pushcclosure(self.raw, f, n as c_int);
        }
    }

    pub fn push_cfunction(&self, f: ffi::lua_CFunction) {
        unsafe {
            ffi::lua_pushcfunction(self.raw, f);
        }
    }

    pub fn push_function(&self, f: fn(l: &'a Lua) -> int) {
        self.push_userdata(f as *());
        self.push_cclosure(_lua_state_closure, 1);
    }

    // Get functions

    pub fn get_float(&self, idx: int) -> f64 {
        unsafe {
            ffi::lua_tonumber(self.raw, idx as c_int) as f64
        }
    }

    pub fn get_int(&self, idx: int) -> int {
        unsafe {
            ffi::lua_tointeger(self.raw, idx as c_int) as int
        }
    }

    pub fn get_uint(&self, idx: int) -> uint {
        unsafe {
            ffi::lua_tounsigned(self.raw, idx as c_int) as uint
        }
    }

    pub fn get_str(&self, idx: int) -> ~str {
        unsafe {
            let c_str = ffi::lua_tostring(self.raw, idx as c_int);
            raw::from_c_str(c_str)
        }
    }

    pub fn get_bool(&self, idx: int) -> bool {
        unsafe {
            ffi::lua_toboolean(self.raw, idx as c_int) != 0
        }
    }

    pub fn get_userdata<T>(&self, idx: int) -> *T {
        unsafe {
            transmute(ffi::lua_touserdata(self.raw, idx as c_int))
        }
    }

    pub fn get_cfunction(&self, idx: int) -> ffi::lua_CFunction {
        unsafe {
            ffi::lua_tocfunction(self.raw, idx as c_int)
        }
    }

    // Function related functions
    pub fn register(&self, name: &str, f: ffi::lua_CFunction) {
        name.with_c_str(|name| unsafe {
            ffi::lua_register(self.raw, name, f);
        });
    }

    // Misc functions
    pub fn error(&self) -> int {
        unsafe {
            ffi::lua_error(self.raw) as int
        }
    }
}

extern "C" fn _lua_state_closure(raw: *ffi::lua_State) -> c_int {
    let lua = Lua {
        state: unsafe {
            State::from_ffi(raw)
        },
    };
    let f: fn(l: &Lua) -> int = match lua.get_arg(0) {
        Some(p) => p,
        None => fail!("I'm sorry Dave I'm afraid I can't do that."),
    };

    f(&lua) as c_int
}

impl<'a> Drop for State<'a> {
    fn drop(&mut self) {
        if self.managed {
            unsafe {
                self.close();
            }
        }
    }
}
