mod bindings;

use crate::bindings::exports::calc::basic::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
