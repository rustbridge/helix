#[macro_export]
macro_rules! declare_types {
    { $(#[$attr:meta])* pub class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(false)] #![pub(true)] $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(false)] #![pub(false)] $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* pub reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(true)] #![pub(true)] $(#[$attr])* pub class $cls { $($body)* } $($rest)* }
    };

    { $(#[$attr:meta])* reopen class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_class! { #![reopen(true)] #![pub(false)] $(#[$attr])* class $cls { $($body)* } $($rest)* }
    };

    { } => { };
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_struct {
    (true $(#[$attr:meta])* $cls:ident $($ident:ident : $ty:ty)*) => (
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        pub struct $cls {
            $($ident : $ty),*
        }
    );

    (false $(#[$attr:meta])* $cls:ident $($ident:ident : $ty:ty)*) => (
        #[derive(Copy, Clone, Debug)]
        #[repr(C)]
        $(#[$attr])*
        struct $cls {
            $($ident : $ty),*
        }
    );
}

#[doc(hidden)]
#[macro_export]
macro_rules! define_class {
    { #![reopen(false)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { struct { $($ident:ident : $ty:ty),* } def initialize() -> $initty:ty { $($initbody:tt)* } $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls $($ident : $ty),*);
        wrapped_class_definition! {
            $cls ;
            fn __initialize__() -> $cls { $($initbody:tt)* } ;
            () ;
            () ;
            $($body)*
        }
        declare_types! { $($rest)* }
    };

    { #![reopen(false)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls);
        class_definition! { $cls ; () ; () ; $($body)* }
        declare_types! { $($rest)* }
    };


    { #![reopen(true)] #![pub($is_pub:tt)] $(#[$attr:meta])* class $cls:ident { $($body:tt)* } $($rest:tt)* } => {
        define_struct!($(#[$attr:meta])* $is_pub $cls);
        reopen_class_definition! { $cls ; () ; () ; $($body)* }
        declare_types! { $($rest)* }
    };

}

#[doc(hidden)]
#[macro_export]
macro_rules! wrapped_class_definition {
    { $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; defn $name:ident ; $self_arg:tt ; ($($arg:ident : $argty:ty),*) ; $body:block ; $ret:ty ; $($rest:tt)* } => {
        wrapped_class_definition! {
            $cls ;
            $($initbody:tt)* ;
            ($($mimpl)* pub fn $name($self_arg, $($arg : $argty),*) -> $ret $body) ;
            ($($mdef)* {
                extern "C" fn __ruby_method__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let checked = __checked_call__(rb_self, $($arg),*);
                    match checked {
                        Ok(val) => $crate::ToRuby::to_ruby(val),
                        Err(err) => { println!("TYPE ERROR: {:?}", err); unsafe { $crate::sys::Qnil } }
                    }
                }

                fn __checked_call__(rb_self: $crate::sys::VALUE, $($arg : $crate::sys::VALUE),*) -> Result<$ret, ::std::ffi::CString> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    let rust_self = $cls::from_checked_rb_value(&mut rb_self);

                    $(
                        let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
                    )*

                    $(
                        let $arg = $crate::ToRust::to_rust($arg);
                    )*

                    Ok(rust_self.$name($($arg),*))
                }

                let name = stringify!($name);
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::new(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    { $cls:ident ; $($initbody:tt)* ; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        wrapped_class_definition! { $cls; $($initbody:tt)* ; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; { $($body)* } ; $ret ; $($rest)*  }
    };

    { $cls:ident ; $($initbody:tt)* ; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        wrapped_class_definition! { $cls; $($initbody:tt)* ; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    { $cls:ident ; $($initbody:tt)* ; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        wrapped_class_definition! { $cls; $($initbody:tt)* ; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; { $($body)* } ; $ret ; $($rest)*  }
    };

    { $cls:ident ; $($initbody:tt)* ; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) $body:block $($rest:tt)* } => {
        wrapped_class_definition! { $cls; $($initbody:tt)* ; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    ( $cls:ident ; $($initbody:tt)* ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; ) => {
        item! {
            impl $cls {
                fn from_checked_rb_value(value: &mut $crate::sys::VALUE) -> &mut $cls {
                    unsafe { std::mem::transmute($crate::sys::Data_Get_Struct_Value(rb_self)) }
                }


                $($mimpl)*
            }
        }

        init! {
            extern "C" fn __free__(klass: Option<Box<$cls>>) {}

            extern "C" fn __alloc__(klass: $crate::sys::VALUE) -> $crate::sys::VALUE {
                let instance = $crate::sys::Data_Wrap_Struct(klass, ::std::ptr::null(), __free__, ::std::ptr::null());

                // FIXME: this should really be called during Ruby's initialize, with arguments
                __initialize__(instance);

                instance
            }

            extern "C" fn __initialize__(rb_self: $crate::sys::VALUE) {
                let data = Box::new($cls::___initialize());
                ::std::mem::forget(data);
                $crate::sys::Data_Set_Struct_Value(rb_self, ::std::mem::transmute(data));
            }


            $crate::ClassDefinition::wrapped(stringify!($cls))$(.define_method($mdef))*;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! class_definition {
    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; defn $name:ident ; $self_arg:tt ; ($($arg:ident : $argty:ty),*) ; $body:block ; $ret:ty ; $($rest:tt)* } => {
        class_definition! {
            $cls ;
            ($($mimpl)* pub fn $name($self_arg, $($arg : $argty),*) -> $ret $body) ;
            ($($mdef)* {
                extern "C" fn __ruby_method__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let checked = __checked_call__(rb_self, $($arg),*);
                    match checked {
                        Ok(val) => $crate::ToRuby::to_ruby(val),
                        Err(err) => { println!("TYPE ERROR: {:?}", err); unsafe { $crate::sys::Qnil } }
                    }
                }

                fn __checked_call__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> Result<$ret, ::std::ffi::CString> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    $(
                        let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
                    )*

                    $(
                        let $arg = $crate::ToRust::to_rust($arg);
                    )*

                    Ok(rb_self.$name($($arg),*))
                }

                let name = stringify!($name);
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::new(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; { $($body)* }; $ret ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; { $($body)* } ; $ret ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) $body:block $($rest:tt)* } => {
        class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    ( $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; ) => {
        item! {
            impl $cls {
                $($mimpl)*
            }
        }

        init! {
            $crate::ClassDefinition::new(stringify!($cls))$(.define_method($mdef))*;
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! reopen_class_definition {
    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; defn $name:ident ; $self_arg:tt ; ($($arg:ident : $argty:ty),*) ; $body:block ; $ret:ty ; $($rest:tt)* } => {
        reopen_class_definition! {
            $cls ;
            ($($mimpl)* pub fn $name($self_arg, $($arg : $argty),*) -> $ret $body) ;
            ($($mdef)* {
                extern "C" fn __ruby_method__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> $crate::sys::VALUE {
                    let checked = __checked_call__(rb_self, $($arg),*);
                    match checked {
                        Ok(val) => $crate::ToRuby::to_ruby(val),
                        Err(err) => { println!("TYPE ERROR: {:?}", err); unsafe { $crate::sys::Qnil } }
                    }
                }

                fn __checked_call__(rb_self: $cls, $($arg : $crate::sys::VALUE),*) -> Result<$ret, ::std::ffi::CString> {
                    #[allow(unused_imports)]
                    use $crate::{ToRust};

                    $(
                        let $arg = try!($crate::UncheckedValue::<$argty>::to_checked($arg));
                    )*

                    $(
                        let $arg = $crate::ToRust::to_rust($arg);
                    )*

                    Ok(rb_self.$name($($arg),*))
                }

                let name = stringify!($name);
                let arity = method_arity!($($arg),*);
                let method = __ruby_method__ as *const $crate::libc::c_void;

                $crate::MethodDefinition::new(name, method, arity)
            }) ;
            $($rest)*
        }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        reopen_class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; { $($body)* } ; $ret ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt , $($arg:ident : $argty:ty),* ) $body:block $($rest:tt)* } => {
        reopen_class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; ($($arg : $argty),*) ; $body ; () ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) -> $ret:ty { $($body:tt)* } $($rest:tt)* } => {
        reopen_class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; { $($body)* } ; $ret ; $($rest)*  }
    };

    { $cls:ident; ($($mimpl:tt)*) ; ($($mdef:tt)*) ; def $name:ident( $self_arg:tt ) $body:block $($rest:tt)* } => {
        reopen_class_definition! { $cls; ($($mimpl)*) ; ($($mdef)*) ; defn $name ; $self_arg ; () ; $body ; () ; $($rest)*  }
    };

    ( $cls:ident ; ($($mimpl:tt)*) ; ($($mdef:block)*) ; ) => {
        item! {
            impl $cls {
                $($mimpl)*
            }
        }

        init! {
            $crate::ClassDefinition::reopen(stringify!($cls))$(.define_method($mdef))*;
        }
    };
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
