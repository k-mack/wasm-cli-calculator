mod bindings;

use crate::bindings::exports::calc::basic::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: f64, b: f64) -> f64 {
        a - b
    }
}
