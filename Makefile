# Paths
CC						?=	gcc
RUSTC					=	rustc
RUSTDOC					=	rustdoc
AR						=	ar
RANLIB					=	ranlib

# Flags
CFLAGS					=	-fPIC
CCFLAGS					=	$(CFLAGS)
LDFLAGS					=	$(CFLAGS)
RUSTCFLAGS				=	-L $(RUST_LIBDIR)
RUSTDOCFLAGS			=

# Variables
DEBUG					=	0

RUST_LIBDIR				=	lib
RUST_DOCDIR				=	doc

ifeq ($(DEBUG),1)
CFLAGS					+=	-ggdb3
RUSTCFLAGS				+=	-g
else
CFLAGS					+=	-O3
RUSTCFLAGS				+=	--opt-level=3
endif

## UTILS
# Recursive wildcard function
# http://blog.jgc.org/2011/07/gnu-make-recursive-wildcard-function.html
rwildcard=$(foreach d,$(wildcard $1*),$(call rwildcard,$d/,$2) \
  $(filter $(subst *,%,$2),$d))

# Rules
all:

clean:

test:

bench:

doc:

# Common rules
%.o:							%.c
	$(CC) $(CCFLAGS) -c -o $@ $^

## Native lib
LUA_VERSION				=	5.2.2
LUA_DIRNAME				=	lua-$(LUA_VERSION)
LUA_IGNORE_SOURCES		=	\
	lua.c					\

LUA_SOURCES				=	$(filter-out $(foreach src,$(LUA_IGNORE_SOURCES),$(LUA_DIRNAME)/src/$(src)),$(wildcard $(LUA_DIRNAME)/src/*.c))
LUA_OBJECTS				=	$(LUA_SOURCES:.c=.o)
LUA_LIBNAME				=	$(RUST_LIBDIR)/liblua-$(LUA_VERSION).a

all:					liblua

liblua:					$(LUA_LIBNAME)
.PHONY:					liblua

clean_liblua:
	rm -f $(LUA_OBJECTS)
	rm -f $(LUA_LIBNAME)
.PHONY:					clean_liblua
clean:					clean_liblua

$(LUA_LIBNAME):				$(LUA_OBJECTS)
	@mkdir -p $(RUST_LIBDIR)
	$(AR) rc $(LUA_LIBNAME) $(LUA_OBJECTS)
	$(RANLIB) $(LUA_LIBNAME)

# Rust lib
LIBRLUA_DIRNAME			=	src/librlua
LIBRLUA_DOCDIR			=	$(RUST_DOCDIR)/librlua
LIBRLUA_RUSTCFLAGS		=	--crate-type rlib,dylib,staticlib
LIBRLUA_RUSTC			=	$(RUSTC) $(RUSTCFLAGS) $(LIBRLUA_RUSTCFLAGS)

LIBRLUA_SOURCES			=	$(wildcard $(LIBRLUA_DIRNAME)/*.rs)
LIBRLUA_ROOT			=	$(LIBRLUA_DIRNAME)/lib.rs
LIBRLUA_NAMES			=	$(addprefix $(RUST_LIBDIR)/,$(shell $(LIBRLUA_RUSTC) --crate-file-name $(LIBRLUA_ROOT)))
LIBRLUA_LIBNAME			=	$(firstword $(LIBRLUA_NAMES))
LIBRLUA_LIBNAME_TEST	=	test_librlua

all:					librlua

librlua:				$(LIBRLUA_LIBNAME)
.PHONY:					librlua

clean_librlua:
	rm -f $(LIBRLUA_NAMES)
.PHONY:					clean_librlua
clean:					clean_librlua

_test_librlua:			$(LIBRLUA_LIBNAME_TEST)
	@./$(LIBRLUA_LIBNAME_TEST)
	@rm -f $(LIBRLUA_LIBNAME_TEST)
.PHONY:					_test_librlua
test:					_test_librlua

_bench_librlua:			$(LIBRLUA_LIBNAME_TEST)
	@./$(LIBRLUA_LIBNAME_TEST) --bench
	@rm -f $(LIBRLUA_LIBNAME_TEST)
.PHONY:					_bench_librlua
bench:					_bench_librlua

_doc_librlua:			$(LIBRLUA_SOURCES)
	@mkdir -p $(LIBRLUA_DOCDIR)
	$(RUSTDOC) $(RUSTDOCFLAGS) -o $(LIBRLUA_DOCDIR) $(LIBRLUA_ROOT)
doc:					_doc_librlua

$(LIBRLUA_LIBNAME):		$(LUA_LIBNAME) $(LIBRLUA_SOURCES)
	@mkdir -p $(RUST_LIBDIR)
	$(LIBRLUA_RUSTC) --out-dir $(RUST_LIBDIR) $(LIBRLUA_ROOT)

$(LIBRLUA_LIBNAME_TEST):$(LUA_LIBNAME) $(LIBRLUA_SOURCES)
	$(LIBRLUA_RUSTC) --test -o $(LIBRLUA_LIBNAME_TEST) $(LIBRLUA_ROOT)

# Rust executable
RLUA_DIRNAME			=	src/rlua
RLUA_RUSTCFLAGS			=	--crate-type bin
RLUA_RUSTC				=	$(RUSTC) $(RUSTCFLAGS) $(RLUA_RUSTCFLAGS)

RLUA_SOURCES			=	$(wildcard $(RLUA_DIRNAME)/*.rs)
RLUA_ROOT				=	$(RLUA_DIRNAME)/main.rs
RLUA_NAMES				=	$(shell $(RLUA_RUSTC) --crate-file-name $(RLUA_ROOT))
RLUA_NAME				=	$(firstword $(RLUA_NAMES))

all:					_rlua

_rlua:				$(RLUA_NAME)
.PHONY:					_rlua

clean_rlua:
	rm -f $(RLUA_NAMES)
.PHONY:					clean_rlua
clean:					clean_rlua

$(RLUA_NAME):		$(LIBRLUA_LIBNAME) $(RLUA_SOURCES)
	@mkdir -p $(RUST_LIBDIR)
	$(RLUA_RUSTC) $(RLUA_ROOT)
