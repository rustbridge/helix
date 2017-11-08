#[macro_export]
macro_rules! codegen_allocator {
    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: $meta:tt,
        struct: (),
        methods: $methods:tt
    }) => ();

    ({
        type: class,
        rust_name: $rust_name:tt,
        ruby_name: $ruby_name:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: $struct:tt,
        methods: [ $($method:tt)* ]
    }) => (
        impl $rust_name {
            extern "C" fn __free__(this: *mut $crate::libc::c_void) {
                if !this.is_null() {
                    let _ = unsafe { Box::from_raw(this as *mut Self) };
                }
            }

            #[inline]
            fn __alloc_with__(rust_self: Option<Box<$rust_name>>) -> $crate::sys::VALUE {
                use ::std::mem::transmute;
                use ::std::ptr::null_mut;

                let ptr = rust_self
                    .map(Box::into_raw)
                    .unwrap_or_else(null_mut);

                unsafe {
                    $crate::sys::Data_Wrap_Struct(
                        transmute($rust_name),
                        None,
                        Some($rust_name::__free__),
                        ptr as *mut $crate::libc::c_void,
                    )
                }
            }
        }
    )
}
