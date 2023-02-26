use std::borrow::Borrow;
use std::io;
use std::io::Write;
use medley::lexer;
use medley::parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input == "exit\r\n" || input == "exit\n" {
                    println!("bye");
                    break;
                }
                let lexer = lexer::Lexer::init(input.chars().collect());
                match parser::Parser::init(lexer) {
                    Ok(mut parser) => {
                        let expr = parser.parse();
                        if let Some(expr) = expr {
                            match eval(expr.borrow()) {
                                Ok(result) => println!(" = {result}"),
                                Err(calc_err) => println!(" = [CALC_ERR] {calc_err}")
                            }
                        }
                    }
                    Err(syntax_err) => println!(" = [SYNTAX_ERR] {syntax_err}")
                }
            }
            Err(input_err) => {
                println!("{input_err}")
            }
        }
    }
}

/*構文木の実行*/
fn eval(expr: &parser::Expr) -> Result<f64, String> {
    match expr {
        parser::Expr::Number(n) => Ok(*n),
        parser::Expr::PrefixExpr { operator: _, right } => Ok(-eval(right)?),
        parser::Expr::InfixExpr {
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
                "Div" => if left == 0 as f64 && right == 0 as f64 {
                    Err(String::from("indeterminate (divided by 0)"))
                } else if right == 0 as f64 {
                    Err(String::from("incompatible (divided by 0)"))
                } else {
                    Ok(left / right)
                }
                _ => Err(String::from("invalid operator"))
            }
        }
        parser::Expr::Fraction {
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