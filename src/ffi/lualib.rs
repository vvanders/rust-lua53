// The MIT License (MIT)
//
// Copyright (c) 2014 J.C. Moyer
//
// Permission is hereby granted, free of charge, to any person obtaining a copy
// of this software and associated documentation files (the "Software"), to deal
// in the Software without restriction, including without limitation the rights
// to use, copy, modify, merge, publish, distribute, sublicense, and/or sell
// copies of the Software, and to permit persons to whom the Software is
// furnished to do so, subject to the following conditions:
//
// The above copyright notice and this permission notice shall be included in
// all copies or substantial portions of the Software.
//
// THE SOFTWARE IS PROVIDED "AS IS", WITHOUT WARRANTY OF ANY KIND, EXPRESS OR
// IMPLIED, INCLUDING BUT NOT LIMITED TO THE WARRANTIES OF MERCHANTABILITY,
// FITNESS FOR A PARTICULAR PURPOSE AND NONINFRINGEMENT. IN NO EVENT SHALL THE
// AUTHORS OR COPYRIGHT HOLDERS BE LIABLE FOR ANY CLAIM, DAMAGES OR OTHER
// LIABILITY, WHETHER IN AN ACTION OF CONTRACT, TORT OR OTHERWISE, ARISING FROM,
// OUT OF OR IN CONNECTION WITH THE SOFTWARE OR THE USE OR OTHER DEALINGS IN
// THE SOFTWARE.

//! Contains definitions from `lualib.h`.

use ffi::lua::lua_State;
use libc::c_int;

pub use super::glue::{
  EXT_LUA_COLIBNAME, EXT_LUA_TABLIBNAME, EXT_LUA_IOLIBNAME, EXT_LUA_OSLIBNAME, EXT_LUA_STRLIBNAME,
  EXT_LUA_UTF8LIBNAME, EXT_LUA_BITLIBNAME, EXT_LUA_MATHLIBNAME, EXT_LUA_DBLIBNAME, EXT_LUA_LOADLIBNAME
};

extern {
  pub fn luaopen_base(L: *mut lua_State) -> c_int;
  pub fn luaopen_coroutine(L: *mut lua_State) -> c_int;
  pub fn luaopen_table(L: *mut lua_State) -> c_int;
  pub fn luaopen_io(L: *mut lua_State) -> c_int;
  pub fn luaopen_os(L: *mut lua_State) -> c_int;
  pub fn luaopen_string(L: *mut lua_State) -> c_int;
  pub fn luaopen_utf8(L: *mut lua_State) -> c_int;
  pub fn luaopen_bit32(L: *mut lua_State) -> c_int;
  pub fn luaopen_math(L: *mut lua_State) -> c_int;
  pub fn luaopen_debug(L: *mut lua_State) -> c_int;
  pub fn luaopen_package(L: *mut lua_State) -> c_int;

  pub fn luaL_openlibs(L: *mut lua_State);
}

