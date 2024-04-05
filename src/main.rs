use std::{
    io::{stdin, Read},
    ops::ControlFlow,
    process::ExitCode,
};

use crate::solver::{Solver, Sudoku};

mod solver;

/// Program usage messaeg
fn usage(prog: &str) -> String {
    format!("Usage: {prog} [SOURCE]")
}

fn cli() -> ControlFlow<ExitCode, (String, Box<[u8]>)> {
    let mut args = std::env::args();
    let Some(prog) = args.next() else {
        eprintln!("[ERROR]: No program name received through arguments");
        return ControlFlow::Break(ExitCode::FAILURE);
    };
    let (Some(src_path), None) = (args.next(), args.next()) else {
        eprintln!("[ERROR]: Invalid number of arguments provided, expected 1\n");
        eprintln!("{}", usage(&prog));
        return ControlFlow::Break(ExitCode::FAILURE);
    };
    let src: Box<[u8]> = match src_path.as_str() {
        "-h" => {
            println!("{}", usage(&prog));
            return ControlFlow::Break(ExitCode::SUCCESS);
        }
        "--help" => {
            println!("{}", usage(&prog));
            return ControlFlow::Break(ExitCode::SUCCESS);
        }
        "help" => {
            println!("{}", usage(&prog));
            return ControlFlow::Break(ExitCode::SUCCESS);
        }
        "-" => {
            let mut stdin = stdin().lock();
            let mut v = vec![];
            if let Err(err) = stdin.read_to_end(&mut v) {
                eprintln!("[ERROR]: failed read from stdin: {err}");
                return ControlFlow::Break(ExitCode::FAILURE);
            };
            v.into()
        }
        path => match std::fs::read(path) {
            Ok(v) => v.into(),
            Err(err) => {
                eprintln!("[ERROR]: failed read from file {path}: {err}");
                return ControlFlow::Break(ExitCode::FAILURE);
            }
        },
    };
    ControlFlow::Continue((src_path, src))
}

fn main() -> ExitCode {
    let (src_path, src) = match cli() {
        ControlFlow::Continue(src) => src,
        ControlFlow::Break(code) => return code,
    };

    // Read source contents
    let start = std::time::Instant::now();
    let total = start;
    let contents: Vec<u8> = match src.bytes().collect() {
        Ok(bytes) => bytes,
        Err(err) => {
            eprintln!("[ERROR]: failed to read contents of file {src_path}: {err}");
            return ExitCode::FAILURE;
        }
    };
    eprintln!(
        "[INFO]: Reading the file took {:.3}ms",
        1000f32 * start.elapsed().as_secs_f32()
    );

    // Parse Sudokus
    let start = std::time::Instant::now();
    let sudokus: Vec<_> = contents
        .split(u8::is_ascii_whitespace)
        .filter(|s| !s.is_empty())
        .map(|line| {
            let s = Sudoku::from_line(line);
            debug_assert_eq!(line, format!("{s:?}").as_bytes());
            s
        })
        .collect();
    let count = sudokus.len();
    let parsing = start.elapsed();
    let total = total.elapsed();
    eprintln!(
        "[INFO]: Parsing the {count} Sudokus took {:.3}ms",
        1000f32 * parsing.as_secs_f32()
    );
    eprintln!(
        "        that is {:.3}us per sudoku",
        1_000_000f32 * parsing.as_secs_f32() / count as f32
    );
    eprintln!("[INFO]: Total time {}s", total.as_secs_f32());

    let start = std::time::Instant::now();
    let _solved: Vec<_> = sudokus
        .into_iter()
        .enumerate()
        .map(|(ix, sudoku)| {
            eprint!("[INFO]: Solving {}/{count}\r", ix + 1);
            solver::IterativeDFS.solve(sudoku)
        })
        .collect();
    let solving = start.elapsed().as_secs_f32();
    eprintln!(
        "[INFO]: Solved {count} sudokus in {solving:.3}s, that is {:.3}ms per sudoku",
        1000f32 * solving / count as f32
    );

    // Done!
    ExitCode::SUCCESS
}
