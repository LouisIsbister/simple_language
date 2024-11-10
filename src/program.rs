#![allow(unused)]

use std::{collections::HashMap, env::Vars, fmt::Debug};

/// structure that keeps track of the root of the binary tree, 
/// and the variables during program runtime 
/// 
/// examples
/// ```
/// let left = ExprNode::Leaf(ExprValue::Int(42));
/// let right = ExprNode::Leaf(ExprValue::Int(42));
/// let add = ExprNode::new_bin_op(Box::new(left), Box::new(right), add);
/// 
/// // create and execute the program 
/// let prog = Program::new(add);
/// let res = prog.exec();     // outputs 84
/// ```
/// 
#[derive(Debug)]
pub struct Program {
    root: ExprNode,
    vars: HashMap<String, Option<ExprNode>>,
}

impl Program {
    pub fn new(root: ExprNode) -> Self {
        Self { root, vars: HashMap::new() }
    }
    pub fn exec(&self) -> ExprValue {
        self.root.execute()
    }
}

///enum type of the primitive types available for the programs
/// 
/// examples
/// ```
/// let int_42 = ExprValue::Int(42);
/// let bool_false = ExprValue::Bool(false));
/// let str = ExprValue::Var("my_var".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum ExprValue {
    Int(i64),
    Bool(bool),
    Var(String)
}

/// represents a node in the abstract syntax tree that gets built from a program
#[derive(Debug)]
pub enum ExprNode {
    Leaf(ExprValue),
    UnaryOp {
        val: Box<ExprNode>,
        op: fn(ExprValue) -> ExprValue,
    },
    BinOp {
        left: Box<ExprNode>,
        right: Box<ExprNode>,
        op: fn(ExprValue, ExprValue) -> ExprValue,
    },
}

impl ExprNode {
    pub fn new_value(value: ExprValue) -> ExprNode {
        ExprNode::Leaf(value)
    }

    pub fn new_unary_op(
        val: Box<ExprNode>,
        op: fn(ExprValue) -> ExprValue,
    ) -> ExprNode {
        ExprNode::UnaryOp { val, op }
    }

    pub fn new_bin_op(
        left: Box<ExprNode>,
        right: Box<ExprNode>,
        op: fn(ExprValue, ExprValue) -> ExprValue,
    ) -> ExprNode {
        ExprNode::BinOp { left, right, op }
    }

    pub fn execute(&self) -> ExprValue {
        match self {
            ExprNode::Leaf(val) => val.clone(),
            ExprNode::UnaryOp { val, op } => {
                op(ExprNode::execute(val))
            }
            ExprNode::BinOp { left, right, op } => {
                op(ExprNode::execute(left), ExprNode::execute(right))
            }
        }
    }
}

/// --- binary operators ---
///  | e < e | e = e | e + e | e - e | e * e | e / e
///  | e & e | e | e | ! e | (e)

pub fn le(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "<") }
pub fn eq(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "=") }
pub fn add(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "+") }
pub fn sub(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "-") }
pub fn mul(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "*") }
pub fn div(left: ExprValue, right: ExprValue) -> ExprValue {
    let denom_zero = match right { 
        ExprValue::Int(l) => l == 0,
        _ => panic!("Division must take numeric values."),
    };
    if denom_zero {
        panic!("Denominator is zero.")
    }
    bin_eval(left, right, "/")
}
pub fn bitwise_and(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "&") }
pub fn bitwise_or(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "|") }

pub fn bin_eval(left: ExprValue, right: ExprValue, op: &str) -> ExprValue {
    match (left, right) {
        (ExprValue::Int(l), ExprValue::Int(r)) => {
            match op {
                "<" => ExprValue::Bool(l < r),
                "=" => ExprValue::Bool(l == r),
                "+" => ExprValue::Int(l + r),
                "-" => ExprValue::Int(l - r),
                "*" => ExprValue::Int(l * r),
                "/" => ExprValue::Int(l / r),
                "&" => ExprValue::Int(l & r),
                "|" => ExprValue::Int(l | r),
                _ => panic!("Unknown binary operator {}", op)
            }
        },
        _ => panic!("Unsupported operand types for numerical binary opertors"),
    }
}

pub fn unary_eval(left: ExprValue, op: &str) -> ExprValue {
    match left {
        ExprValue::Bool(l) => {
            match op {
                "!" => ExprValue::Bool(!l),
                _ => panic!("Unknown unary operator {}", op)
            }
        },
        _ => panic!("Unsupported operand types for numerical binary opertors"),
    }
}