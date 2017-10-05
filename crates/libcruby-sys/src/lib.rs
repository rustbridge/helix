#![allow(non_snake_case)]
#![allow(non_camel_case_types)]

extern crate libc;

use std::ffi::CStr;

pub const PKG_VERSION: &'static str = env!("CARGO_PKG_VERSION");

pub fn check_version() {
    let raw_version = unsafe { CStr::from_ptr(HELIX_RUNTIME_VERSION) };
    let version = raw_version.to_str().expect("HELIX_RUNTIME_VERSION must be defined");

    if PKG_VERSION != version {
        panic!("libcsys-ruby version ({}) doesn't match helix_runtime version ({}).", PKG_VERSION, version);
    }
}

pub type void = libc::c_void;
pub type c_func = *const void;
pub type c_string = *const libc::c_char;
// pub type c_func = extern "C" fn(...);

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct ID(*mut void);

#[repr(C)]
#[derive(Eq, PartialEq, Copy, Clone, Debug)]
pub struct VALUE(*mut void);

#[repr(C)]
#[derive(Copy, Clone, Eq, PartialEq, Debug)]
pub struct RubyException(isize);

impl RubyException {
    pub fn new() -> RubyException {
        RubyException(0)
    }

    pub fn empty() -> RubyException {
        RubyException(0)
    }

    pub fn for_tag(tag: isize) -> RubyException {
        RubyException(tag)
    }
}

pub const EMPTY_EXCEPTION: RubyException = RubyException(0);

