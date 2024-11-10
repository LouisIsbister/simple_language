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
    pub fn add_var(&mut self, id: String, value: Option<ExprNode>) {
        self.vars.insert(id, value);
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
/// let str = ExprValue::Var("x".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum ExprValue {
    Int(i64),
    Bool(bool),
    Var(String)
}

/// represents a node in the abstract syntax tree that gets built from a program
#[derive(Debug, Clone)]
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
    Func {
        param: Box<ExprNode>,
        func: fn(&ExprNode) -> ExprValue,
    }
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
            ExprNode::Func { param, func } => {
                func(param)
            }
        }
    }
}

/// --- binary operators ---
///  | e < e | e = e | e + e | e - e | e * e | e / e
///  | e & e | e | e | ! e | (e)

pub fn add(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "+") }
pub fn sub(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "-") }
pub fn mul(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "*") }
pub fn div(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "/") }
pub fn bitwise_and(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "&") }
pub fn bitwise_or(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "|") }

pub fn lt(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "<") }
pub fn le(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "<=") }
pub fn gt(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, ">") }
pub fn ge(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, ">=") }
pub fn eq(left: ExprValue, right: ExprValue) -> ExprValue { bin_eval(left, right, "=") }

pub fn not(left: ExprValue) -> ExprValue { unary_logic_eval(left, "!") }
pub fn logic_and(left: ExprValue, right: ExprValue) -> ExprValue { binary_logic_eval(left, right, "&&") }
pub fn logic_or(left: ExprValue, right: ExprValue) -> ExprValue { binary_logic_eval(left, right, "||") }

pub fn bin_eval(left: ExprValue, right: ExprValue, op: &str) -> ExprValue {
    match (left, right) {
        (ExprValue::Int(l), ExprValue::Int(r)) => match op {
            "+" => ExprValue::Int(l + r),
            "-" => ExprValue::Int(l - r),
            "*" => ExprValue::Int(l * r),
            "/" => {
                if r == 0 { panic!("Division by 0!"); }
                ExprValue::Int(l / r)
            },
            "&" => ExprValue::Int(l & r),
            "|" => ExprValue::Int(l | r),
            "<" => ExprValue::Bool(l < r),
            "<=" => ExprValue::Bool(l <= r),
            ">" => ExprValue::Bool(l > r),
            ">=" => ExprValue::Bool(l >= r),
            "=" => ExprValue::Bool(l == r),
            _ => panic!("Unknown binary operator {}", op)
        },
        _ => panic!("Unsupported operand types for numerical binary opertors."),
    }
}

pub fn binary_logic_eval(left: ExprValue, right: ExprValue, op: &str) -> ExprValue {
    match (left, right) {
        (ExprValue::Bool(l), ExprValue::Bool(r)) => match op {
            "&&" => ExprValue::Bool(l && r),
            "||" => ExprValue::Bool(l || r),
            _ => panic!("Unknown unary operator {}", op)
        },
        _ => panic!("Unsupported operand types for unary opertors."),
    }
}

pub fn unary_logic_eval(left: ExprValue, op: &str) -> ExprValue {
    match left {
        ExprValue::Bool(l) => match op {
            "!" => ExprValue::Bool(!l),
            _ => panic!("Unknown unary operator {}", op)
        },
        _ => panic!("Unsupported operand types for unary opertors."),
    }
}