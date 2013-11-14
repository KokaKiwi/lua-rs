#[macro_escape];

macro_rules! lua_struct(
    ($name:ident {
        $( $field:ident: $ty:ty ),+
    }) => {
        mod $name
        {
            use lua::state::State;
            use lua::traits::{FromLua, ToLua};

            pub struct $name
            {
                $( $field: $ty, )+
            }

            impl FromLua for $name
            {
                fn from_lua(state: &State, idx: int) -> Option<$name>
                {
                    let s = $name {
                        $(
                        $field: {
                            state.get_field(idx, stringify!($field));
                            let r = match FromLua::from_lua(state, idx) {
                                Some(r) => r,
                                None => {
                                    return None;
                                }
                            };
                            state.pop(1);

                            r
                        },
                        )+
                    };

                    Some(s)
                }
            }

            impl ToLua for $name
            {
                fn to_lua(self, state: &State)
                {
                    state.new_table();

                    $(
                    state.push_str(stringify!($field));
                    self.$field.to_lua(state);
                    state.raw_set(-3);
                    )+
                }
            }
        }
    }
)

macro_rules! lua_fn_export(
    // Function with no args and no return value
    ($proxy_name:ident: $name:ident()) => {
        fn $proxy_name(_: &lua::Lua) -> int
        {
            $name();
            0
        }
    };

    // Function with no args but return value
    ($proxy_name:ident: $name:ident() -> $rty:ty) => {
        fn $proxy_name(l: &lua::Lua) -> int
        {
            let ret = $name();

            l.push(ret);
            1
        }
    };

    // Function with args and no return value
    ($proxy_name:ident: $name:ident($($arg:ident: $ty:ty),+)) => {
        fn $proxy_name(l: &lua::Lua) -> int
        {
            let mut _i = 1;
            $(
                let $arg: $ty = match l.get(_i) {
                    Some(v) => v,
                    None => fail!("Bad argument: {}", _i),
                };
                _i += 1;
            )+

            $name($($arg),*);
            0
        }
    };

    // Function with args and return value
    ($proxy_name:ident: $name:ident($($arg:ident: $ty:ty),+) -> $rty:ty) => {
        fn $proxy_name(l: &lua::Lua) -> int
        {
            let mut _i = 1;
            $(
                let $arg: $ty = match l.get(_i) {
                    Some(v) => v,
                    None => fail!("Bad argument: {}", _i),
                };
                _i += 1;
            )+

            let ret = $name($($arg),*);

            l.push(ret);
            1
        }
    };
)

macro_rules! lua_fn_import(
    // Function with no args and no return value
    ($name:ident()) => {
        fn $name(l: &lua::Lua)
        {
            l.state.get_global(stringify!($name));

            l.state.pcall(0, 0, 0);
        }
    };

    // Function with no args but return value
    ($name:ident() -> $rty:ty) => {
        fn $name(l: &lua::Lua) -> $rty
        {
            l.state.get_global(stringify!($name));

            l.state.pcall(0, 1, 0);

            match l.pop()
            {
                Some(ret) => ret,
                None => fail!("Bad return value!"),
            }
        }
    };

    // Function with args and no return value
    ($name:ident($($arg:ident: $ty:ty),+)) => {
        fn $name(l: &lua::Lua, $($arg: $ty),+)
        {
            l.state.get_global(stringify!($name));

            let mut _len = 0;

            $(
                l.push($arg);
                _len += 1;
            )+

            l.state.pcall(_len, 0, 0);
        }
    };

    // Function with args and return value
    ($name:ident($($arg:ident: $ty:ty),+) -> $rty:ty) => {
        fn $name(l: &lua::Lua, $($arg: $ty),+) -> $rty
        {
            l.state.get_global(stringify!($name));

            let mut _len = 0;

            $(
                l.push($arg);
                _len += 1;
            )+

            l.state.pcall(_len, 1, 0);

            match l.pop()
            {
                Some(ret) => ret,
                None => fail!("Bad return value!"),
            }
        }
    };

    // Closure with no args and no return value
    ($lua:ident: $name:ident | |) => {
        || {
            lua_fn_import!($name())

            $name(&$lua)
        }
    };

    // Closure with no args but return value
    ($lua:ident: $name: ident | | -> $rty:ty) => {
        || {
            lua_fn_import!($name() -> $rty)

            $name(&$lua)
        }
    };

    // Closure with args and no return value
    ($lua:ident: $name: ident | $($arg:ident: $ty:ty),+ |) => {
        |$($arg),+| {
            lua_fn_import!($name($($arg: $ty),+))

            $name(&$lua, $($arg),+)
        }
    };

    // Closure with args and return value
    ($lua:ident: $name: ident | $($arg:ident: $ty:ty),+ | -> $rty:ty) => {
        |$($arg),+| {
            lua_fn_import!($name($($arg: $ty),+) -> $rty)

            $name(&$lua, $($arg),+)
        }
    };
)
