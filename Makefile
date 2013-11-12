# Rust
RUST_MODULES                        :=  liblua lua
lua_DEPS                            :=  liblua

include                             rust-mk/rust.mk

# Lua
CC                                  ?=  gcc
CFLAGS                              :=  -fPIC
AR                                  ?=  ar
RANLIB                              ?=  ranlib

LUA_VERSION                         :=  5.2.2
LUA_DIRNAME                         :=  lua-$(LUA_VERSION)

RUSTCFLAGS                          +=  -L $(RUST_BUILDDIR)

$(RUST_BUILDDIR)/.build_liblua:     $(RUST_BUILDDIR)/liblua.a

lua_SOURCES                         :=  $(filter-out $(LUA_DIRNAME)/src/lua.c $(LUA_DIRNAME)/src/luac.c,$(wildcard $(LUA_DIRNAME)/src/*.c))
lua_OBJECTS                         :=  $(addprefix $(RUST_BUILDDIR)/lua/,$(notdir $(lua_SOURCES:.c=.o)))

$(RUST_BUILDDIR)/liblua.a:          $(RUST_BUILDDIR)/lua $(lua_OBJECTS)
	$(AR) rc $@ $(lua_OBJECTS)
	$(RANLIB) $@

$(RUST_BUILDDIR)/lua/%.o:           $(LUA_DIRNAME)/src/%.c
	$(CC) $(CFLAGS) -c -o $@ $^

$(RUST_BUILDDIR)/lua:
	@mkdir -p $@

expand:
	$(RUSTC) $(RUSTCFLAGS) --parse-only --pretty expanded src/liblua/lib.rs

llvm:
	$(RUSTC) --opt-level=3 -S --emit-llvm -o lib.ll src/liblua/lib.rs
