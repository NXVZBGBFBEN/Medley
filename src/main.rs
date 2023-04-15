use dialoguer::{theme::ColorfulTheme, Select};
use medley::{internal_engine, lexer};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
use std::borrow::Borrow;
use std::io;
use std::io::Write;
use strum::IntoEnumIterator;
use strum_macros::{Display, EnumIter};

struct Config {
    engine: Engine,
}

#[derive(Display, EnumIter, FromPrimitive)]
enum Engine {
    Internal,
    Maxima,
}

impl Config {
    fn default() -> Self {
        Self {
            engine: Engine::Internal,
        }
    }
    fn engine_select(&mut self) -> Result<(), String> {
        let engine_list: Vec<Engine> = Engine::iter().collect();
        match Select::with_theme(&ColorfulTheme::default())
            .items(&engine_list)
            .default(0)
            .interact_opt()
        {
            Ok(x) => {
                if let Some(selection) = x {
                    self.engine = Engine::from_usize(selection).unwrap();
                    Ok(())
                } else {
                    Ok(())
                }
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
                    if let Err(select_err) = config.engine_select() {
                        println!("{}", select_err);
                    }
                    continue;
                }
                match lexer::Lexer::lex(input.chars().collect()) {
                    Ok(tokens) => {
                        //TODO 内蔵エンジンのwrapper作成・エンジン実行部の関数化
                        match config.engine {
                            Engine::Internal => {
                                if let Some(expr) = internal_engine::Parser::parse(tokens) {
                                    match internal_engine::eval(expr.borrow()) {
                                        Ok(result) => println!(" = {result}"),
                                        Err(calc_err) => println!(" = [CALC_ERR] {calc_err}"),
                                    }
                                }
                            }
                            _ => println!("?"),
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
