#[macro_use]
extern crate helix_runtime;

declare_types! {
    class Calculator {
        def multiply(&self, one: f64, two: f64) -> f64 {
            one * two
        }
    }
}
