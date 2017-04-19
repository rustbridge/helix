#[macro_export]
macro_rules! declare_types {
    // pub class Name { ... }
    { $(#[$attr:meta])* pub class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(false)] #![pub(true)] $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    // class Name { ... }
    { $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(false)] #![pub(false)] $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    // pub reopen class Name { ... }
    { $(#[$attr:meta])* pub reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(true)] #![pub(true)] $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    // reopen class Name { ... }
    { $(#[$attr:meta])* reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(true)] #![pub(false)] $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    { } => { };
}

#[macro_export]
macro_rules! throw {
    ($msg:expr) => {
        panic!($crate::ExceptionInfo::with_message(String::from($msg)))
    }
}

// TODO: Can we change this to use the macro from libcruby?
#[macro_export]
macro_rules! ruby_funcall {
    // NOTE: Class and method cannot be variables. If that becomes necessary, I think we'll have to pass them
    ($rb_class:expr, $meth:expr, $( $arg:expr ),*) => {
        {
            use $crate::ToRuby;

            // This method takes a Ruby Array of arguments
            // If there is a way to make this behave like a closure, we could further simplify things.
            #[allow(unused_variables)]
            extern "C" fn __ruby_funcall_cb(arg_ary: $crate::sys::VALUE) -> $crate::sys::VALUE {
                unsafe {
                    // NOTE: We're using rb_intern_str, not rb_intern in the hopes that this means
                    //   Ruby will clean up the string in the event that there is an exception
                    $crate::sys::rb_funcallv($rb_class, sys::rb_intern_str(String::from($meth).to_ruby()),
                                                $crate::sys::RARRAY_LEN(arg_ary), $crate::sys::RARRAY_PTR(arg_ary))
                }
            }

            let mut state = $crate::sys::EMPTY_EXCEPTION;

            let res = unsafe {
                let mut arg_ary = Vec::new();
                $(
                    // We have to create this iteratively since we have to call to_ruby individually
                    arg_ary.push($arg.to_ruby());
                )*
                let arg_ary = $crate::sys::rb_ary_new_from_values(arg_ary.len() as isize, arg_ary.as_mut_ptr());
                $crate::sys::rb_protect(__ruby_funcall_cb, arg_ary, &mut state)
            };

            if !state.is_empty() {
                panic!($crate::ExceptionInfo::from_state(state));
            }

            res
        }
    };

    ($rb_class:expr, $meth:expr) => {
        ruby_funcall!($rb_class, $meth, )
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_struct {
    // pub struct Name { ... }
    (true $(#[$attr:meta])* $cls:ident $($fields:tt)*) => (
        #[derive(Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        pub struct $cls {
            helix: $crate::Metadata,
            $($fields)*
        }
    );

    // struct Name { ... }
    (false $(#[$attr:meta])* $cls:ident $($fields:tt)*) => (
        #[derive(Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        struct $cls {
            helix: $crate::Metadata,
            $($fields)*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_class {
    // no reopen, with initializer and args
    { #![reopen(false)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { struct { $($fields:tt)* } def initialize($helix:ident, $($args:tt)*) { $($initbody:tt)* } $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls $($fields)*);
        class_definition! { #![reopen(false)] #![struct(true)] $cls ; () ; () ; $($body)* fn initialize($helix, $($args)*) { $($initbody)* } }
        declare_types! { $($rest)* }
    };

    // no reopen, with initializer and no args
    { #![reopen(false)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { struct { $($fields:tt)* } def initialize($helix:ident) { $($initbody:tt)* } $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls $($fields)*);
        class_definition! { #![reopen(false)] #![struct(true)] $cls ; () ; () ; $($body)* fn initialize($helix,) { $($initbody)* } }
        declare_types! { $($rest)* }
    };

    // no reopen, without initializer
    { #![reopen(false)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls);
        class_definition! { #![reopen(false)] #![struct(false)] $cls ; () ; () ; $($body)* () }
        declare_types! { $($rest)* }
    };

    // reopen, without initializer
    { #![reopen(true)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls);
        class_definition! { #![reopen(true)] #![struct(false)] $cls ; () ; () ; $($body)* () }
        declare_types! { $($rest)* }
    };

}

#[doc(hidden)]
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

#[doc(hidden)]
#[macro_export]
macro_rules! class_definition {
    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; defi $name:ident ; { $($alt_mod:tt)* } ; { $($self_mod:tt)* } ; $self_arg:tt ; ($($arg:ident : $argty:ty),*) ; $body:block ; $ret:ty ; $($rest:tt)* } => {
        class_definition! {
            #![reopen($reopen)]
            #![struct($has_struct)]
            $cls ;
            ($($mimpl)* pub fn $name($($self_mod)* $self_arg, $($arg : $argty),*) -> $ret $body) ;
            ($($mdef)* {
                use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise, rb_jump_tag};

                #[repr(C)]
                struct CallResult {
                    error_klass: VALUE,
                    ruby_exception: $crate::sys::RubyException,
                    value: VALUE
                }

                extern "C" fn __ruby_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let result = __rust_method__(rb_self, $($arg),*);

                    if result.error_klass != unsafe { Qnil } {
                        unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
                    } else if !result.ruby_exception.is_empty() {
                        unsafe { rb_jump_tag(result.ruby_exception) }
                    } else {
                        result.value
                    }
                }

                #[inline]
                fn __rust_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> CallResult {
                    let checked = __checked_call__(rb_self, $($arg),*);

                    match checked {
                        Ok(val) => CallResult { error_klass: unsafe { Qnil }, ruby_exception: $crate::sys::EMPTY_EXCEPTION, value: $crate::ToRuby::to_ruby(val) },
                        Err(err) => CallResult { error_klass: err.exception(), ruby_exception: err.ruby_exception(), value: err.message() }
                    }
                }

                #[inline]
                fn __checked_call__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$ret, $crate::ExceptionInfo> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    let rust_self = match $crate::UncheckedValue::<$($alt_mod)* $cls>::to_checked(rb_self) {
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
                        rust_self.$name($($arg),*)
                    }
                }

                let name = cstr!(stringify!($name));
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::instance(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; defs $name:ident ; ($($arg:ident : $argty:ty),*) ; $body:block ; $ret:ty ; $($rest:tt)* } => {
        class_definition! {
            #![reopen($reopen)]
            #![struct($has_struct)]
            $cls ;
            ($($mimpl)* pub fn $name($($arg : $argty),*) -> $ret $body) ;
            ($($mdef)* {
                use $crate::sys::{VALUE, SPRINTF_TO_S, Qnil, rb_raise, rb_jump_tag};

                #[repr(C)]
                struct CallResult {
                    error_klass: VALUE,
                    ruby_exception: $crate::sys::RubyException,
                    value: VALUE
                }

                extern "C" fn __ruby_method__(_: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let result = __rust_method__($($arg),*);

                    if result.error_klass != unsafe { Qnil } {
                        unsafe { rb_raise(result.error_klass, SPRINTF_TO_S, result.value) }
                    } else if !result.ruby_exception.is_empty() {
                        unsafe { rb_jump_tag(result.ruby_exception) }
                    } else {
                        result.value
                    }
                }

                #[inline]
                fn __rust_method__($($arg : $crate::sys::VALUE),*) -> CallResult {
                    let checked = __checked_call__($($arg),*);

                    match checked {
                        Ok(val) => CallResult { error_klass: unsafe { Qnil }, ruby_exception: $crate::sys::EMPTY_EXCEPTION, value: $crate::ToRuby::to_ruby(val) },
                        Err(err) => CallResult { error_klass: err.exception(), ruby_exception: err.ruby_exception(), value: err.message() }
                    }
                }

                #[inline]
                fn __checked_call__($($arg : $crate::sys::VALUE),*) -> Result<$ret, $crate::ExceptionInfo> {
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
                        $cls::$name($($arg),*)
                    }
                }

                let name = cstr!(stringify!($name));
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::class(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    // def ident(&self, ...args) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { & } ; { & } ; $self_arg ; ($($arg : $argty),*) ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&self, ...args) { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { & } ; { & } ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    // def ident(&self) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { & } ; { & } ; $self_arg ; () ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&self) { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { & } ; { & } ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    // def ident(&mut self, ...args) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { &mut } ; { &mut } ; $self_arg ; ($($arg : $argty),*) ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&mut self, ...args) { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { &mut } ; { &mut } ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    // def ident(&mut self) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { &mut } ; { &mut } ; $self_arg ; () ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&mut self) { ... }
    { #![reopen($reopen:tt)] #![struct(true)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(true)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { &mut } ; { &mut } ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    // def ident(&self, ...args) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { & } ; $self_arg ; ($($arg : $argty),*) ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&self, ...args) { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { & } ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    // def ident(&self) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { & } ; $self_arg ; () ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&self) { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( & $self_arg:tt ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { & } ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    // def ident(&mut self, ...args) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { &mut } ; $self_arg ; ($($arg : $argty),*) ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&mut self, ...args) { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { &mut } ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    // def ident(&mut self) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { &mut } ; $self_arg ; () ; $body ; $ret ; $($rest)*  }
    };

    // def ident(&mut self) { ... }
    { #![reopen($reopen:tt)] #![struct(false)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( &mut $self_arg:tt ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct(false)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defi $name ; { } ; { &mut } ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    // def ident(...args) -> ty { ... }
    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $($arg:ident : $argty:ty),* ) -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct($has_struct)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defs $name ; ($($arg : $argty),*) ; $body ; $ret ; $($rest)*  }
    };

    // def ident(...args) { ... }
    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct($has_struct)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defs $name ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    // def ident() -> ty { ... }
    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident() -> $ret:ty $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct($has_struct)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defs $name ; () ; $body ; $ret ; $($rest)*  }
    };

    // def ident() { ... }
    { #![reopen($reopen:tt)] #![struct($has_struct:tt)] $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident() $body:block $($rest:tt)* } => {
        class_definition! { #![reopen($reopen)] #![struct($has_struct)] $cls; ($($mimpl)*) ; ($($mdef)*) ; defs $name ; () ; $body ; () ; $($rest)*  }
    };

    ( #![reopen(false)] #![struct(true)] $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; fn initialize($helix:ident, $($arg:ident : $argty:ty),*) { $($initbody:tt)* } ) => {
        item! {
            impl $cls {
                pub fn new($($arg : $argty),*) -> $cls {
                    $cls::initialize(unsafe { $crate::sys::Qnil } $(, $arg)*)
                }

                fn initialize($helix: $crate::Metadata, $($arg : $argty),*) -> $cls {
                    $($initbody)*
                }

                $($mimpl)*
            }
        }

        impl_struct_to_rust!(&'a $cls);
        impl_struct_to_rust!(&'a mut $cls);

        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);

        static mut __HELIX_ID: usize = 0;

        init! {
            extern "C" fn __mark__(_klass: &$cls) {}
            extern "C" fn __free__(_klass: Option<Box<$cls>>) {}

            extern "C" fn __alloc__(_klass: $crate::sys::VALUE) -> $crate::sys::VALUE {
                __alloc_with__(None)
            }

            fn __alloc_with__(rust_self: Option<Box<$cls>>) -> $crate::sys::VALUE {
                use ::std::mem::transmute;

                unsafe {
                    let instance = $crate::sys::Data_Wrap_Struct(
                        transmute(__HELIX_ID),
                        transmute(__mark__ as usize),
                        transmute(__free__ as usize),
                        transmute(rust_self)
                    );

                    instance
                }
            }

            impl $crate::ToRuby for $cls {
                fn to_ruby(self) -> $crate::sys::VALUE {
                    __alloc_with__(Some(Box::new(self)))
                }
            }

            let def_initialize = {
                extern "C" fn __initialize__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let result = __checked_initialize__(rb_self $(, $arg)*);

                    match result {
                        Ok(rust_self) => {
                            let data = Box::new(rust_self);
                            unsafe { $crate::sys::Data_Set_Struct_Value(rb_self, ::std::mem::transmute(data)) };
                        }
                        Err(err) => { println!("TYPE ERROR: {:?}", err); }
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

                let arity = method_arity!($($arg),*);
                let method = __initialize__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::instance(cstr!("initialize"), method, arity)
            };

            let def = $crate::ClassDefinition::wrapped(cstr!(stringify!($cls)), __alloc__)
                .define_method(def_initialize)
                $(.define_method($mdef))*;

            unsafe { __HELIX_ID = ::std::mem::transmute(def.class) };
        }
    };

    ( #![reopen(false)] #![struct(false)] $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; () ) => {
        impl_simple_class!( $cls ; ($($mimpl)*) );

        static mut __HELIX_ID: usize = 0;

        init! {
            let def = $crate::ClassDefinition::new(cstr!(stringify!($cls)))$(.define_method($mdef))*;
            unsafe { __HELIX_ID = ::std::mem::transmute(def.class) };
        }
    };

    ( #![reopen(true)] #![struct(false)] $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; () ) => {
        impl_simple_class!( $cls ; ($($mimpl)*) );

        static mut __HELIX_ID: usize = 0;

        init! {
            let def = $crate::ClassDefinition::reopen(cstr!(stringify!($cls)))$(.define_method($mdef))*;
            unsafe { __HELIX_ID = ::std::mem::transmute(def.class) };
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_struct_to_rust {
    ($cls:ty) => {
        item! {
            impl<'a> $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
                fn to_rust(self) -> $cls {
                    unsafe { ::std::mem::transmute($crate::sys::Data_Get_Struct_Value(self.inner)) }
                }
            }
        }

        item! {
            impl<'a> $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
                fn to_checked(self) -> $crate::CheckResult<$cls> {
                    use $crate::{CheckedValue, sys};
                    use ::std::ffi::{CStr};

                    if unsafe { __HELIX_ID == ::std::mem::transmute(sys::rb_obj_class(self)) } {
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
}

#[doc(hidden)]
#[macro_export]
macro_rules! impl_simple_class {
    ( $cls:ident ; ($($mimpl:tt)*) ) => {
        item! {
            impl $cls {
                $($mimpl)*
            }
        }

        item! {
            impl $crate::UncheckedValue<$cls> for $crate::sys::VALUE {
                fn to_checked(self) -> $crate::CheckResult<$cls> {
                    use $crate::{CheckedValue, sys};
                    use ::std::ffi::{CStr};

                    if unsafe { __HELIX_ID == ::std::mem::transmute(sys::rb_obj_class(self)) } {
                        Ok(unsafe { CheckedValue::new(self) })
                    } else {
                        let val = unsafe { CStr::from_ptr(sys::rb_obj_classname(self)).to_string_lossy() };
                        Err(format!("No implicit conversion of {} into {}", val, stringify!($cls)))
                    }
                }
            }
        }

        item! {
            impl $crate::ToRust<$cls> for $crate::CheckedValue<$cls> {
                fn to_rust(self) -> $cls {
                    $cls { helix: self.inner }
                }
            }
        }

        impl_to_ruby!($cls);
        impl_to_ruby!(&'a $cls);
        impl_to_ruby!(&'a mut $cls);
    }
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
macro_rules! init {
    { $($body:tt)* } => {
        #[allow(non_snake_case)]
        #[no_mangle]
        pub extern "C" fn Init_native() {
            $crate::sys::check_version();
            $($body)*
        }
    }
}

#[macro_export]
macro_rules! method {
    ( $name:ident( $($args:ident),* ) { $($block:stmt;)* } ) => {
        #[no_mangle]
        pub extern "C" fn $name(rb_self: $crate::sys::VALUE, $($args : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
            $($block;)*
            $crate::sys::Qnil
        }
    }
}

#[doc(hidden)]
#[macro_export]
macro_rules! item {
    ($it: item) => { $it }
}

#[doc(hidden)]
#[macro_export]
macro_rules! replace_expr {
    ($_t:tt $sub:expr) => {$sub};
}

#[doc(hidden)]
#[macro_export]
macro_rules! method_arity {
  ( $($id:pat ),* ) => {
    { 0isize $(+ replace_expr!($id 1isize))* }
  }
}

// This macro is copied instead of depended upon because of https://github.com/rust-lang/rust/issues/29638

#[doc(hidden)]
#[macro_export]
macro_rules! cstr {
    ($s:expr) => (
        concat!($s, "\0") as *const str as *const [::std::os::raw::c_char] as *const ::std::os::raw::c_char
    )
}

#[macro_export]
macro_rules! rb_sprintf {
    ($s:tt , $($params:expr),+) => {
        $crate::sys::rb_sprintf(rb_sprintf_specifier!($s).as_ptr(), $($params),*)
    };

    ($s:tt) => {
        $crate::sys::rb_sprintf(rb_sprintf_specifier!($s).as_ptr())
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! rb_sprintf_specifier {
    ({ $s:tt $($sep:ident $s2:tt)+ $sep2:ident }) => {
        {
            let s = format!(
                concat!("{}", $("{}{}"),*),
                $s,
                $(
                    unsafe { CStr::from_ptr( $crate::sys:: $sep ).to_string_lossy() },
                    $s2
                ),* ,
                unsafe { CStr::from_ptr( $crate::sys:: $sep2 ).to_string_lossy() }
            );
            CString::new(s).unwrap()
        }
    };

    ({ $s:tt $($sep:ident $s2:tt)* }) => {
        {
            use ::std::ffi::CStr;
            let s = format!(
                concat!("{}", $("{}{}"),*),
                $s,
                $(
                    unsafe { CStr::from_ptr( $crate::sys:: $sep ).to_string_lossy() },
                    $s2
                ),*
            );
            CString::new(s).unwrap()
        }
    };

    ({ $s:tt $sep:ident }) => {
        {
            use ::std::ffi::CStr;
            let s = format!( "{}{}", $s, unsafe { CStr::from_ptr( $crate::sys:: $sep ).to_string_lossy() });
            CString::new(s).unwrap()
        }
    };
}
