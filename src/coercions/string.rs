use libc;
use std;
use sys;
use sys::{VALUE};

use ruby::Value;
use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

// VALUE -> to_coercible_rust<String> -> CheckResult<String> -> unwrap() -> Coercible<String> -> to_rust() -> String

impl<'a> UncheckedValue<String> for Value<'a> {
    type ToRust = CheckedValue<'a, String>;

    fn to_checked(self) -> CheckResult<Self::ToRust> {
        if unsafe { sys::RB_TYPE_P(self.inner(), sys::T_STRING) } {
            Ok(unsafe { CheckedValue::<String>::new(self) })
        } else {
            Err(format!("No implicit conversion of {} into String", ::inspect(self)))
        }
    }
}

impl<'a> ToRust<String> for CheckedValue<'a, String> {
    fn to_rust(self) -> String {
        let size = unsafe { sys::RSTRING_LEN(self.inner.inner()) };
        let ptr = unsafe { sys::RSTRING_PTR(self.inner.inner()) };
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
