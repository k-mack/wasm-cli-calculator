mod bindings;

use crate::bindings::exports::docs::calculator::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: i32, b: i32) -> i32 {
        a - b
    }
}
