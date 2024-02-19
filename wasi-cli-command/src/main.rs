mod bindings;

use crate::bindings::calc::basic::types::Numeric;
use crate::bindings::calc::basic::calculate::{eval_expression, Op};

fn main() {
    let mut args = std::env::args();

    let _ = args.next(); // ignore first arg as it is the program name

    // TODO: handle erroneous inputs
    let x = args.next().unwrap().parse::<Numeric>().unwrap();
    let y = args.next().unwrap().parse::<Numeric>().unwrap();
    let op = args.next().unwrap();

    let result = match op.as_str() {
        "add" => Some((eval_expression(Op::Add, x, y), "+")),
        "subtract" => Some((eval_expression(Op::Subtract, x, y), "-")),
        _ => None,
    };

    if let Some((answer, op_symbol)) = result {
        println!("{} {} {} = {}", x, op_symbol, y, answer);
    } else {
        println!("unsupported operation \"{}\"", op);
    }
}
