mod bindings;

use crate::bindings::exports::calc::basic::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: f64, b: f64) -> f64 {
        a + b
    }
}
