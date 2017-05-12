#[macro_export]
macro_rules! codegen_init {
    { [ $($class:tt)* ] } => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn Init_native() {
            $crate::sys::check_version();


            $(
                codegen_class_binding!($class, $class);
            )*
        }
    }
}

#[macro_export]
macro_rules! codegen_class_binding {
    { $class:tt, {
        type: class,
        name: $name:tt,
        meta: { pub: $pub:tt, reopen: false },
        struct: (),
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;
        let def = $crate::ClassDefinition::new(cstr!(stringify!($name)));

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $name = transmute::<_, usize>(def.class) };
    });

    { $class:tt, {
        type: class,
        name: $name:tt,
        meta: { pub: $pub:tt, reopen: true },
        struct: (),
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;
        let def = $crate::ClassDefinition::reopen(cstr!(stringify!($name)));

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $name = transmute::<_, usize>(def.class) };
    });

    { $class:tt, {
        type: class,
        name: $cls:tt,
        meta: { pub: $pub:tt, reopen: $reopen:tt },
        struct: { $($struct:tt)* },
        methods: [ $($method:tt)* ]
    } } => ({
        use ::std::mem::transmute;

        extern "C" fn __alloc__(_klass: $crate::sys::VALUE) -> $crate::sys::VALUE {
            $cls::__alloc_with__(None)
        }

        let def = $crate::ClassDefinition::wrapped(cstr!(stringify!($cls)), __alloc__);

        $(
            codegen_define_method!(def, $class, $method);
        )*

        unsafe { $cls = transmute::<_, usize>(def.class) }
    });

}

#[macro_export]
macro_rules! codegen_define_method {
    ($def:tt, {
        type: class,
        name: $cls:tt,
        $($rest:tt)*
    }, {
        type: class_method,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: (),
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise};

        #[repr(C)]
        #[derive(Debug)]
        struct CallResult {
            error_klass: VALUE,
            value: VALUE
        }

        extern "C" fn __ruby_method__(_: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __rust_method__($($arg),*);

            if result.error_klass == unsafe { Qnil } {
                result.value
            } else {
                unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
            }
        }

        #[inline]
        fn __rust_method__($($arg : $crate::sys::VALUE),*) -> CallResult {
            let checked = __checked_call__($($arg),*);

            match checked {
                Ok(val) => CallResult { error_klass: unsafe { Qnil }, value: $crate::ToRuby::to_ruby(val) },
                Err(err) => CallResult { error_klass: err.exception.inner(), value: err.message }
            }
        }

        #[inline]
        fn __checked_call__($($arg : $crate::sys::VALUE),*) -> Result<$($ret)*, $crate::ExceptionInfo> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            $(
                let $arg = match $crate::UncheckedValue::<$argty>::to_checked($arg) {
                    Ok(v) => v,
                    Err(e) => return Err($crate::ExceptionInfo::type_error(e))
                };
            )*

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            handle_exception! {
                $cls::$rust_name($($arg),*)
            }
        }

        let name = cstr!($($ruby_name)*);
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::class(name, method, arity));
    });

    ($def:tt, {
        type: class,
        name: $cls:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: instance_method,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: { ownership: { $($ownership:tt)* }, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise};

        #[repr(C)]
        struct CallResult {
            error_klass: VALUE,
            value: VALUE
        }

        extern "C" fn __ruby_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __rust_method__(rb_self, $($arg),*);

            if result.error_klass == unsafe { Qnil } {
                result.value
            } else {
                unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
            }
        }

        #[inline]
        fn __rust_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> CallResult {
            let checked = __checked_call__(rb_self, $($arg),*);

            match checked {
                Ok(val) => CallResult { error_klass: unsafe { Qnil }, value: $crate::ToRuby::to_ruby(val) },
                Err(err) => CallResult { error_klass: err.exception.inner(), value: err.message }
            }
        }

        #[inline]
        fn __checked_call__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$($ret)*, $crate::ExceptionInfo> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            let rust_self = match $crate::UncheckedValue::<codegen_self_pointer_type! { struct: $struct, ownership: { $($ownership)* }, type: $cls }>::to_checked(rb_self) {
                Ok(v)  => v,
                Err(e) => return Err($crate::ExceptionInfo::with_message(e))
            };

            $(
                let $arg = match $crate::UncheckedValue::<$argty>::to_checked($arg) {
                    Ok(v) => v,
                    Err(e) => return Err($crate::ExceptionInfo::type_error(e))
                };
            )*

            let rust_self = rust_self.to_rust();

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            handle_exception! {
                rust_self.$rust_name($($arg),*)
            }
        }

        let name = cstr!($($ruby_name)*);
        let method = __ruby_method__ as *const $crate::libc::c_void;
        let arity = method_arity!($($arg)*);

        $def.define_method($crate::MethodDefinition::instance(name, method, arity))
    });

    ($def:tt, {
        type: class,
        name: $cls:tt,
        meta: $meta:tt,
        struct: $struct:tt,
        $($rest:tt)*
    }, {
        type: initializer,
        rust_name: $rust_name:tt,
        ruby_name: { $($ruby_name:tt)* },
        self: { ownership: {}, name: $self:tt },
        args: [ $($arg:tt : $argty:ty),* ],
        ret: { $($ret:tt)* },
        body: $body:tt
    }) => ({
        impl $cls {
            pub fn new($($arg : $argty),*) -> $($ret)* {
                $cls::$rust_name(unsafe { $crate::sys::Qnil } , $($arg),*)
            }
        }

        extern "C" fn __initialize__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            let result = __checked_initialize__(rb_self $(, $arg)*);

            match result {
                Ok(rust_self) => {
                    let data = Box::new(rust_self);
                    unsafe { $crate::sys::Data_Set_Struct_Value(rb_self, ::std::mem::transmute(data)) };
                }
                Err(e) => $crate::ExceptionInfo::type_error(e).raise()
            }

            rb_self
        }

        fn __checked_initialize__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$cls, String> {
            #[allow(unused_imports)]
            use $crate::{ToRust};

            $(
                let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
            )*

            $(
                let $arg = $crate::ToRust::to_rust($arg);
            )*

            Ok($cls::initialize(rb_self, $($arg),*))
        }

        let arity = method_arity!($($arg)*);
        let method = __initialize__ as *const $crate::libc::c_void;

        $def.define_method($crate::MethodDefinition::instance(cstr!($($ruby_name)*), method, arity));
    });
}


#[macro_export]
macro_rules! codegen_self_pointer_type {
    {
        struct: (),
        ownership: $ownership:tt,
        type: $type:tt
    } => {
        $type
    };

    {
        struct: $struct:tt,
        ownership: { $($ownership:tt)* },
        type: $type:tt
    } => {
        $($ownership)* $type
    };
}

#[macro_export]
macro_rules! method_arity {
  ( $($arg:tt)* ) => {
    { 0isize $(+ replace_expr!($arg 1isize))* }
  }
}

#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char] as *const ::std::os::raw::c_char
    )
}

#[macro_export]
macro_rules! handle_exception {
    { $($body:tt)* } => {
        let hide_err = ::std::env::var("RUST_BACKTRACE").is_err();
        if hide_err {
            ::std::panic::set_hook(Box::new(|_| {
                // Silence
            }));
        }

        let res = ::std::panic::catch_unwind(|| {
            $($body)*
        });

        if hide_err {
            let _ = ::std::panic::take_hook();
        }

        res.map_err(|e| $crate::ExceptionInfo::from_any(e))
    }
}
