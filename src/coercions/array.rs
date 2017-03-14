use sys;
use sys::{VALUE};
use std::ffi::CString;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

impl<T> UncheckedValue<Vec<T>> for VALUE
        where VALUE: UncheckedValue<T> {
    fn to_checked(self) -> CheckResult<Vec<T>> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_ARRAY) } {
            // Make sure we can actually do the conversions for the values.
            // Ideally, we'd find a way to pass along the CheckedValues so we don't have to do it again.
            let len = unsafe { sys::RARRAY_LEN(self) };
            for i in 0..len {
                let val = unsafe { sys::rb_ary_entry(self, i) };
                if let Err(error) = val.to_checked() {
                    return Err(CString::new(format!("Failed to convert value for Vec<T>: {}", error.to_str().unwrap())).unwrap())
                }
            }
            Ok(unsafe { CheckedValue::<Vec<T>>::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(CString::new(format!("No implicit conversion of {} into Vec<T>", val.to_rust())).unwrap())
        }
    }
}

impl<T> ToRust<Vec<T>> for CheckedValue<Vec<T>>
        where VALUE: UncheckedValue<T>, CheckedValue<T>: ToRust<T> {
    fn to_rust(self) -> Vec<T> {
        let len = unsafe { sys::RARRAY_LEN(self.inner) };
        let mut vec: Vec<T> = Vec::new();
        for i in 0..len {
            let val = unsafe { sys::rb_ary_entry(self.inner, i) };
            let checked = val.to_checked().unwrap();
            vec.push(checked.to_rust());
        }
        vec
    }
}

impl<T: ToRuby> ToRuby for Vec<T> {
    fn to_ruby(self) -> VALUE {
        let ary = unsafe { sys::rb_ary_new_capa(self.len() as isize) };
        for item in self {
            unsafe { sys::rb_ary_push(ary, item.to_ruby()); }
        }
        ary
    }
}
