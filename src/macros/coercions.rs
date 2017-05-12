#[macro_export]
macro_rules! codegen_coercions {
    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: (),
        methods: $methods:tt
    }) => (
        impl $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
            fn to_checked(self) -> $crate::CheckResult<$cls> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $cls == ::std::mem::transmute::<_, usize>(sys::rb_obj_class(self)) } {
                    Ok(unsafe { CheckedValue::new(self) })
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self)).to_string_lossy() };
                    Err(format!("No implicit conversion of {} into {}", val, stringify!($cls)))
                }
            }
        }

        impl $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
            fn to_rust(self) -> $cls {
                $cls { helix: self.inner }
            }
        }

        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);
    );

    ({
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: $methods:tt
    }) => (
        impl_struct_to_rust!(&'a $cls, $cls);
        impl_struct_to_rust!(&'a mut $cls, $cls);

        impl $crate::ToRuby for $cls {
            fn to_ruby(self) -> $crate::sys::VALUE {
                $cls::__alloc_with__(Some(Box::new(self)))
            }
        }

        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);
    );
}


#[doc(hidden)]
#[macro_export]
macro_rules! impl_to_ruby {
    ($cls:ty) => {
        item! {
            impl<'a> $crate::ToRuby for $cls {
                fn to_ruby(self) -> $crate::sys::VALUE {
                    self.helix
                }
            }
        }
    }
}

#[macro_export]
macro_rules! impl_struct_to_rust {
    ($cls:ty, $helix_id:tt) => {
        impl<'a> $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
            fn to_rust(self) -> $cls {
                unsafe { ::std::mem::transmute($crate::sys::Data_Get_Struct_Value(self.inner)) }
            }
        }

        impl<'a> $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
            fn to_checked(self) -> $crate::CheckResult<$cls> {
                use $crate::{CheckedValue, sys};
                use ::std::ffi::{CStr};

                if unsafe { $helix_id == ::std::mem::transmute(sys::rb_obj_class(self)) } {
                    if unsafe { $crate::sys::Data_Get_Struct_Value(self) == ::std::ptr::null_mut() } {
                        Err(format!("Uninitialized {}", $crate::inspect(unsafe { sys::rb_obj_class(self) })))
                    } else {
                        Ok(unsafe { CheckedValue::new(self) })
                    }
                } else {
                    let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self)).to_string_lossy() };
                    Err(format!("No implicit conversion of {} into {}", val, $crate::inspect(unsafe { sys::rb_obj_class(self) })))
                }
            }
        }
    }
}
