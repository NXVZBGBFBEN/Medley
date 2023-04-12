use medley::{lexer, internal_engine};
use std::borrow::Borrow;
use std::io;
use std::io::Write;
use dialoguer::Select;
use dialoguer::theme::ColorfulTheme;

#[derive(Default)]
struct Config {
    engine: u8,
}

impl Config {
    fn engine_select(&self) -> Result<Self, String> {
        //TODO エンジンリストの列挙型管理
        let engine_list = vec!["Internal", "Maxima"];
        //TODO .interact()を.interact_on()に
        match Select::with_theme(&ColorfulTheme::default()).items(&engine_list).default(0).interact() {
            Ok(x) => {
                Ok(Self {
                    engine: match x {
                        0_usize => 0,
                        1_usize => 1,
                        _ => 2,
                    }
                })
            }
            Err(e) => Err(e.to_string()),
        }
    }
}

fn main() {
    println!("This is Medley, Version 1.2.0-alpha (2023-04-03)");
    let mut config = Config::default();
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
                if input.trim() == "engine-select" {
                    match config.engine_select() {
                        Ok(x) => config = x,
                        Err(e) => println!("{e}"),
                    }
                    continue;
                }
                match lexer::Lexer::lex(input.chars().collect()) {
                    Ok(tokens) => {
                        //TODO 内蔵エンジンのwrapper作成・エンジン実行部の関数化
                        match config.engine {
                            0 => {
                                if let Some(expr) = internal_engine::Parser::parse(tokens) {
                                    match internal_engine::eval(expr.borrow()) {
                                        Ok(result) => println!(" = {result}"),
                                        Err(calc_err) => println!(" = [CALC_ERR] {calc_err}"),
                                    }
                                }
                            }
                            _ => println!("?")
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
