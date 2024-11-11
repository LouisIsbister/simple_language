#![allow(unused)]

use std::{collections::HashMap, env::Vars, fmt::Debug};

/// structure that keeps track of the root of the binary tree, 
/// and the variables during program runtime 
/// 
/// examples
/// ```
/// let left = Expr::Leaf(Value::Int(42));
/// let right = Expr::Leaf(Value::Int(42));
/// let add = Expr::new_bin_op(Box::new(left), Box::new(right), add);
/// 
/// // create and execute the program 
/// let prog = Program::new(add);
/// let res = prog.exec();     // outputs 84
/// ```
/// 
#[derive(Debug)]
pub struct Program {
    root: Expr,
    vars: HashMap<String, Option<Expr>>,
}

impl Program {
    pub fn new(root: Expr) -> Self {
        Self { root, vars: HashMap::new() }
    }
    pub fn add_var(&mut self, id: String, val: Option<Expr>) {
        self.vars.insert(id, val);
    }
    pub fn exec(&self) -> Value {
        self.root.execute()
    }
}

///enum type of the primitive types available for the programs
/// 
/// examples
/// ```
/// let int_42 = Value::Int(42);
/// let bool_false = Value::Bool(false));
/// let str = Value::Var("x".to_string());
/// ```
#[derive(Debug, Clone)]
pub enum Value {
    Int(i64),
    Bool(bool),
    Var(String)
}

/// represents a node in the abstract syntax tree that gets built from a program
#[derive(Debug, Clone)]
pub enum Expr {
    Leaf(Value),
    UnaryOp {
        val: Box<Expr>,
        func: fn(Value) -> Value,
    },
    BinOp {
        left: Box<Expr>,
        right: Box<Expr>,
        func: fn(Value, Value) -> Value,
    },
    Func {
        param: Box<Expr>,
        func: fn(&Expr) -> Value,
    }
}

impl Expr {
    pub fn new_unary_op(val: Box<Expr>, func: fn(Value) -> Value) -> Expr {
        Expr::UnaryOp { val, func }
    }

    pub fn new_bin_op(left: Box<Expr>, right: Box<Expr>, func: fn(Value, Value) -> Value) -> Expr {
        Expr::BinOp { left, right, func }
    }

    pub fn execute(&self) -> Value {
        match self {
            Expr::Leaf(val) => val.clone(),
            Expr::UnaryOp { val, func } => func(val.execute()),
            Expr::BinOp { left, right, func } => func(left.execute(), right.execute()),
            Expr::Func { param, func } => func(param),
        }
    }
}

/// --- binary operators ---
///  | e < e | e = e | e + e | e - e | e * e | e / e
///  | e & e | e | e | ! e | (e)

pub fn add(left: Value, right: Value) -> Value { bin_eval(left, right, "+") }
pub fn sub(left: Value, right: Value) -> Value { bin_eval(left, right, "-") }
pub fn mul(left: Value, right: Value) -> Value { bin_eval(left, right, "*") }
pub fn div(left: Value, right: Value) -> Value { bin_eval(left, right, "/") }
pub fn bitwise_and(left: Value, right: Value) -> Value { bin_eval(left, right, "&") }
pub fn bitwise_or(left: Value, right: Value) -> Value { bin_eval(left, right, "|") }

pub fn lt(left: Value, right: Value) -> Value { bin_eval(left, right, "<") }
pub fn le(left: Value, right: Value) -> Value { bin_eval(left, right, "<=") }
pub fn gt(left: Value, right: Value) -> Value { bin_eval(left, right, ">") }
pub fn ge(left: Value, right: Value) -> Value { bin_eval(left, right, ">=") }
pub fn eq(left: Value, right: Value) -> Value { bin_eval(left, right, "==") }

pub fn not(left: Value) -> Value { unary_logic_eval(left, "!") }
pub fn logic_and(left: Value, right: Value) -> Value { binary_logic_eval(left, right, "&&") }
pub fn logic_or(left: Value, right: Value) -> Value { binary_logic_eval(left, right, "||") }

/// Supported operators (eventually)
///   Numeric: + - * / 
///   Bitwise: & |
///   Boolean (numeric): < <= > >= ==
///   Boolean: ! && ||

pub fn bin_eval(left: Value, right: Value, op: &str) -> Value {
    match (left, right) {
        (Value::Int(l), Value::Int(r)) => match op {
            "+" => Value::Int(l + r),
            "-" => Value::Int(l - r),
            "*" => Value::Int(l * r),
            "/" => {
                if r == 0 { panic!("Division by 0!"); }
                Value::Int(l / r)
            },
            "&" => Value::Int(l & r),
            "|" => Value::Int(l | r),
            "<" => Value::Bool(l < r),
            "<=" => Value::Bool(l <= r),
            ">" => Value::Bool(l > r),
            ">=" => Value::Bool(l >= r),
            "==" => Value::Bool(l == r),
            _ => panic!("Unknown binary operator {}", op)
        },
        _ => panic!("Unsupported operand types for numerical binary opertors."),
    }
}

pub fn binary_logic_eval(left: Value, right: Value, op: &str) -> Value {
    match (left, right) {
        (Value::Bool(l), Value::Bool(r)) => match op {
            "&&" => Value::Bool(l && r),
            "||" => Value::Bool(l || r),
            _ => panic!("Unknown unary operator {}", op)
        },
        _ => panic!("Unsupported operand types for unary opertors."),
    }
}

pub fn unary_logic_eval(left: Value, op: &str) -> Value {
    match left {
        Value::Bool(l) => match op {
            "!" => Value::Bool(!l),
            _ => panic!("Unknown unary operator {}", op)
        },
        _ => panic!("Unsupported operand types for unary opertors."),
    }
}