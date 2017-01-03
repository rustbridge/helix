use sys::{self, VALUE, Qtrue, Qfalse};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl UncheckedValue<bool> for VALUE {
    fn to_checked(self) -> CheckResult<bool> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_TRUE) || sys::RB_TYPE_P(self, sys::T_FALSE) } {
            Ok(unsafe { CheckedValue::new(self) })
        } else {
            Err(CString::new(format!("No implicit conversion from {} to Rust bool", "?")).unwrap())
        }
    }
}

impl ToRust<bool> for CheckedValue<bool> {
    fn to_rust(self) -> bool {
        unsafe {
            if self.inner == Qtrue {
                true
            } else {
                false
            }
        }
    }
}

impl ToRuby for bool {
    fn to_ruby(self) -> VALUE {
        unsafe {
            if self {
                Qtrue
            } else {
                Qfalse
            }
        }
    }
}
