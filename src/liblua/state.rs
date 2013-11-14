use std::libc::*;
use std::cast::transmute;
use std::str::raw;

use ffi;
use types::LuaType;
use status::LuaStatus;
use lua::Lua;

/**
 *  Proxy to Lua state FFI.
 *
 *  The goal is to provide some functions which don't need to be called in unsafe block everytime we want to call them.
 */
pub struct State<'self>
{
    priv L: *ffi::lua_State,
    priv managed: bool,
}

impl<'self> State<'self>
{
    /**
     *  Create a new Lua state, which is managed by this struct.
     *
     *  When this struct is dropped, the state is closed, and is not usable anymore.
     */
    #[fixed_stack_segment]
    pub fn new() -> State
    {
        State {
            L: unsafe {
                ffi::luaL_newstate()
            },
            managed: true,
        }
    }

    /**
     *  Create a new state from an existing Lua state.
     *
     *  Marked as unsafe as we know nothing about this Lua state lifetime.
     */
    pub unsafe fn from_ffi(L: *ffi::lua_State) -> State
    {
        State {
            L: L,
            managed: false,
        }
    }

    /**
     *  Close the Lua state.
     *
     *  Marked as unsafe as the pointer is not usable anymore after this function call.
     */
    #[fixed_stack_segment]
    pub unsafe fn close(&mut self)
    {
        ffi::lua_close(self.L);
    }

    /// Load lua standard libraries.
    #[fixed_stack_segment]
    pub fn load_stdlibs(&self)
    {
        unsafe
        {
            ffi::luaL_openlibs(self.L);
        }
    }

    /// Get type of a stack element at specified index.
    #[fixed_stack_segment]
    pub fn get_type(&self, idx: int) -> LuaType
    {
        let ty = unsafe {
            ffi::lua_type(self.L, idx as c_int)
        };
        LuaType::from_lua(ty)
    }

    /// Pop a stack element specified by his index.
    #[fixed_stack_segment]
    pub fn pop(&self, idx: int)
    {
        unsafe
        {
            ffi::lua_pop(self.L, idx as c_int);
        }
    }

    /// Get global value. Push to the stack.
    #[fixed_stack_segment]
    pub fn get_global(&self, name: &str)
    {
        unsafe
        {
            let c_name = name.to_c_str().unwrap();
            ffi::lua_getglobal(self.L, c_name);
            free(transmute(c_name));
        }
    }

    /// Set global value. Pop from the stack.
    #[fixed_stack_segment]
    pub fn set_global(&self, name: &str)
    {
        unsafe
        {
            let c_name = name.to_c_str().unwrap();
            ffi::lua_setglobal(self.L, c_name);
            free(transmute(c_name));
        }
    }

    /// Get field value. Push to the stack.
    #[fixed_stack_segment]
    pub fn get_field(&self, idx: int, name: &str)
    {
        unsafe
        {
            let c_name = name.to_c_str().unwrap();
            ffi::lua_getfield(self.L, idx as c_int, c_name);
            free(transmute(c_name));
        }
    }

    /// Set field value. Pop from the stack.
    #[fixed_stack_segment]
    pub fn set_field(&self, idx: int, name: &str)
    {
        unsafe
        {
            let c_name = name.to_c_str().unwrap();
            ffi::lua_setfield(self.L, idx as c_int, c_name);
            free(transmute(c_name));
        }
    }

    #[fixed_stack_segment]
    pub fn load_file(&self, filename: &str) -> LuaStatus
    {
        let status = unsafe {
            let filename = filename.to_c_str().unwrap();
            let status = ffi::luaL_loadfile(self.L, filename);
            free(transmute(filename));

            status
        };

        LuaStatus::from_lua(status)
    }

    #[fixed_stack_segment]
    pub fn pcall(&self, nargs: int, nresults: int, errfunc: int) -> LuaStatus
    {
        let status = unsafe {
            ffi::lua_pcall(self.L, nargs as c_int, nresults as c_int, errfunc as c_int)
        };

        LuaStatus::from_lua(status)
    }

    pub fn exec(&self) -> LuaStatus
    {
        self.pcall(0, ffi::LUA_MULTRET as int, 0)
    }

    // Stack functions
    #[fixed_stack_segment]
    pub fn upvalue_index(i: int) -> int
    {
        unsafe
        {
            ffi::lua_upvalueindex(i as c_int) as int
        }
    }

    // Table functions
    #[fixed_stack_segment]
    pub fn new_table(&self, )
    {
        unsafe
        {
            ffi::lua_newtable(self.L);
        }
    }

    #[fixed_stack_segment]
    pub fn create_table(&self, narr: int, nrec: int)
    {
        unsafe
        {
            ffi::lua_createtable(self.L, narr as c_int, nrec as c_int);
        }
    }

    #[fixed_stack_segment]
    pub fn set_table(&self, idx: int)
    {
        unsafe
        {
            ffi::lua_settable(self.L, idx as c_int);
        }
    }

