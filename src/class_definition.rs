use libc;
use std::ffi::CString;
use { Class, sys };

pub struct MethodDefinition<'a> {
    name: &'a str,
    function: *const libc::c_void,
    arity: isize,
}

impl<'a> MethodDefinition<'a> {
    pub fn new(name: &str, function: *const libc::c_void, arity: isize) -> MethodDefinition {
        MethodDefinition { name: name, function: function, arity: arity }
    }
}

pub struct ClassDefinition {
    class: Class,
}

impl ClassDefinition {
    pub fn new(name: &str) -> ClassDefinition {
        let raw_class = unsafe { sys::rb_define_class(CString::new(name).unwrap().as_ptr(), sys::rb_cObject) };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn wrapped(name: &str, alloc_func: extern "C" fn(klass: sys::VALUE) -> sys::VALUE) -> ClassDefinition {
        let raw_class = unsafe { sys::rb_define_class(CString::new(name).unwrap().as_ptr(), sys::rb_cObject) };
        unsafe { sys::rb_define_alloc_func(raw_class, alloc_func) };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn reopen(name: &str) -> ClassDefinition {
        let raw_class = unsafe {
            let class_id = sys::rb_intern(CString::new(name).unwrap().as_ptr());
            sys::rb_const_get(sys::rb_cObject, class_id)
        };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn define_method(self, def: MethodDefinition) -> ClassDefinition {
        unsafe {
            sys::rb_define_method(
                self.class.0,
                CString::new(def.name).unwrap().as_ptr(),
                def.function,
                def.arity
            );
        };
        self
    }
}
