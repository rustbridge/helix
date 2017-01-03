use sys::{self, VALUE};
use ToRuby;

impl ToRuby for () {
    fn to_ruby(self) -> VALUE {
        unsafe { sys::Qnil }
    }
}
