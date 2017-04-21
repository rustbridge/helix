extern crate cslice;

#[doc(hidden)]
pub extern crate libc;

#[doc(hidden)]
pub extern crate libcruby_sys as sys;
// pub use rb;

use std::ffi::CString;
use sys::{VALUE, RubyException};

#[macro_use]
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

pub trait RubyMethod {
    fn install(self, class: VALUE, name: &str);
}

impl RubyMethod for extern "C" fn(VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        ruby_try!(sys::safe::rb_define_method(
            class,
            CString::new(name).unwrap().as_ptr(),
            self as *const libc::c_void,
            0
        ));
    }
}

impl RubyMethod for extern "C" fn(VALUE, VALUE) -> VALUE {
    fn install(self, class: VALUE, name: &str) {
        ruby_try!(sys::safe::rb_define_method(
            class,
            CString::new(name).unwrap().as_ptr(),
            self as *const libc::c_void,
            1
        ));
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
    unsafe { CheckedValue::<String>::new(ruby_try!(sys::safe::rb_inspect(val))).to_rust() }
}

pub type Metadata = ::VALUE;


#[derive(Copy, Clone, Debug)]
pub enum ExceptionInfo {
    Library { exception: Class, message: VALUE },
    Ruby(RubyException)
}

impl ExceptionInfo {
    pub fn with_message<T: ToRuby>(string: T) -> ExceptionInfo {
        ExceptionInfo::Library {
            exception: Class(unsafe { sys::rb_eRuntimeError }),
            message: string.to_ruby(),
        }
    }

    pub fn type_error<T: ToRuby>(string: T) -> ExceptionInfo {
        ExceptionInfo::Library {
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

    pub fn from_state(state: RubyException) -> ExceptionInfo {
        ExceptionInfo::Ruby(state)
    }

    pub fn exception(&self) -> VALUE {
        match *self {
            ExceptionInfo::Library { exception, message: _ } => exception.inner(),
            _                                                => unsafe { sys::Qnil }
        }
    }

    pub fn ruby_exception(&self) -> RubyException {
        match *self {
            ExceptionInfo::Ruby(e) => e,
            _                      => sys::EMPTY_EXCEPTION
        }
    }

    pub fn message(&self) -> VALUE {
        match *self {
            ExceptionInfo::Library { exception: _, message } => message,
            _                                                => unsafe { sys::Qnil }
        }
    }

    pub fn raise(&self) -> ! {
        // Both of these will immediately leave the Rust stack. We need to be careful that nothing is
        // left behind. If there are memory leaks, this is definitely a possible culprit.
        match *self {
            ExceptionInfo::Library { exception, message } => {
                unsafe {
                    sys::rb_raise(exception.0,
                                  sys::SPRINTF_TO_S,
                                  message);
                }
            }

            ExceptionInfo::Ruby(t) => {
                unsafe {
                    sys::rb_jump_tag(t)
                }
            }
        }
    }
}

unsafe impl Send for ExceptionInfo {}
unsafe impl Sync for ExceptionInfo {}
