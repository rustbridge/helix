use sys::{VALUE, RB_TYPE_P, T_HASH, RHASH_SIZE, rb_hash_foreach, rb_hash_new, rb_hash_aset, void, st_retval};
use super::{FromRuby, CheckResult, ToRuby, ToRubyResult};
use std::collections::hash_map::HashMap;
use std::hash::Hash;
use ::std::mem::transmute;

extern "C" fn rb_hash_collect(key: VALUE, value: VALUE, vec: *mut void) -> st_retval {
    let vec: &mut Vec<(VALUE, VALUE)> = unsafe { transmute(vec) };
    vec.push((key, value));
    st_retval::ST_CONTINUE
}

impl<K: FromRuby + Eq + Hash, V: FromRuby> FromRuby for HashMap<K, V> {
    type Checked = Vec<(K::Checked, V::Checked)>;

    fn from_ruby(value: VALUE) -> CheckResult<Self::Checked> {
        if unsafe { RB_TYPE_P(value, T_HASH) } {
            let len = unsafe { RHASH_SIZE(value) };

            let mut pairs = Vec::<(VALUE, VALUE)>::with_capacity(len as usize);
            unsafe { rb_hash_foreach(value, Some(rb_hash_collect), transmute(&mut pairs)) };

            let mut checked = Vec::<(K::Checked, V::Checked)>::with_capacity(len as usize);

            for (k, v) in pairs.into_iter() {
                let k = K::from_ruby(k)?;
                let v = V::from_ruby(v)?;

                checked.push((k, v));
            }

            Ok(checked)
        } else {
            type_error!("a hash");
        }
    }

    fn from_checked(checked: Self::Checked) -> HashMap<K, V> {
        checked.into_iter().map(|(k, v)| (K::from_checked(k), V::from_checked(v))).collect()
    }
}

impl<K: ToRuby + Eq + Hash, V: ToRuby> ToRuby for HashMap<K, V> {
    fn to_ruby(self) -> ToRubyResult {
        let hash = unsafe { rb_hash_new() };

        for (k,v) in self.into_iter() {
            unsafe { rb_hash_aset(hash, k.to_ruby()?, v.to_ruby()?) };
        }

        Ok(hash)
    }
}
