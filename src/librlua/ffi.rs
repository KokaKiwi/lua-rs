use std::libc::*;
use std::cast::transmute;
use std::ptr::*;

// Types
pub struct lua_State;

pub struct lua_Debug
{
    event: c_int,
    name: *c_char,
    namewhar: *c_char,
    what: *c_char,
    source: *c_char,
    currentline: c_int,
    linedefined: c_int,
    lastlinedefined: c_int,
    nups: c_uchar,
    nparams: c_uchar,
    isvararg: c_char,
    istailcall: c_char,
    short_src: [c_char, ..LUA_IDSIZE],

    priv i_ci: *c_void,
}

pub type lua_CFunction = extern "C" fn(L: *lua_State) -> c_int;

pub type lua_Reader = extern "C" fn(L: *lua_State, ud: *c_void, sz: size_t) -> *c_char;
pub type lua_Writer = extern "C" fn(L: *lua_State, p: *c_void, ze: size_t, ud: *c_void);

pub type lua_Alloc = extern "C" fn(ud: *c_void, ptr: *c_void, osize: size_t, nsize: size_t);

pub type lua_Integer = ptrdiff_t;
pub type lua_Number = c_double;
pub type lua_Unsigned = c_ulong;

pub type lua_Hook = extern fn(L: *lua_State, ar: *lua_Debug);

// Constants
pub static LUAI_MAXSTACK: c_int = 1000000;
pub static LUAI_FIRSTPSEUDOIDX: c_int = -LUAI_MAXSTACK - 1000;
pub static LUA_IDSIZE: c_int = 60;
pub static LUA_MULTRET: c_int = -1;

// - Pseudo-indices
pub static LUA_REGISTRYINDEX: c_int = LUAI_FIRSTPSEUDOIDX;

// - thread status
pub static LUA_OK: c_int = 0;
pub static LUA_YIELD: c_int = 1;
pub static LUA_ERRRUN: c_int = 2;
pub static LUA_ERRSYNTAX: c_int = 3;
pub static LUA_ERRMEM: c_int = 4;
pub static LUA_ERRGCMM: c_int = 5;
pub static LUA_ERRERR: c_int = 6;
pub static LUA_ERRFILE: c_int = LUA_ERRERR + 1;

// - Lua types
pub static LUA_TNONE: c_int = -1;

pub static LUA_TNIL: c_int = 0;
pub static LUA_TBOOLEAN: c_int = 1;
pub static LUA_TLIGHTUSERDATA: c_int = 2;
pub static LUA_TNUMBER: c_int = 3;
pub static LUA_TSTRING: c_int = 4;
pub static LUA_TTABLE: c_int = 5;
pub static LUA_TFUNCTION: c_int = 6;
pub static LUA_TUSERDATA: c_int = 7;
pub static LUA_TTHREAD: c_int = 8;

pub static LUA_NUMTAGS: c_int = 9;

// - Lua misc constants
pub static LUA_MINSTACK: c_int = 20;

pub static LUA_RIDX_MAINTHREAD: c_int = 1;
pub static LUA_RIDX_GLOBALS: c_int = 2;
pub static LUA_RIDX_LAST: c_int = LUA_RIDX_GLOBALS;

// - Comparison and arithmetic constants
pub static LUA_OPADD: c_int = 0;
pub static LUA_OPSUB: c_int = 1;
pub static LUA_OPMUL: c_int = 2;
pub static LUA_OPDIV: c_int = 3;
pub static LUA_OPMOD: c_int = 4;
pub static LUA_OPPOW: c_int = 5;
pub static LUA_OPUNM: c_int = 6;

pub static LUA_OPEQ: c_int = 0;
pub static LUA_OPLT: c_int = 1;
pub static LUA_OPLE: c_int = 2;

// - Garbage-collection options
pub static LUA_GCSTOP: c_int = 0;
pub static LUA_GCRESTART: c_int = 1;
pub static LUA_GCCOLLECT: c_int = 2;
pub static LUA_GCCOUNT: c_int = 3;
pub static LUA_GCCOUNTB: c_int = 4;
pub static LUA_GCSTEP: c_int = 5;
pub static LUA_GCSETPAUSE: c_int = 6;
pub static LUA_GCSETSTEPMUL: c_int = 7;
pub static LUA_GCSETMAJORINC: c_int = 8;
pub static LUA_GCISRUNNING: c_int = 9;
pub static LUA_GCGEN: c_int = 10;
pub static LUA_GCINC: c_int = 11;

