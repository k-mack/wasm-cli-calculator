mod bindings;

use crate::bindings::docs::calculator::add::add;
use crate::bindings::exports::docs::calculator::calculate::{Guest, Op};

struct Component;

impl Guest for Component {
    fn eval_expression(op: Op, x: u32, y: u32) -> u32 {
        match op {
            Op::Add => add(x, y),
        }
    }
}
