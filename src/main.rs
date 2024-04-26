use std::{error::Error, fmt::format};
use std::io::Write;
use errors::CalculatorError;
use scanner::Scanner;

mod scanner;
mod errors;
mod token;


fn main() {
    loop {
        print!("> ");
        match std::io::stdout().flush() {
            Err(_) => {
                println!("error flushing buffer");
                continue;
            }
            _ => {}
        }
        let mut input = String::new();
        if let Err(_) = std::io::stdin().read_line(&mut input) {
            println!("error reading input");
            continue;
        }
        print!("{}", input);
        input.pop();
        let mut scanner = Scanner::new(&input);

        match scanner.parse() {
            Err(err) => println!("error parsing: {}", err),
            Ok(()) => println!("parsed {:?}", scanner)
        }
    }
}