// - Event codes
pub static LUA_HOOKCALL: c_int = 0;
pub static LUA_HOOKRET: c_int = 1;
pub static LUA_HOOKLINE: c_int = 2;
pub static LUA_HOOKCOUNT: c_int = 3;
pub static LUA_HOOKTAILCALL: c_int = 4;

// - Event masks
pub static LUA_MASKCALL: c_int = 1 << LUA_HOOKCALL;
pub static LUA_MASKRET: c_int = 1 << LUA_HOOKRET;
pub static LUA_MASKLINE: c_int = 1 << LUA_HOOKLINE;
pub static LUA_MASKCOUNT: c_int = 1 << LUA_HOOKCOUNT;

#[link(name = "lua")]
extern
{
    // State manipulation
    pub fn lua_newstate(f: lua_Alloc, ud: *c_void) -> *lua_State;
    pub fn lua_close(L: *lua_State);
    pub fn lua_newthread(L: *lua_State) -> *lua_State;

    pub fn lua_atpanic(L: *lua_State, panicf: lua_CFunction) -> lua_CFunction;

    pub fn lua_version(L: *lua_State) -> *lua_Number;

    // Basic stack manipulation
    pub fn lua_absindex(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_gettop(L: *lua_State) -> c_int;
    pub fn lua_settop(L: *lua_State, idx: c_int);
    pub fn lua_pushvalue(L: *lua_State, idx: c_int);
    pub fn lua_remove(L: *lua_State, idx: c_int);
    pub fn lua_insert(L: *lua_State, idx: c_int);
    pub fn lua_replace(L: *lua_State, idx: c_int);
    pub fn lua_copy(L: *lua_State, fromidx: c_int, toidx: c_int);
    pub fn lua_checkstack(L: *lua_State, sz: c_int) -> c_int;

    pub fn lua_xmove(from: *lua_State, to: *lua_State, n: c_int);

    // Access functions (stack -> C)
    pub fn lua_isnumber(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_isstring(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_iscfunction(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_isuserdata(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_type(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_typename(L: *lua_State, tp: c_int) -> *c_char;

    pub fn lua_tonumberx(L: *lua_State, idx: c_int, isnum: *c_int) -> lua_Number;
    pub fn lua_tointegerx(L: *lua_State, idx: c_int, isnum: *c_int) -> lua_Integer;
    pub fn lua_tounsignedx(L: *lua_State, idx: c_int, isnum: *c_int) -> lua_Unsigned;
    pub fn lua_toboolean(L: *lua_State, idx: c_int) -> c_int;
    pub fn lua_tolstring(L: *lua_State, idx: c_int, len: *size_t) -> *c_char;
    pub fn lua_rawlen(L: *lua_State, idx: c_int) -> size_t;
    pub fn lua_tocfunction(L: *lua_State, idx: c_int) -> lua_CFunction;
    pub fn lua_touserdata(L: *lua_State, idx: c_int) -> *c_void;
    pub fn lua_tothread(L: *lua_State, idx: c_int) -> *lua_State;
    pub fn lua_topointer(L: *lua_State, idx: c_int) -> *c_void;

    // Comparison and arithmetic functions
    pub fn lua_arith(L: *lua_State, op: c_int);

    pub fn lua_rawequal(L: *lua_State, idx1: c_int, idx2: c_int) -> c_int;
    pub fn lua_compare(L: *lua_State, idx1: c_int, idx2: c_int, op: c_int) -> c_int;

    // push functions (C -> stack)
    pub fn lua_pushnil(L: *lua_State);
    pub fn lua_pushnumber(L: *lua_State, n: lua_Number);
    pub fn lua_pushinteger(L: *lua_State, n: lua_Integer);
    pub fn lua_pushunsigned(L: *lua_State, n: lua_Unsigned);
    pub fn lua_pushlstring(L: *lua_State, s: *c_char, l: size_t) -> *c_char;
    pub fn lua_pushstring(L: *lua_State, s: *c_char) -> *c_char;
    pub fn lua_pushcclosure(L: *lua_State, f: lua_CFunction, n: c_int);
    pub fn lua_pushboolean(L: *lua_State, b: c_int);
    pub fn lua_pushlightuserdata(L: *lua_State, p: *c_void);
    pub fn lua_pushthread(L: *lua_State) -> c_int;

    // get functions (Lua -> stack)
    pub fn lua_getglobal(L: *lua_State, var: *c_char);
    pub fn lua_gettable(L: *lua_State, idx: c_int);
    pub fn lua_getfield(L: *lua_State, idx: c_int, k: *c_char);
    pub fn lua_rawget(L: *lua_State, idx: c_int);
    pub fn lua_rawgeti(L: *lua_State, idx: c_int, n: c_int);
    pub fn lua_rawgetp(L: *lua_State, idx: c_int, p: *c_void);
    pub fn lua_createtable(L: *lua_State, narr: c_int, nrec: c_int);
    pub fn lua_newuserdata(L: *lua_State, sz: size_t) -> *c_void;
    pub fn lua_getmetatable(L: *lua_State, objindex: c_int) -> c_int;
    pub fn lua_getuservalue(L: *lua_State, idx: c_int);

    // set functions (stack -> Lua)
    pub fn lua_setglobal(L: *lua_State, var: *c_char);
    pub fn lua_settable(L: *lua_State, idx: c_int);
    pub fn lua_setfield(L: *lua_State, idx: c_int, k: *c_char);
    pub fn lua_rawset(L: *lua_State, idx: c_int);
    pub fn lua_rawseti(L: *lua_State, idx: c_int, n: c_int);
    pub fn lua_rawsetp(L: *lua_State, idx: c_int, p: *c_void);
    pub fn lua_setmetatable(L: *lua_State, objindex: c_int) -> c_int;
    pub fn lua_setuservalue(L: *lua_State, idx: c_int);

    // 'load' and 'call' functions (load and run Lua code)
    pub fn lua_callk(L: *lua_State, nargs: c_int, nresults: c_int, ctx: c_int, k: lua_CFunction);
    pub fn lua_getctx(L: *lua_State, ctx: *c_int) -> c_int;
    pub fn lua_pcallk(L: *lua_State, nargs: c_int, nresults: c_int, errfunc: c_int, ctx: c_int, k: lua_CFunction) -> c_int;
    pub fn lua_load(L: *lua_State, reader: lua_Reader, dt: *c_void, chunkname: *c_char, mode: *c_char) -> c_int;
    pub fn lua_dump(L: *lua_State, writer: lua_Writer, data: *c_void) -> c_int;

    // coroutine functions
    pub fn lua_yieldk(L: *lua_State, nresults: c_int, ctx: c_int, k: lua_CFunction) -> c_int;

    pub fn lua_resume(L: *lua_State, from: *lua_State, narg: c_int) -> c_int;
    pub fn lua_status(L: *lua_State) -> c_int;

    // garbage-collection function
    pub fn lua_gc(L: *lua_State, what: c_int, data: c_int) -> c_int;

    // miscellaneous functions
    pub fn lua_error(L: *lua_State) -> c_int;

    pub fn lua_next(L: *lua_State, idx: c_int) -> c_int;

    pub fn lua_concat(L: *lua_State, n: c_int);
    pub fn lua_len(L: *lua_State, idx: c_int);

    pub fn lua_getallocf(L: *lua_State, ud: **c_void) -> lua_Alloc;
    pub fn lua_setallocf(L: *lua_State, f: lua_Alloc, ud: *c_void);

    // Functions to be called by the debugger in specific events
    pub fn lua_getstack(L: *lua_State, level: c_int, ar: *lua_Debug) -> c_int;
    pub fn lua_getinfo(L: *lua_State, what: *c_char, ar: *lua_Debug) -> c_int;
    pub fn lua_getlocal(L: *lua_State, ar: *lua_Debug, n: c_int) -> *c_char;
    pub fn lua_setlocal(L: *lua_State, ar: *lua_Debug, n: c_int) -> *c_char;
    pub fn lua_getupvalue(L: *lua_State, funcindex: c_int, n: c_int) -> *c_char;
    pub fn lua_setupvalue(L: *lua_State, funcindex: c_int, n: c_int) -> *c_char;

    pub fn lua_upvalueid(L: *lua_State, fidx: c_int, n: c_int) -> *c_void;
    pub fn lua_upvaluejoin(L: *lua_State, fidx1: c_int, n1: c_int, fidx2: c_int, n2: c_int);

    pub fn lua_sethook(L: *lua_State, func: lua_Hook, mask: c_int, count: c_int) -> c_int;
    pub fn lua_gethook(L: *lua_State) -> lua_Hook;
    pub fn lua_gethookmask(L: *lua_State) -> c_int;
    pub fn lua_gethookcount(L: *lua_State) -> c_int;

    // Auxiliary API
    pub fn luaL_newstate() -> *lua_State;
    pub fn luaL_openlibs(L: *lua_State);
    pub fn luaL_loadfilex(L: *lua_State, filename: *c_char, mode: *c_char) -> c_int;
    pub fn luaL_loadstring(L: *lua_State, s: *c_char) -> c_int;
    pub fn luaL_ref(L: *lua_State, t: c_int) -> c_int;
    pub fn luaL_unref(L: *lua_State, t: c_int, r: c_int);
}

pub unsafe fn lua_upvalueindex(i: c_int) -> c_int
{
    LUA_REGISTRYINDEX - i
}

pub unsafe fn lua_call(L: *lua_State, nargs: c_int, nresults: c_int)
{
    lua_callk(L, nargs, nresults, 0, transmute(null::<c_void>()))
}

pub unsafe fn lua_pcall(L: *lua_State, nargs: c_int, nresults: c_int, errfunc: c_int) -> c_int
{
    lua_pcallk(L, nargs, nresults, errfunc, 0, transmute(null::<c_void>()))
}

pub unsafe fn lua_yield(L: *lua_State, nresults: c_int) -> c_int
{
    lua_yieldk(L, nresults, 0, transmute(null::<c_void>()))
}

pub unsafe fn lua_tonumber(L: *lua_State, idx: c_int) -> lua_Number
{
    lua_tonumberx(L, idx, transmute(null::<c_void>()))
}

pub unsafe fn lua_tointeger(L: *lua_State, idx: c_int) -> lua_Integer
{
    lua_tointegerx(L, idx, transmute(null::<c_void>()))
}

pub unsafe fn lua_tounsigned(L: *lua_State, idx: c_int) -> lua_Unsigned
{
    lua_tounsignedx(L, idx, transmute(null::<c_void>()))
}

pub unsafe fn lua_tostring(L: *lua_State, idx: c_int) -> *c_char
{
    lua_tolstring(L, idx, transmute(null::<c_void>()))
}

pub unsafe fn lua_pop(L: *lua_State, idx: c_int)
{
    lua_settop(L, -idx - 1)
}

pub unsafe fn lua_newtable(L: *lua_State)
{
    lua_createtable(L, 0, 0)
}

pub unsafe fn lua_pushcfunction(L: *lua_State, f: lua_CFunction)
{
    lua_pushcclosure(L, f, 0)
}

pub unsafe fn lua_register(L: *lua_State, var: *c_char, f: lua_CFunction)
{
    lua_pushcfunction(L, f);
    lua_setglobal(L, var)
}

pub unsafe fn lua_isfunction(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TFUNCTION
}

pub unsafe fn lua_istable(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TTABLE
}

pub unsafe fn lua_islightuserdata(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TLIGHTUSERDATA
}

pub unsafe fn lua_isnil(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TNIL
}

pub unsafe fn lua_isboolean(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TBOOLEAN
}

pub unsafe fn lua_isthread(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TTHREAD
}

pub unsafe fn lua_isnone(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) == LUA_TNONE
}

pub unsafe fn lua_isnoneornil(L: *lua_State, idx: c_int) -> bool
{
    lua_type(L, idx) <= 0
}

pub unsafe fn lua_pushglobaltable(L: *lua_State)
{
    lua_rawgeti(L, LUA_REGISTRYINDEX, LUA_RIDX_GLOBALS)
}

pub unsafe fn luaL_loadfile(L: *lua_State, filename: *c_char) -> c_int
{
    luaL_loadfilex(L, filename, transmute(null::<c_void>()))
}