#[cfg_attr(windows, link(name="helix-runtime"))]
extern "C" {
    #[link_name = "HELIX_RUNTIME_VERSION"]
    pub static HELIX_RUNTIME_VERSION: c_string;

    #[link_name = "HELIX_Qfalse"]
    pub static Qfalse: VALUE;

    #[link_name = "HELIX_Qtrue"]
    pub static Qtrue: VALUE;

    #[link_name = "HELIX_Qnil"]
    pub static Qnil: VALUE;

    #[link_name = "HELIX_PRIsVALUE"]
    pub static PRIsVALUE: c_string;

    #[link_name = "HELIX_SPRINTF_TO_S"]
    pub static SPRINTF_TO_S: c_string;

    #[link_name = "HELIX_SPRINTF_INSPECT"]
    pub static SPRINTF_INSPECT: c_string;

    #[link_name = "rb_cObject"]
    pub static rb_cObject: VALUE;

    #[link_name = "rb_eRuntimeError"]
    pub static rb_eRuntimeError: VALUE;

    #[link_name = "rb_eTypeError"]
    pub static rb_eTypeError: VALUE;

    #[link_name = "HELIX_RSTRING_LEN"]
    pub fn RSTRING_LEN(string: VALUE) -> isize;

    #[link_name = "HELIX_RSTRING_PTR"]
    pub fn RSTRING_PTR(string: VALUE) -> c_string;

    #[link_name = "HELIX_rb_utf8_str_new"]
    pub fn rb_utf8_str_new(string: c_string, len: libc::c_long) -> VALUE;

    #[link_name = "HELIX_RARRAY_LEN"]
    pub fn RARRAY_LEN(array: VALUE) -> isize;

    #[link_name = "HELIX_RARRAY_PTR"]
    pub fn RARRAY_PTR(array: VALUE) -> *mut VALUE;

    #[link_name = "HELIX_RARRAY_CONST_PTR"]
    pub fn RARRAY_CONST_PTR(array: VALUE) -> *const VALUE;

    #[link_name = "HELIX_RB_TYPE_P"]
    pub fn RB_TYPE_P(val: VALUE, rb_type: isize) -> bool;

    #[link_name = "HELIX_TYPE"]
    pub fn TYPE(val: VALUE) -> isize;

    pub fn rb_check_type(v: VALUE, rb_type: isize);

    #[link_name = "HELIX_NUM2U64"]
    pub fn NUM2U64(v: VALUE) -> u64;

    #[link_name = "HELIX_U642NUM"]
    pub fn U642NUM(num: u64) -> VALUE;

    #[link_name = "HELIX_NUM2I64"]
    pub fn NUM2I64(v: VALUE) -> i64;

    #[link_name = "HELIX_I642NUM"]
    pub fn I642NUM(num: i64) -> VALUE;

    #[link_name = "HELIX_NUM2U32"]
    pub fn NUM2U32(v: VALUE) -> u32;

    #[link_name = "HELIX_U322NUM"]
    pub fn U322NUM(num: u32) -> VALUE;

    #[link_name = "HELIX_NUM2I32"]
    pub fn NUM2I32(v: VALUE) -> i32;

    #[link_name = "HELIX_I322NUM"]
    pub fn I322NUM(num: i32) -> VALUE;

    #[link_name = "HELIX_NUM2F64"]
    pub fn NUM2F64(v: VALUE) -> f64;

    #[link_name = "HELIX_F642NUM"]
    pub fn F642NUM(num: f64) -> VALUE;

    #[link_name = "HELIX_T_STRING"]
    pub static T_STRING: isize;

    #[link_name = "HELIX_T_ARRAY"]
    pub static T_ARRAY: isize;

    #[link_name = "HELIX_T_TRUE"]
    pub static T_TRUE: isize;

    #[link_name = "HELIX_T_FALSE"]
    pub static T_FALSE: isize;

    #[link_name = "HELIX_T_FIXNUM"]
    pub static T_FIXNUM: isize;

    #[link_name = "HELIX_T_FLOAT"]
    pub static T_FLOAT: isize;

    #[link_name = "HELIX_T_BIGNUM"]
    pub static T_BIGNUM: isize;

    // unknown if working?
    // fn rb_define_variable(name: c_string, value: *const VALUE);
    pub fn rb_obj_class(obj: VALUE) -> VALUE;
    pub fn rb_obj_classname(obj: VALUE) -> c_string;
    pub fn rb_const_get(class: VALUE, name: ID) -> VALUE;
    pub fn rb_define_global_const(name: c_string, value: VALUE);
    pub fn rb_define_module(name: c_string) -> VALUE;
    pub fn rb_define_module_under(namespace: VALUE, name: c_string) -> VALUE;
    pub fn rb_define_class(name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_class_under(namespace: VALUE, name: c_string, superclass: VALUE) -> VALUE;
    pub fn rb_define_alloc_func(klass: VALUE, func: extern "C" fn(klass: VALUE) -> VALUE);
    pub fn rb_define_method(class: VALUE, name: c_string, func: c_func, arity: isize);
    pub fn rb_define_singleton_method(class: VALUE, name: c_string, func: c_func, arity: isize);
    pub fn rb_sprintf(specifier: c_string, ...) -> VALUE;
    pub fn rb_inspect(value: VALUE) -> VALUE;
    pub fn rb_intern(string: c_string) -> ID;
    pub fn rb_ary_entry(ary: VALUE, offset: isize) -> VALUE;
    pub fn rb_raise(exc: VALUE, string: c_string, ...) -> !;

    pub fn rb_jump_tag(state: RubyException) -> !;
    pub fn rb_protect(try: extern "C" fn(v: *mut void) -> VALUE,
                      arg: *mut void,
                      state: *mut RubyException)
                      -> VALUE;

    #[link_name = "HELIX_Data_Wrap_Struct"]
    pub fn Data_Wrap_Struct(klass: VALUE, mark: extern "C" fn(*mut void), free: extern "C" fn(*mut void), data: *mut void) -> VALUE;

    #[link_name = "HELIX_Data_Get_Struct_Value"]
    pub fn Data_Get_Struct_Value(obj: VALUE) -> *mut void;

    #[link_name = "HELIX_Data_Set_Struct_Value"]
    pub fn Data_Set_Struct_Value(obj: VALUE, data: *mut void);
}
