mod bindings;

use crate::bindings::calc::basic::calculate::{eval_expression, Op};
use crate::bindings::calc::basic::types::Numeric;

fn main() {
    let mut args = std::env::args();

    let Some(app_name) = args.next() else {
        panic!("Error: program name not available");
    };

    let Some(Ok(x)) = args.next().map(|s| s.parse::<Numeric>()) else {
        usage(&app_name);
        return;
    };
    let Some(Ok(y)) = args.next().map(|s| s.parse::<Numeric>()) else {
        usage(&app_name);
        return;
    };
    let Some(op) = args.next() else {
        usage(&app_name);
        return;
    };

    let result = match op.to_lowercase().as_str() {
        "add" => Some((eval_expression(Op::Add, x, y), "+")),
        "subtract" => Some((eval_expression(Op::Subtract, x, y), "-")),
        _ => None,
    };

    if let Some((answer, op_symbol)) = result {
        println!("{} {} {} = {}", x, op_symbol, y, answer);
    } else {
        usage(&app_name);
    }
}

fn usage(app_name: &str) {
    println!("usage: {} <number> <number> <add | subtract>", app_name);
}
