extern crate cslice;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CString;
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
    fn install(self, class: VALUE, name: &str);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        unsafe {
            sys::rb_define_method(
                class,
                CString::new(name).unwrap().as_ptr(),
                self as *const libc::c_void,
                0
            );
        }
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        unsafe {
            sys::rb_define_method(
                class,
                CString::new(name).unwrap().as_ptr(),
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
    pub fn new(name: &str) -> Class {
        ObjectClass().subclass(name)
    }

    pub fn subclass(&self, name: &str) -> Class {
        unsafe {
            Class(sys::rb_define_class(CString::new(name).unwrap().as_ptr(), self.0))
        }
    }

    pub fn define_method<T: RubyMethod>(&self, name: &str, method: T) {
        method.install(self.0, name);
    }
}

pub fn inspect(val: VALUE) -> String {
    unsafe { CheckedValue::<String>::new(sys::rb_inspect(val)).to_rust() }
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
        match any.downcast_ref::<ExceptionInfo>() {
            Some(e) => *e,
            None => {
                match any.downcast_ref::<&'static str>() {
                    Some(e) => ExceptionInfo::with_message(e.to_string()),
                    None => {
                        match any.downcast_ref::<String>() {
                            Some(e) => ExceptionInfo::with_message(e.as_str()),
                            None => ExceptionInfo::with_message(format!("Unknown Error; err={:?}", any)),
                        }
                    }
                }
            }
        }
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
