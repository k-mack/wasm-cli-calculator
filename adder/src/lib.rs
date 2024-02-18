mod bindings;

use crate::bindings::calc::basic::types::Numeric;
use crate::bindings::exports::calc::basic::add::Guest;

struct Component;

impl Guest for Component {
    fn add(a: Numeric, b: Numeric) -> Numeric {
        a + b
    }
}
