use libc;
use std;
use sys;
use sys::{VALUE};

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};
use super::super::Symbol;

// VALUE -> to_coercible_rust<String> -> CheckResult<String> -> unwrap() -> Coercible<String> -> to_rust() -> String

impl UncheckedValue<String> for VALUE {
    fn to_checked(self) -> CheckResult<String> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_STRING) || sys::RB_TYPE_P(self, sys::T_SYMBOL) } {
            Ok(unsafe { CheckedValue::<String>::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into String", val.to_rust()))
        }
    }
}

impl ToRust<String> for CheckedValue<String> {
    fn to_rust(self) -> String {
        let val = if unsafe { sys::RB_TYPE_P(self.inner, sys::T_SYMBOL) } {
            unsafe { sys::rb_sym2str(self.inner) }
        } else {
            self.inner
        };
        let size = unsafe { sys::RSTRING_LEN(val) };
        let ptr = unsafe { sys::RSTRING_PTR(val) };
        let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
        unsafe { std::str::from_utf8_unchecked(slice) }.to_string()
    }
}

impl ToRuby for String {
    fn to_ruby(self) -> VALUE {
        let ptr = self.as_ptr();
        let len = self.len();
        unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) }
    }
}

impl<'a> ToRuby for &'a str {
    fn to_ruby(self) -> VALUE {
        let ptr = self.as_ptr();
        let len = self.len();
        unsafe { sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long) }
    }
}

impl ToRuby for Symbol {
    fn to_ruby(self) -> VALUE {
        let ptr = self.0.as_ptr();
        let len = self.0.len();
        unsafe { sys::rb_to_symbol(sys::rb_utf8_str_new(ptr as *const libc::c_char, len as libc::c_long)) }
    }
}
