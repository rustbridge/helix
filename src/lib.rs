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
#[derive(Copy, Clone)]
pub struct Class(sys::VALUE);

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


pub type Metadata = ::sys::VALUE;
