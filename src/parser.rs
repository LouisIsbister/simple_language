
use regex::Regex;

use crate::program::{
    self, 
    Expr, 
    Value, 
    Program
};

pub fn parse(src_code: String) -> Expr {
    let mut tokens: Vec<&str> = src_code.split_whitespace().collect();
    assert!(!tokens.is_empty());
    let init = tokens.remove(0);
    let root = parse_tokens(init, tokens);

    Expr::Leaf(Value::Bool(false))
}

/// x | Z | T | F
///   | e < e | e = e | e + e | e - e | e * e | e / e
///   | e & e | e | e | ! e | (e)
///   | func x => e | apply( e, e ) | if e then e else e
fn parse_tokens(cur: &str, tokens: Vec<&str>) -> Expr {
    match cur {
        "T" => Expr::Leaf(Value::Bool(true)),
        "F" => Expr::Leaf(Value::Bool(false)),

        _ => panic!("Unknown char sequence {}", cur),
    }
}


// fn parse_token(token: &str) -> Expr {
//     match token {
//         "T" => Expr::Leaf(ExprValue::Bool(true)),
//         "F" => Expr::Leaf(ExprValue::Bool(false)),

//         _ => panic!("Unknown char sequence {}", token),
//     }
// }

