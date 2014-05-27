# Paths
CC                      ?=  gcc
AR                      =   ar
RANLIB                  =   ranlib

# Flags
CFLAGS                  =   -fPIC
CCFLAGS                 =   $(CFLAGS)

# Variables
DEBUG                   =   1

RUSTDEBUG               =   $(DEBUG)
RUSTAUTORULES           =   0

ifeq ($(DEBUG),1)
CFLAGS                  +=  -ggdb3
else
CFLAGS                  +=  -O3
endif

# rlua
RUSTCRATES              =   liblua rlua
rlua_CRATE_DEPS         +=  liblua

include                 rust-mk/rust.mk

# liblua
LUA_VERSION             =   5.2.3
LUA_DIRNAME             =   lua-$(LUA_VERSION)
LUA_IGNORE_SOURCES      =   \
  lua.c                     \

LUA_SOURCES             =   $(filter-out $(foreach src,$(LUA_IGNORE_SOURCES),$(LUA_DIRNAME)/src/$(src)),$(wildcard $(LUA_DIRNAME)/src/*.c))
LUA_OBJECTS             =   $(LUA_SOURCES:.c=.o)
LUA_LIBNAME             =   $(RUSTLIBDIR)/liblua.a
liblua_BUILD_DEPS      +=  $(LUA_LIBNAME)

clean_liblua_native:
	rm -f $(LUA_LIBNAME)
	rm -f $(LUA_OBJECTS)
.PHONY clean:           clean_liblua_native

%.o:                    %.c
	$(CC) $(CCFLAGS) -c -o $@ $^

$(LUA_LIBNAME):         $(LUA_OBJECTS)
	@mkdir -p $(dir $(LUA_LIBNAME))
	$(AR) rc $(LUA_LIBNAME) $(LUA_OBJECTS)
	$(RANLIB) $(LUA_LIBNAME)

$(eval $(call RUST_CRATES_RULES))
