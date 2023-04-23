//! # Medley
//! The CAS Front-End with LaTeX Syntax

use colored::*;
use dialoguer::{theme::ColorfulTheme, Select};
use medley::{internal_engine, lexer};
use num_derive::FromPrimitive;
use num_traits::FromPrimitive;
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

impl Default for Config {
    fn default() -> Self {
        Self {
            engine: Engine::Internal,
        }
    }
}

impl Config {
    fn engine_select(&mut self) -> Result<(), io::Error> {
        let engine_list: Vec<Engine> = Engine::iter().collect();
        if let Some(selection) = Select::with_theme(&ColorfulTheme::default())
            .items(&engine_list)
            .default(0)
            .interact_opt()?
        {
            self.engine = Engine::from_usize(selection).unwrap();
        }
        Ok(())
    }
}

fn main() {
    println!("This is Medley, Version 2.0.0-alpha (2023-04-03)");
    let mut config = Config::default();
    loop {
        print!("> ");
        io::stdout().flush().unwrap();
        let mut input = String::new();
        match io::stdin().read_line(&mut input) {
            Ok(_) => match input.trim() {
                "exit" => {
                    println!("bye");
                    break;
                }
                "engine-select" => {
                    if let Err(select_err) = config.engine_select() {
                        error_expander(&select_err, select_err.kind())
                    }
                    continue;
                }
                "" => continue,
                _ => match lexer::Lexer::lex(input.trim().chars().collect()) {
                    Ok(tokens) => match engine_executor(&config.engine, tokens) {
                        Ok(result) => println!(" = {result}"),
                        Err(calc_err) => println!("[{}]\n{calc_err}", "CALC_ERR".bright_red()),
                    },
                    Err(syntax_err) => error_expander(&syntax_err, syntax_err.kind()),
                },
            },
            Err(input_err) => error_expander(&input_err, input_err.kind()),
        }
    }
}

fn engine_executor(engine: &Engine, tokens: Vec<lexer::Token>) -> Result<String, String> {
    match engine {
        Engine::Internal => internal_engine::run(tokens),
        Engine::Maxima => Ok(String::from("Unimplemented")),
    }
}

fn error_expander<T, U>(err: T, kind: U)
where
    T: std::error::Error,
    U: std::fmt::Display,
{
    eprintln!("[ {}: {} ]", "ERR".bright_red(), kind);
    eprintln!("{}", err);
}
