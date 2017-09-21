extern crate cslice;

#[allow(unused_imports)]
#[macro_use]
extern crate cstr_macro;

#[doc(hidden)]
pub use cstr_macro::*;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CStr;
use sys::VALUE;

mod macros;
mod class_definition;
mod coercions;

pub use coercions::*;

pub use class_definition::{ClassDefinition, MethodDefinition};

#[repr(C)]
#[derive(Copy, Clone, Debug)]
pub struct Class(VALUE);

impl Class {
    pub fn inner(&self) -> VALUE {
        self.0
    }
}

#[derive(Clone, PartialEq, Eq, Hash)]
pub struct Symbol(String);

// Unfortunately the tuple struct constructor isn't public, do this to expose it
impl Symbol {
    pub fn new(string: String) -> Symbol {
        Symbol(string)
    }
}

pub trait RubyMethod {
    fn install(self, class: VALUE, name: &CStr);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &CStr) {
        unsafe {
            sys::rb_define_method(
                class,
                name.as_ptr(),
                self as *const libc::c_void,
                0
            );
        }
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &CStr) {
        unsafe {
            sys::rb_define_method(
                class,
                name.as_ptr(),
                self as *const libc::c_void,
                1
            );
        }
    }
}

#[allow(non_snake_case)]
#[inline]
fn ObjectClass() -> Class {
    Class(unsafe { sys::rb_cObject })
}

impl Class {
    pub fn new(name: &CStr) -> Class {
        ObjectClass().subclass(name)
    }

    pub fn subclass(&self, name: &CStr) -> Class {
        unsafe {
            Class(sys::rb_define_class(name.as_ptr(), self.0))
        }
    }

    pub fn define_method<T: RubyMethod>(&self, name: &CStr, method: T) {
        method.install(self.0, name);
    }
}

pub fn inspect(val: VALUE) -> String {
    unsafe { CheckedValue::<String>::new(sys::rb_inspect(val)).to_rust() }
}

pub fn invalid(val: VALUE, expected: &str) -> String {
    let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(val)) };
    format!("Expected {}, got {}", expected, val.to_rust())
}

pub unsafe fn as_usize(value: ::VALUE) -> usize {
    std::mem::transmute(value)
}

pub type Metadata = ::VALUE;

#[derive(Copy, Clone, Debug)]
pub struct ExceptionInfo {
    pub exception: Class,
    pub message: VALUE
}

impl ExceptionInfo {
    pub fn with_message<T: ToRuby>(string: T) -> ExceptionInfo {
        ExceptionInfo {
            exception: Class(unsafe { sys::rb_eRuntimeError }),
            message: string.to_ruby(),
        }
    }

    pub fn type_error<T: ToRuby>(string: T) -> ExceptionInfo {
        ExceptionInfo {
            exception: Class(unsafe { sys::rb_eTypeError }),
            message: string.to_ruby(),
        }
    }

    pub fn from_any(any: Box<std::any::Any>) -> ExceptionInfo {
        any.downcast_ref::<ExceptionInfo>()
            .map(|e| *e)
            .or_else(||
                any.downcast_ref::<&'static str>()
                    .map(|e| e.to_string())
                    .map(ExceptionInfo::with_message)
            )
            .or_else(||
                any.downcast_ref::<String>()
                    .map(|e| e.as_str())
                    .map(ExceptionInfo::with_message)
            )
            .unwrap_or_else(||
                ExceptionInfo::with_message(format!("Unknown Error; err={:?}", any))
            )
    }

    pub fn message(&self) -> VALUE {
        self.message
    }

    pub fn raise(&self) -> ! {
        unsafe {
            sys::rb_raise(self.exception.0,
                          sys::SPRINTF_TO_S,
                          self.message);
        }
    }
}

unsafe impl Send for ExceptionInfo {}
unsafe impl Sync for ExceptionInfo {}
