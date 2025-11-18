mod error;
mod run;
mod repl;

use std::env;
use std::path::Path;
use error::{CliError, ExitCode};

fn main() {
    let args: Vec<String> = env::args().collect();
    
    let exit_code = match args.len() {
        1 => {
            // No arguments - run REPL
            match repl::repl() {
                Ok(_) => ExitCode::Success,
                Err(e) => {
                    eprintln!("Error: {}", e);
                    ExitCode::RuntimeError
                }
            }
        },
        2 => {
            let arg = &args[1];
            if arg == "repl" || arg == "--repl" || arg == "-i" {
                // Explicit REPL
                match repl::repl() {
                    Ok(_) => ExitCode::Success,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        ExitCode::RuntimeError
                    }
                }
            } else if arg == "help" || arg == "--help" || arg == "-h" {
                print_usage();
                ExitCode::Success
            } else {
                // Treat as file path
                let path = Path::new(arg);
                match run::run_file(path) {
                    Ok(code) => code,
                    Err(e) => {
                        eprintln!("Error: {}", e);
                        ExitCode::RuntimeError
                    }
                }
            }
        },
        _ => {
            eprintln!("{}", CliError::UsageError("Too many arguments".into()));
            print_usage();
            ExitCode::CompileError
        }
    };
    
    std::process::exit(exit_code as i32);
}

fn print_usage() {
    println!("Brief Language Interpreter");
    println!();
    println!("Usage:");
    println!("  brief [file.bf]    Run a Brief source file");
    println!("  brief repl          Start the REPL");
    println!("  brief help          Show this help message");
    println!();
    println!("If no arguments are provided, the REPL is started.");
}
