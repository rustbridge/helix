use sys::{RARRAY_LEN, RB_TYPE_P, T_ARRAY, VALUE, rb_ary_entry};
use super::{FromRuby, CheckResult};

impl<T: FromRuby> FromRuby for Vec<T> {
    type Checked = Vec<T::Checked>;

    fn from_ruby(value: VALUE) -> CheckResult<Self::Checked> {
        if unsafe { RB_TYPE_P(value, T_ARRAY) } {
            let size = unsafe { RARRAY_LEN(value) };
            let mut vec = Vec::with_capacity(size as usize);

            for i in 0..size {
                let entry = unsafe { rb_ary_entry(value, i) };

                match T::from_ruby(entry) {
                    Ok(checked) => vec.push(checked),
                    Err(reason) => {
                        type_error!(format!("Cannot convert element {}: {}", i, reason))
                    }
                }
            }

            Ok(vec)
        } else {
            type_error!(value, "an array")
        }
    }

    fn from_checked(checked: Self::Checked) -> Vec<T> {
        checked.into_iter().map(T::from_checked).collect()
    }
}
