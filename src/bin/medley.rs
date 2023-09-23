//! # Medley
//! MedleyのWindows用CLI実装

use cfe_medley::config_manager::*;
use cfe_medley::{internal_engine, lexer};
use colored::Colorize;
use std::io;
use std::io::Write;
use std::process::ExitCode;

fn main() -> ExitCode {
    println!("This is Medley, Version 2.0.0-alpha (2023-04-03)");
    match load_config() {
        Ok(parsed_config) => {
            let config = parsed_config;
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
                        "" => continue,
                        _ => match lexer::lex(input.trim().chars().collect()) {
                            Ok(tokens) => match engine_executor(&config.engine, tokens) {
                                Ok(result) => println!(" = {result}"),
                                Err(calc_err) => {
                                    println!("[{}]\n{calc_err}", "CALC_ERR".bright_red())
                                }
                            },
                            Err(syntax_err) => error_expander(&syntax_err, syntax_err.kind()),
                        },
                    },
                    Err(input_err) => error_expander(&input_err, input_err.kind()),
                }
            }
        }
        Err(config_error) => {
            error_expander(&config_error, config_error.kind());
            return ExitCode::FAILURE;
        }
    }
    ExitCode::SUCCESS
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
