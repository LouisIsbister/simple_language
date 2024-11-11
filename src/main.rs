use std::collections::HashMap;

mod test;
mod parser;
mod program;
use program::{Expr, Value, Program};

/// e ::= x | Z | T | F
///         | e < e | e = e | e + e | e - e | e * e | e / e
///         | e & e | e | e | ! e | (e)
///         | func x => e | apply( e, e ) | if e then e else e

fn main() {
    let left = Expr::Leaf(Value::Int(42));
    let right = Expr::Leaf(Value::Int(42));
    let add = Expr::new_bin_op(
        Box::new(left),
        Box::new(right),
        program::add
    );

    let mut vars: HashMap<String, Option<Expr>> = HashMap::new();
    vars.insert("int_var".to_string(), Some(Expr::Leaf(Value::Int(42))));
    vars.insert("bool_var".to_string(), Some(Expr::Leaf(Value::Bool(true))));

    let prog = Program::new(add);
    let res = prog.exec();

    println!("{:?}\nres: {:?}", prog, res);
}
