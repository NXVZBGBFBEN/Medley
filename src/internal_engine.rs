//! ## 内蔵演算エンジン(Original)

mod parser;
mod syntax_tree;

use self::parser::Parser;
use self::syntax_tree::SyntaxTree;
use crate::lexer;
use std::borrow::Borrow;

pub fn run(input: Vec<lexer::Token>) -> Result<String, String> {
    let expr = Parser::parse(input).unwrap();
    eval(expr.borrow()).map(|v| format!("{v}"))
}

/*構文木の実行*/
pub fn eval(expr: &SyntaxTree) -> Result<f64, String> {
    match expr {
        SyntaxTree::Number(n) => Ok(*n),
        SyntaxTree::PrefixExpr { operator: _, right } => Ok(-eval(right)?),
        SyntaxTree::InfixExpr {
            left,
            operator,
            right,
        } => {
            let left = eval(left)?;
            let right = eval(right)?;
            match operator.as_str() {
                "Plus" => Ok(left + right),
                "Minus" => Ok(left - right),
                "Times" => Ok(left * right),
                "Div" => {
                    if left == 0 as f64 && right == 0 as f64 {
                        Err(String::from("indeterminate (divided by 0)"))
                    } else if right == 0 as f64 {
                        Err(String::from("incompatible (divided by 0)"))
                    } else {
                        Ok(left / right)
                    }
                }
                _ => Err(String::from("invalid operator")),
            }
        }
        SyntaxTree::Fraction {
            numerator,
            denominator,
        } => {
            let numerator = eval(numerator)?;
            let denominator = eval(denominator)?;
            if numerator == 0 as f64 && denominator == 0 as f64 {
                Err(String::from("indeterminate (denominator is 0)"))
            } else if denominator == 0 as f64 {
                Err(String::from("incompatible (denominator is 0)"))
            } else {
                Ok(numerator / denominator)
            }
        }
    }
}
