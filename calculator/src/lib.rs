mod bindings;

use crate::bindings::docs::calculator::add::add;
use crate::bindings::docs::calculator::subtract::subtract;
use crate::bindings::exports::docs::calculator::calculate::{Guest, Op};

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: i32, y: i32) -> i32 {
        match op {
            Op::Add => add(x, y),
            Op::Subtract => subtract(x, y),
        }
    }
}
