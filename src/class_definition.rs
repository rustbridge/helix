use { Class };
use sys::{self, c_string, c_func};

pub struct MethodSpecification {
    name: c_string,
    function: c_func,
    arity: isize,
}

pub enum MethodDefinition {
    Class(MethodSpecification),
    Instance(MethodSpecification)
}

impl MethodDefinition {
    pub fn class(name: c_string, function: c_func, arity: isize) -> MethodDefinition {
        MethodDefinition::Class(MethodSpecification { name: name, function: function, arity: arity })
    }

    pub fn instance(name: c_string, function: c_func, arity: isize) -> MethodDefinition {
        MethodDefinition::Instance(MethodSpecification { name: name, function: function, arity: arity })
    }
}

pub struct ClassDefinition {
    pub class: Class,
}

impl ClassDefinition {
    pub fn new(name: c_string) -> ClassDefinition {
        let raw_class = ruby_try!(sys::safe::rb_define_class(name, unsafe { sys::rb_cObject }));
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn wrapped(name: c_string, alloc_func: extern "C" fn(klass: sys::VALUE) -> sys::VALUE) -> ClassDefinition {
        let raw_class = ruby_try!(sys::safe::rb_define_class(name, unsafe { sys::rb_cObject }));
        ruby_try!(sys::safe::rb_define_alloc_func(raw_class, alloc_func));
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn reopen(name: c_string) -> ClassDefinition {
        let raw_class = unsafe {
            let class_id = sys::rb_intern(name);
            ruby_try!(sys::safe::rb_const_get(sys::rb_cObject, class_id))
        };
        ClassDefinition { class: Class(raw_class) }
    }

    pub fn define_method(self, def: MethodDefinition) -> ClassDefinition {
        match def {
            MethodDefinition::Instance(def) => {
                ruby_try!(sys::safe::rb_define_method(
                    self.class.0,
                    def.name,
                    def.function,
                    def.arity
                ));
            },
            MethodDefinition::Class(def) => {
                ruby_try!(sys::safe::rb_define_singleton_method(
                    self.class.0,
                    def.name,
                    def.function,
                    def.arity
                ));
            }
        }

        self
    }
}
