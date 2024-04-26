use std::{error::Error, fmt::format};
use std::io::Write;
use errors::CalculatorError;
use scanner::Scanner;
use parser::Parser;

mod scanner;
mod errors;
mod token;
mod parser;

fn main() {
    loop {
        print!("> ");

        if let Err(_) = std::io::stdout().flush() {
            println!("error flushing buffer");
            continue;
        }

        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("error reading input");
            continue;
        }

        // print!("{}", input);
        // input.pop();

        let mut scanner = Scanner::new(&input);

        match scanner.parse() {
            Err(err) => println!("error scanning: {}", err),
            Ok(()) => {
                // println!("parsed {:?}", scanner);

                let parser = Parser::new(&scanner.token_list);

                match parser.parse() {
                    Ok(res) => {
                        println!("{}", res);
                    },
                    Err(err) => {
                        println!("error parsing: {}", err)
                    },
                }
            }
        }
    }
}
