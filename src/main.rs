use std::{collections::HashMap};

mod program;
use program::{ExprNode, ExprValue, Program};

/// e ::= x | Z | T | F
///         | e < e | e = e | e + e | e - e | e * e | e / e
///         | e & e | e | e | ! e | (e)
///         | func x => e | apply( e, e ) | if e then e else e

fn main() {
    let left = ExprNode::Leaf(ExprValue::Int(42));
    let right = ExprNode::Leaf(ExprValue::Int(42));
    let add = ExprNode::new_bin_op(
        Box::new(left),
        Box::new(right),
        program::add
    );

    let mut vars: HashMap<String, Option<ExprNode>> = HashMap::new();
    vars.insert("int_var".to_string(), Some(ExprNode::Leaf(ExprValue::Int(42))));
    vars.insert("bool_var".to_string(), Some(ExprNode::Leaf(ExprValue::Bool(true))));

    let prog = Program::new(add);
    let res = prog.exec();

    println!("{:?}\nres: {:?}", prog, res);
}
