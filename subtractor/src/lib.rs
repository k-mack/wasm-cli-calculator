mod bindings;

use crate::bindings::calc::basic::types::Numeric;
use crate::bindings::exports::calc::basic::subtract::Guest;

struct Component;

impl Guest for Component {
    fn subtract(a: Numeric, b: Numeric) -> Numeric {
        a - b
    }
}
