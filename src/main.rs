use std::borrow::Borrow;
use std::io;
use std::io::Write;
use medley::lexer;
use medley::parser;

fn main() {
    loop {
        print!(">> ");
        io::stdout().flush().unwrap();
        let mut code = String::new();
        io::stdin().read_line(&mut code).ok().expect("failed to read line");
        if code == "exit\n" {
            break;
        }
        let lexer = lexer::Lexer::init(code.chars().collect());
        let mut parser = parser::Parser::init(lexer);
        let expr = parser.parse();
        if let Some(expr) = expr {
            println!("{}", eval(expr.borrow()));
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
    }
}
