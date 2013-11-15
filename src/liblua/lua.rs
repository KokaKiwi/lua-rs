use state::State;
use traits::{FromLua, ToLua};
use status::{LuaStatus, LuaOk};

/**
 *  Proxy to Lua general functions.
 *
 *  The goal is to provide unified API in order to interact with Lua in a transparent way.
 */
pub struct Lua<'self>
{
    state: State<'self>,
}

impl<'self> Lua<'self>
{
    pub fn new() -> Lua
    {
        Lua {
            state: State::new(),
        }
    }

    /// Push a Rust value to the Lua stack.
    pub fn push<T: ToLua>(&self, val: T)
    {
        val.to_lua(&self.state);
    }

    /// Get a Rust value from the Lua stack at specified index.
    pub fn get<T: FromLua>(&self, idx: int) -> Option<T>
    {
        FromLua::from_lua(&self.state, idx)
    }

    /// Get a Rust value from call arguments.
    pub fn get_arg<T: FromLua>(&self, idx: int) -> Option<T>
    {
        // Incremented by one to match Lua indexing.
        self.get(State::upvalue_index(idx + 1))
    }

    /// Pop a Rust value from the Lua stack.
    pub fn pop<T: FromLua>(&self) -> Option<T>
    {
        let v = self.peek();
        self.state.pop(1);
        v
    }

    /// Peek a Rust value from the Lua stack.
    pub fn peek<T: FromLua>(&self) -> Option<T>
    {
        self.get(-1)
    }

    /// Get a Rust value from the Lua globals.
    pub fn get_global<T: FromLua>(&self, name: &str) -> Option<T>
    {
        self.state.get_global(name);
        self.get(-1)
    }

    /// Set a Rust value to the Lua globals.
    pub fn set_global<T: ToLua>(&self, name: &str, val: T)
    {
        self.push(val);
        self.state.set_global(name);
    }

    /// Execute a file then return a status.
    pub fn exec_file(&self, filename: &str) -> LuaStatus
    {
        match self.state.load_file(filename)
        {
            LuaOk => self.state.exec(),
            status => status,
        }
    }

    /// Execute a string then return a status.
    pub fn exec_str(&self, source: &str) -> LuaStatus
    {
        match self.state.load_str(source)
        {
            LuaOk => self.state.exec(),
            status => status,
        }
    }
}
