use sys;
use sys::{VALUE};
use std::ffi::CString;
use std::cmp::Eq;
use std::hash::Hash;
use std::collections::HashMap;

use super::{UncheckedValue, CheckResult, CheckedValue, ToRust, ToRuby};

// TODO: Should we convert the key directly to str?

impl<K, V> UncheckedValue<HashMap<K, V>> for VALUE
        where VALUE: UncheckedValue<K> + UncheckedValue<V> {
    fn to_checked(self) -> CheckResult<HashMap<K, V>> {
        if unsafe { sys::RB_TYPE_P(self, sys::T_HASH) } {
            // Make sure we can actually do the conversions for the keys and values.
            // Ideally, we'd find a way to pass along the CheckedValues so we don't have to do it again.
            let keys = unsafe { sys::rb_funcall(self, sys::rb_intern(CString::new("keys").unwrap().as_ptr()), 0) };
            let values = unsafe { sys::rb_funcall(self, sys::rb_intern(CString::new("values").unwrap().as_ptr()), 0) };
            let len = unsafe { sys::RARRAY_LEN(keys) };
            for i in 0..len {
                let key = unsafe { sys::rb_ary_entry(keys, i) };
                if let Err(error) = UncheckedValue::<K>::to_checked(key) {
                    return Err(format!("Failed to convert key for HashMap<K, V>: {}", error))
                }

                let val = unsafe { sys::rb_ary_entry(values, i) };
                if let Err(error) = UncheckedValue::<V>::to_checked(val) {
                    return Err(format!("Failed to convert value for HashMap<K, V>: {}", error))
                }
            }
            Ok(unsafe { CheckedValue::<HashMap<K, V>>::new(self) })
        } else {
            let val = unsafe { CheckedValue::<String>::new(sys::rb_inspect(self)) };
            Err(format!("No implicit conversion of {} into HashMap<K, V>", val.to_rust()))
        }
    }
}

impl<K, V> ToRust<HashMap<K, V>> for CheckedValue<HashMap<K, V>>
        where VALUE: UncheckedValue<K> + UncheckedValue<V>, CheckedValue<K>: ToRust<K>, CheckedValue<V>: ToRust<V>, K: Eq + Hash {
    fn to_rust(self) -> HashMap<K, V> {
        let keys = unsafe { sys::rb_funcall(self.inner, sys::rb_intern(CString::new("keys").unwrap().as_ptr()), 0) };
        let values = unsafe { sys::rb_funcall(self.inner, sys::rb_intern(CString::new("values").unwrap().as_ptr()), 0) };
        let len = unsafe { sys::RARRAY_LEN(keys) };

        let mut map: HashMap<K, V> = HashMap::new();

        for i in 0..len {
            let key = unsafe { sys::rb_ary_entry(keys, i) };
            let val = unsafe { sys::rb_ary_entry(values, i) };
            let checked_key = UncheckedValue::<K>::to_checked(key).unwrap();
            let checked_val = UncheckedValue::<V>::to_checked(val).unwrap();
            map.insert(checked_key.to_rust(), checked_val.to_rust());
        }

        map
    }
}

impl<K, V> ToRuby for HashMap<K, V>
        where K: ToRuby + Eq + Hash, V: ToRuby {
    fn to_ruby(self) -> VALUE {
        let hash = unsafe { sys::rb_hash_new() };
        for (k,v) in self.into_iter() {
            unsafe { sys::rb_hash_aset(hash, k.to_ruby(), v.to_ruby()); }
        }
        hash
    }
}
