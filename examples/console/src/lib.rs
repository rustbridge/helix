#[macro_use]
extern crate helix;

declare_types! {
    class Console {
        def alt_new() -> helix::sys::VALUE {
            let args: &[helix::sys::VALUE] = &[];
            Console::ruby_new(args)
        }

        def helix_version() -> String {
            String::from(helix::sys::PKG_VERSION)
        }

        def log(&self, string: String) {
            println!("{}", string);
        }

        def inspect(&self) {
            println!("{:?}", self)
        }

        def hello(&self) {
            self.log(String::from("hello"));
        }

        def loglog(&self, string1: String, string2: String) {
            println!("{} {}", string1, string2);
        }

        def log_if(&self, string: String, condition: bool) {
            if condition { self.log(string) };
        }

        def colorize(&self, string: String) -> String {
            format!("\x1B[0;31;49m{}\x1B[0m", string)
        }

        def is_red(&self, string: String) -> bool {
            string.starts_with("\x1B[0;31;49m") && string.ends_with("\x1B[0m")
        }
    }
}






















































// macro_rules! cstr {
//     ( $x: expr ) => { $x.as_ptr() as *const i8 }
// }


// extern "C" fn log(_: VALUE, message: VALUE) -> VALUE {
//     #[repr(C)]
//     struct CheckTypeArgs {
//         value: VALUE,
//         rb_type: isize,
//     }

//     extern "C" fn CheckType(args: &CheckTypeArgs) -> VALUE {
//         unsafe { rb_check_type(args.value, args.rb_type) };
//         Qnil
//     }

//     let result = std::panic::catch_unwind(|| {
//         with_protect(CheckType,
//                      &CheckTypeArgs {
//                          value: message,
//                          rb_type: T_STRING,
//                      });
//     });

//     if let Err(state) = result {
//         let state = state.downcast_ref::<RubyException>().unwrap();
//         unsafe { rb_jump_tag(*state) };
//     } else {
//         if unsafe { RB_TYPE_P(message, T_STRING) } {
//             let size = unsafe { RSTRING_LEN(message) };
//             let ptr = unsafe { RSTRING_PTR(message) };
//             let slice = unsafe { std::slice::from_raw_parts(ptr as *const u8, size as usize) };
//             let string = unsafe { std::str::from_utf8_unchecked(slice) };
//             println!("size: {}", size);
//             println!("ptr: {:?}", ptr);
//             println!("string: {}", string);
//             Qtrue
//         } else {
//             Qfalse
//         }
//     }

// }

// fn with_protect<T>(func: extern "C" fn(&T) -> VALUE, arg: &T) {
//     let mut state: RubyException = RubyException::new();
//     let arg: void_ptr = unsafe { mem::transmute(arg) };
//     let func: extern "C" fn(void_ptr) -> VALUE = unsafe { mem::transmute(func) };

//     unsafe { rb_protect(func, arg, &mut state as *mut RubyException) };

//     if state == RubyException::new() {
//         println!("IT WORKED");
//     } else {
//         panic!(state);
//     }
// }
