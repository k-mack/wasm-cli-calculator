mod bindings;

use crate::bindings::calc::basic::add::add;
use crate::bindings::calc::basic::subtract::subtract;
use crate::bindings::exports::calc::basic::calculate::{Guest, Op};

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: i32, y: i32) -> i32 {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}
