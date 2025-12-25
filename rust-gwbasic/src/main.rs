use rust_gwbasic::{Lexer, Parser, Interpreter};
use std::io::{self, Write};

fn main() {
    println!("GW-BASIC (Rust) interpreter v{}", rust_gwbasic::VERSION);
    println!("Type BASIC statements or 'EXIT' to quit");
    println!();

    let mut interpreter = Interpreter::new();

    loop {
        print!("> ");
        io::stdout().flush().unwrap();

        let mut input = String::new();
        if io::stdin().read_line(&mut input).is_err() {
            eprintln!("Error reading input");
            continue;
        }

        let input = input.trim();
        if input.is_empty() {
            continue;
        }

        if input.eq_ignore_ascii_case("EXIT") || input.eq_ignore_ascii_case("QUIT") {
            break;
        }

        // Try to tokenize, parse, and execute
        let mut lexer = Lexer::new(input);
        let tokens = match lexer.tokenize() {
            Ok(t) => t,
            Err(e) => {
                eprintln!("Lexer error: {}", e);
                continue;
            }
        };

        let mut parser = Parser::new(tokens);
        let ast = match parser.parse() {
            Ok(a) => a,
            Err(e) => {
                eprintln!("Parser error: {}", e);
                continue;
            }
        };

        if let Err(e) = interpreter.execute(ast) {
            eprintln!("Runtime error: {}", e);
        }
    }

    println!("Goodbye!");
}

