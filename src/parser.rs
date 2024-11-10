
use regex::Regex;

use crate::program::{
    self, 
    ExprNode, 
    ExprValue, 
    Program
};

pub fn parse(src_code: String) -> ExprNode {
    let mut tokens: Vec<&str> = src_code.split_whitespace().collect();
    assert!(!tokens.is_empty());
    let init = tokens.remove(0);
    let root = parse_tokens(init, tokens);

    ExprNode::Leaf(ExprValue::Bool(false))
}

/// x | Z | T | F
///   | e < e | e = e | e + e | e - e | e * e | e / e
///   | e & e | e | e | ! e | (e)
///   | func x => e | apply( e, e ) | if e then e else e
fn parse_tokens(cur: &str, tokens: Vec<&str>) -> ExprNode {
    match cur {
        "T" => ExprNode::Leaf(ExprValue::Bool(true)),
        "F" => ExprNode::Leaf(ExprValue::Bool(false)),

        _ => panic!("Unknown char sequence {}", cur),
    }
}


// fn parse_token(token: &str) -> ExprNode {
//     match token {
//         "T" => ExprNode::Leaf(ExprValue::Bool(true)),
//         "F" => ExprNode::Leaf(ExprValue::Bool(false)),

//         _ => panic!("Unknown char sequence {}", token),
//     }
// }

