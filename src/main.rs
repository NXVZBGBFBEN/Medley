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
            Ok(_) => {}
            Err(err) => {
                println!("{err}")
            }
        }
        if input == "exit\n" {
            println!(" bye");
            break;
        }
        let lexer = lexer::Lexer::init(input.chars().collect());
        let mut parser = parser::Parser::init(lexer);
        let expr = parser.parse();
        if let Some(expr) = expr {
            println!(" = {}", eval(expr.borrow()));
        }
    }
}

/*構文木の実行*/
fn eval(expr: &parser::Expr) -> f64 {
    match expr {
        parser::Expr::Number(n) => *n,
        parser::Expr::PrefixExpr { operator: _, right } => -eval(right),
        parser::Expr::InfixExpr {
            left,
            operator,
            right,
        } => {
            let left = eval(left);
            let right = eval(right);
            match operator.as_str() {
                "Plus" => left + right,
                "Minus" => left - right,
                "Times" => left * right,
                "Div" => left / right,
                _ => panic!("invalid operator"),
            }
        }
        parser::Expr::Fraction {
            numerator,
            denominator,
        } => {
            let numerator = eval(numerator);
            let denominator = eval(denominator);
            numerator / denominator
        }
    }
}
