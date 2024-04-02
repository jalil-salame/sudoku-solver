use std::{
    io::{stdin, Read},
    process::ExitCode,
};

fn usage(prog: &str) -> String {
    format!("Usage: {prog} [SOURCE]")
}

fn main() -> ExitCode {
    let mut args = std::env::args();

    let Some(prog) = args.next() else {
        eprintln!("[ERROR]: No program name received through arguments");
        return ExitCode::FAILURE;
    };

    let (Some(src), None) = (args.next(), args.next()) else {
        eprintln!("[ERROR]: Invalid number of arguments provided, expected 1\n");
        eprintln!("{}", usage(&prog));
        return ExitCode::FAILURE;
    };

    let _src: Box<dyn Read> = match src.as_str() {
        "-h" => {
            println!("{}", usage(&prog));
            return ExitCode::SUCCESS;
        }
        "--help" => {
            println!("{}", usage(&prog));
            return ExitCode::SUCCESS;
        }
        "help" => {
            println!("{}", usage(&prog));
            return ExitCode::SUCCESS;
        }
        "-" => Box::new(stdin().lock()),
        path => Box::new(match std::fs::File::open(path) {
            Ok(file) => file,
            Err(err) => {
                eprintln!("[ERROR]: failed to open file {path}: {err}");
                return ExitCode::FAILURE;
            }
        }),
    };

    ExitCode::SUCCESS
}
