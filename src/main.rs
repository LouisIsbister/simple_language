use std::{collections::HashMap, fmt::Debug};

/// Node types for the expression tree:
///  - binary operators
///  - functions
///     -
///     -
///     -
///  - leaves
///     - variable (x)
///     - integer: Int
///     - true/false: Bool
///     -
/// e ::= x | Z | T | F
///         | e < e | e = e | e + e | e - e | e * e | e / e
///         | e & e | e | e | ! e | (e)
///         | func x => e | apply( e, e ) | if e then e else e

#[derive(Debug)]
pub struct Program {
    vars: HashMap<String, Box<dyn ProgExpr>>,
    root: Option<Box<dyn ProgExpr>>,
}

impl Program {
    pub fn new(
        vars: HashMap<String, Box<dyn ProgExpr>>,
        root: Option<Box<dyn ProgExpr>>,
    ) -> Program {
        Self { vars, root }
    }

    pub fn execute(&mut self) -> Result<ExprResult, String> {
        return self
            .root
            .as_mut()
            .ok_or_else(|| "Could not execute the program as root == None!".to_string())
            .and_then(|node| Ok(node.execute()));
    }
}

pub trait ProgExpr: Debug {
    fn execute(&self) -> ExprResult;
}

#[derive(Debug)]
pub enum ExprResult {
    Int(i64),
    Bool(bool),
}

#[derive(Debug)]
struct IntNode {
    value: i64,
}
impl ProgExpr for IntNode {
    fn execute(&self) -> ExprResult {
        ExprResult::Int(self.value)
    }
}

#[derive(Debug)]
struct BoolNode {
    value: bool,
}
impl ProgExpr for BoolNode {
    fn execute(&self) -> ExprResult {
        ExprResult::Bool(self.value)
    }
}

fn main() {
    // Insert values into the global `VARS`.

    let root = IntNode { value: 42 };
    let mut vars: HashMap<String, Box<dyn ProgExpr>> = HashMap::new();
    vars.insert("int_var".to_string(), Box::new(IntNode { value: 42 }));
    vars.insert("bool_var".to_string(), Box::new(BoolNode { value: true }));

    let mut  prog = Program::new(vars, Some(Box::new(root)));
    let res = prog.execute();

    match res {
        Ok(int) => {
            println!("{:?}\nres: {:?}", prog, int);
        }
        Err(_) => (),
    }

    
}
