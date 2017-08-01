#define LUA_32BITS

#include "../../lua-source/src/lua.h"
#include "../../lua-source/src/lualib.h"
#include "../../lua-source/src/lauxlib.h"

static const int EXT_LUAL_NUMSIZES = LUAL_NUMSIZES;
static const int EXT_LUA_EXTRASPACE = LUA_EXTRASPACE;

static const char* EXT_LUA_COLIBNAME = "coroutine";
static const char* EXT_LUA_TABLIBNAME = "table";
static const char* EXT_LUA_IOLIBNAME = "io";
static const char* EXT_LUA_OSLIBNAME = "os";
static const char* EXT_LUA_STRLIBNAME = "string";
static const char* EXT_LUA_UTF8LIBNAME = "utf8";
static const char* EXT_LUA_BITLIBNAME = "bit32";
static const char* EXT_LUA_MATHLIBNAME = "math";
static const char* EXT_LUA_DBLIBNAME = "debug";
static const char* EXT_LUA_LOADLIBNAME = "package";

static const int EXT_LUA_MININTEGER = LUA_MININTEGER;
static const int EXT_LUA_MAXINTEGER = LUA_MAXINTEGER;