    #[fixed_stack_segment]
    pub fn raw_set(&self, idx: int)
    {
        unsafe
        {
            ffi::lua_rawset(self.L, idx as c_int);
        }
    }

    #[fixed_stack_segment]
    pub fn raw_set_index(&self, idx: int, i: int)
    {
        unsafe
        {
            ffi::lua_rawseti(self.L, idx as c_int, i as c_int);
        }
    }

    // Push functions

    #[fixed_stack_segment]
    pub fn push_nil(&self)
    {
        unsafe
        {
            ffi::lua_pushnil(self.L);
        }
    }

    #[fixed_stack_segment]
    pub fn push_float(&self, n: f64)
    {
        unsafe
        {
            ffi::lua_pushnumber(self.L, n as ffi::lua_Number);
        }
    }

    #[fixed_stack_segment]
    pub fn push_int(&self, n: int)
    {
        unsafe
        {
            ffi::lua_pushinteger(self.L, n as ffi::lua_Integer);
        }
    }

    #[fixed_stack_segment]
    pub fn push_uint(&self, n: uint)
    {
        unsafe
        {
            ffi::lua_pushunsigned(self.L, n as ffi::lua_Unsigned);
        }
    }

    #[fixed_stack_segment]
    pub fn push_str(&self, s: &str)
    {
        unsafe
        {
            let c_str = s.to_c_str().unwrap();
            ffi::lua_pushstring(self.L, c_str);
            free(transmute(c_str));
        }
    }

    #[fixed_stack_segment]
    pub fn push_bool(&self, b: bool)
    {
        unsafe
        {
            let n = if b { 1 } else { 0 };
            ffi::lua_pushboolean(self.L, n);
        }
    }

    #[fixed_stack_segment]
    pub fn push_userdata<T>(&self, p: *T)
    {
        unsafe
        {
            ffi::lua_pushlightuserdata(self.L, transmute(p));
        }
    }

    #[fixed_stack_segment]
    pub fn push_cclosure(&self, f: ffi::lua_CFunction, n: int)
    {
        unsafe
        {
            ffi::lua_pushcclosure(self.L, f, n as c_int);
        }
    }

    #[fixed_stack_segment]
    pub fn push_cfunction(&self, f: ffi::lua_CFunction)
    {
        unsafe
        {
            ffi::lua_pushcfunction(self.L, f);
        }
    }

    pub fn push_function(&self, f: fn(l: &'self Lua) -> int)
    {
        self.push_userdata(f as *());
        self.push_cclosure(_lua_state_closure, 1);
    }

    // Get functions

    #[fixed_stack_segment]
    pub fn get_float(&self, idx: int) -> f64
    {
        unsafe
        {
            ffi::lua_tonumber(self.L, idx as c_int) as f64
        }
    }

    #[fixed_stack_segment]
    pub fn get_int(&self, idx: int) -> int
    {
        unsafe
        {
            ffi::lua_tointeger(self.L, idx as c_int) as int
        }
    }

    #[fixed_stack_segment]
    pub fn get_uint(&self, idx: int) -> uint
    {
        unsafe
        {
            ffi::lua_tounsigned(self.L, idx as c_int) as uint
        }
    }

    #[fixed_stack_segment]
    pub fn get_str(&self, idx: int) -> ~str
    {
        unsafe
        {
            let c_str = ffi::lua_tostring(self.L, idx as c_int);
            raw::from_c_str(c_str)
        }
    }

    #[fixed_stack_segment]
    pub fn get_bool(&self, idx: int) -> bool
    {
        unsafe
        {
            ffi::lua_toboolean(self.L, idx as c_int) != 0
        }
    }

    #[fixed_stack_segment]
    pub fn get_userdata<T>(&self, idx: int) -> *T
    {
        unsafe
        {
            transmute(ffi::lua_touserdata(self.L, idx as c_int))
        }
    }

    #[fixed_stack_segment]
    pub fn get_cfunction(&self, idx: int) -> ffi::lua_CFunction
    {
        unsafe
        {
            ffi::lua_tocfunction(self.L, idx as c_int)
        }
    }

    // Function related functions

    #[fixed_stack_segment]
    pub fn register(&self, name: &str, f: ffi::lua_CFunction)
    {
        unsafe
        {
            let c_name = name.to_c_str().unwrap();
            ffi::lua_register(self.L, c_name, f);
            free(transmute(c_name));
        }
    }
}

extern "C" fn _lua_state_closure(L: *ffi::lua_State) -> c_int
{
    let lua = Lua {
        state: unsafe {
            State::from_ffi(L)
        },
    };
    let f: fn(l: &Lua) -> int = match lua.get_arg(0) {
        Some(p) => p,
        None => fail!("I'm sorry Dave I'm afraid I can't do that."),
    };

    f(&lua) as c_int
}

impl<'self> Drop for State<'self>
{
    fn drop(&mut self)
    {
        if self.managed
        {
            unsafe
            {
                self.close();
            }
        }
    }
}
