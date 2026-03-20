use rust_gwbasic::{Lexer, Parser, Interpreter};
use std::io::{self, Write};
use std::fs;
use std::env;
use std::process::Command;

fn main() {
    maybe_reexec_with_x11();

    let args: Vec<String> = env::args().collect();

    // Parse command line arguments
    let mut use_gui = false;
    let mut filename: Option<String> = None;
    let mut input_values: Vec<String> = Vec::new();
    let mut i = 1usize;

    while i < args.len() {
        match args[i].as_str() {
            "--gui" | "-g" => {
                use_gui = true;
            }
            "--help" | "-h" => {
                print_usage();
                return;
            }
            "--input" | "-i" => {
                i += 1;
                if i >= args.len() {
                    eprintln!("Missing value for --input");
                    std::process::exit(2);
                }
                input_values.push(args[i].clone());
            }
            arg if !arg.starts_with('-') && filename.is_none() => {
                filename = Some(arg.to_string());
            }
            arg => {
                eprintln!("Unknown argument: {}", arg);
                print_usage();
                std::process::exit(2);
            }
        }
        i += 1;
    }

    // If a filename is provided, run it
    if let Some(file) = filename {
        run_file(&file, use_gui, &input_values);
        return;
    }

    // Otherwise, start REPL
    println!("GW-BASIC (Rust) interpreter v{}", rust_gwbasic::VERSION);
    println!("Type BASIC statements or 'EXIT' to quit");
    println!();

    let display_available = has_display();
    let should_use_gui = use_gui || display_available;
    let mut interpreter = if should_use_gui {
        match Interpreter::new_with_gui() {
            Ok(interp) => interp,
            Err(e) => {
                if use_gui {
                    eprintln!("Error creating GUI window: {}", e);
                    eprintln!("Falling back to ASCII mode...");
                }
                Interpreter::new()
            }
        }
    } else {
        Interpreter::new()
    };

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

fn print_usage() {
    println!("GW-BASIC (Rust) v{}", rust_gwbasic::VERSION);
    println!();
    println!("USAGE:");
    println!("  rust-gwbasic [OPTIONS] [FILE]");
    println!();
    println!("OPTIONS:");
    println!("  -g, --gui      Force GUI window for graphics mode");
    println!("  -i, --input    Queue one INPUT value (repeatable)");
    println!("  -h, --help     Show this help message");
    println!();
    println!("EXAMPLES:");
    println!("  rust-gwbasic                    Start REPL");
    println!("  rust-gwbasic program.bas        Run program (GUI auto-detected if display exists)");
    println!("  rust-gwbasic --gui program.bas  Run program with GUI window");
    println!("  rust-gwbasic -i 32 program.bas  Provide INPUT value from CLI");
}

fn run_file(filename: &str, use_gui: bool, input_values: &[String]) {
    // Read the file
    let content = match fs::read_to_string(filename) {
        Ok(c) => c,
        Err(e) => {
            eprintln!("Error reading file '{}': {}", filename, e);
            std::process::exit(1);
        }
    };

    let display_available = has_display();
    let should_use_gui = use_gui || display_available;

    // Create interpreter with specified graphics backend
    let mut interpreter = if should_use_gui {
        match Interpreter::new_with_gui() {
            Ok(interp) => interp,
            Err(e) => {
                if use_gui {
                    eprintln!("Error creating GUI window: {}", e);
                    eprintln!("Falling back to ASCII mode...");
                }
                Interpreter::new()
            }
        }
    } else {
        Interpreter::new()
    };

    if !input_values.is_empty() {
        interpreter.set_input_queue(input_values.to_vec());
    }

    // Tokenize
    let mut lexer = Lexer::new(&content);
    let tokens = match lexer.tokenize() {
        Ok(t) => t,
        Err(e) => {
            eprintln!("Lexer error: {}", e);
            std::process::exit(1);
        }
    };

    // Parse
    let mut parser = Parser::new(tokens);
    let ast = match parser.parse() {
        Ok(a) => a,
        Err(e) => {
            eprintln!("Parser error: {}", e);
            std::process::exit(1);
        }
    };

    // Execute (this loads line-numbered programs)
    if let Err(e) = interpreter.execute(ast) {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }

    // If the program had line numbers, run it now
    if let Err(e) = interpreter.run_stored_program() {
        eprintln!("Runtime error: {}", e);
        std::process::exit(1);
    }
}

fn has_display() -> bool {
    env::var("DISPLAY").map(|v| !v.is_empty()).unwrap_or(false)
        || env::var("WAYLAND_DISPLAY").map(|v| !v.is_empty()).unwrap_or(false)
}

fn maybe_reexec_with_x11() {
    let display = env::var("DISPLAY").ok().filter(|v| !v.is_empty());
    let wayland = env::var("WAYLAND_DISPLAY").ok().filter(|v| !v.is_empty());
    let already_reexec = env::var("GWBASIC_X11_REEXEC").ok().filter(|v| v == "1").is_some();

    if display.is_some() && wayland.is_some() && !already_reexec {
        if let Ok(exe) = env::current_exe() {
            let args: Vec<String> = env::args().skip(1).collect();
            let status = Command::new(exe)
                .args(args)
                .env("WAYLAND_DISPLAY", "")
                .env("XDG_SESSION_TYPE", "x11")
                .env("GWBASIC_X11_REEXEC", "1")
                .status();

            match status {
                Ok(s) => std::process::exit(s.code().unwrap_or(1)),
                Err(_) => {
                    // Fall back to current process if re-exec fails.
                }
            }
        }
    }
}
