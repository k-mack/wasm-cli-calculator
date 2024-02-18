mod bindings;

use crate::bindings::calc::basic::add::add;
use crate::bindings::calc::basic::subtract::subtract;
use crate::bindings::calc::basic::types::Numeric;
use crate::bindings::exports::calc::basic::calculate::{Guest, Op};

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: Numeric, y: Numeric) -> Numeric {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}
