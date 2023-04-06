use medley::internal_engine;
use medley::lexer;
use std::borrow::Borrow;
use std::io;
use std::io::Write;

fn main() {
    println!("This is Medley, Version 1.2.0-alpha (2023-04-03)");
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => {
                if input.trim() == "exit" {
                    println!("bye");
                    break;
                }
                match lexer::Lexer::lex(input.chars().collect()) {
                    Ok(tokens) => {
                        if let Some(expr) = internal_engine::Parser::parse(tokens) {
                            match internal_engine::eval(expr.borrow()) {
                                Ok(result) => println!(" = {result}"),
                                Err(calc_err) => println!(" = [CALC_ERR] {calc_err}"),
                            }
                        }
                    }
                    Err(syntax_err) => println!(" = [SYNTAX_ERR] {syntax_err}"),
                }
            }
            Err(input_err) => {
                println!("{input_err}")
            }
        }
    }
}
