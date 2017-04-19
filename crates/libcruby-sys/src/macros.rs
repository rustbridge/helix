// TODO: See if we can simplify
#[doc(hidden)]
#[macro_export]
macro_rules! ruby_extern_fns {
    // We don't need the body here
    { #[$attr:meta] $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty { $($_body:tt)* } $($rest:tt)* } => {
        ruby_extern_fns! { #[$attr] $name($( $argn: $argt ),*) -> $ret; $($rest)* } };

    { #[$attr:meta] $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty; $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" {
            #[$attr]
            pub fn $name($($argn: $argt),*) -> $ret;
        }
        ruby_extern_fns! { $($rest)* }
    };
    { #[$attr:meta] $name:ident($( $argn:ident: $argt:ty ),*); $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" {
            #[$attr]
            pub fn $name($($argn: $argt),*);
        }
        ruby_extern_fns! { $($rest)* }
    };

    // We don't need the body here
    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty { $($_body:tt)* } $($rest:tt)* } => {
        ruby_extern_fns! { $name($( $argn: $argt ),*) -> $ret; $($rest)* } };

    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty; $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" { pub fn $name($($argn: $argt),*) -> $ret; }
        ruby_extern_fns! { $($rest)* }
    };
    { $name:ident($( $argn:ident: $argt:ty ),*); $($rest:tt)* } => {
        #[cfg_attr(windows, link(name="helix-runtime"))]
        extern "C" { pub fn $name($($argn: $argt),*); }
        ruby_extern_fns! { $($rest)* }
    };

    { } => ()
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_fn {
    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty { $($funcs:tt)+ } } => {
        pub fn $name($( $argn: $argt ),*) -> Result<$ret, $crate::RubyException> {
            // FIXME: Avoid creating args struct if there are no args
            #[repr(C)]
            #[derive(Copy, Clone, Debug)]
            struct Args {
                pub $($argn: $argt),*
            };

            let args = Args { $($argn: $argn),* };

            // Must include ret_to_ptr and ptr_to_ret
            $($funcs)+

            extern "C" fn cb(args_ptr: *mut $crate::void) -> *mut $crate::void {
                let ret = unsafe {
                    let args: &Args = &*(args_ptr as *const Args);
                    $crate::$name($( args.$argn ),*)
                };
                ret_to_ptr(ret)
            }

            let mut state = $crate::EMPTY_EXCEPTION;

            let res = unsafe {
                let args_ptr: *mut $crate::void = &args as *const _ as *mut $crate::void;
                $crate::rb_protect(cb, args_ptr, &mut state)
            };

            if !state.is_empty() {
                Err(state)
            } else {
                Ok(ptr_to_ret(res))
            }
        }
    };
}

#[doc(hidden)]
#[macro_export]
macro_rules! ruby_safe_fns {
    // We don't need the meta here
    { #[$attr:meta] $($rest:tt)* } => {
        ruby_safe_fns! { $($rest)* }
    };

    // It's not quite ideal to have to define each type separately, but the coercions are different
    { $name:ident($( $argn:ident: $argt:ty ),*) -> VALUE; $($rest:tt)* } => {
        ruby_safe_fn! {
            $name($( $argn: $argt ),*) -> $crate::VALUE {
                fn ret_to_ptr(ret: $crate::VALUE) -> *mut $crate::void { ret.as_ptr() }
                fn ptr_to_ret(ptr: *mut $crate::void) -> $crate::VALUE { $crate::VALUE::wrap(ptr) }
            }
        }

        ruby_safe_fns! { $($rest)* }
    };

    { $name:ident($( $argn:ident: $argt:ty ),*) -> $ret:ty { $($conv:tt)* } $($rest:tt)* } => {
        ruby_safe_fn! {
            $name($( $argn: $argt ),*) -> $ret { $($conv)* }
        }

        ruby_safe_fns! { $($rest)* }
    };

    { $name:ident($( $argn:ident: $argt:ty ),*); $($rest:tt)* } => {
        ruby_safe_fn! {
            $name($( $argn: $argt ),*) -> () {
                fn ret_to_ptr(_: ()) -> *mut $crate::void { unsafe { $crate::Qnil }.as_ptr() }
                fn ptr_to_ret(_: *mut $crate::void) { }
            }
        }

        ruby_safe_fns! { $($rest)* }
    };

    { } => ()
}

#[macro_export]
macro_rules! ruby_safe_c {
    { $($parts:tt)+ } => {
        ruby_extern_fns! {
            $($parts)+
        }

        pub mod safe {
            use $crate::*;

            ruby_safe_fns! {
                $($parts)+
            }
        }
    }
}