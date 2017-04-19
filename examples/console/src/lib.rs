#[macro_use]
extern crate helix_runtime as helix;

use helix::{sys,UncheckedValue,ToRust};

declare_types! {
    class Console {
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

        def freak_out(&self) {
            throw!("Aaaaahhhhh!!!!!");
        }

        def behave_badly(&self) {
            ruby_funcall!(sys::rb_cObject, "does_not_exist", String::from("one"));
        }

        def call_ruby(&self) -> String {
            let a = ruby_funcall!(sys::rb_cObject, "name"); // No arg
            let b = ruby_funcall!(sys::rb_cObject, "is_a?", sys::rb_cObject); // One arg
            let c = ruby_funcall!(sys::rb_cObject, "respond_to?", String::from("inspect"), true); // Two args
            format!("{:?}, {:?}, {:?}", UncheckedValue::<String>::to_checked(a).unwrap().to_rust(),
                                        UncheckedValue::<bool>::to_checked(b).unwrap().to_rust(),
                                        UncheckedValue::<bool>::to_checked(c).unwrap().to_rust())
        }
    }
}
