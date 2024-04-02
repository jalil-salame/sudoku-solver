use std::{
    fmt::{Debug, Display},
    io::{stdin, Read},
    process::ExitCode,
};

fn usage(prog: &str) -> String {
    format!("Usage: {prog} [SOURCE]")
}

fn main() -> ExitCode {
    // Handle CLI arguments
    let mut args = std::env::args();
    let Some(prog) = args.next() else {
        eprintln!("[ERROR]: No program name received through arguments");
        return ExitCode::FAILURE;
    };
    let (Some(src_path), None) = (args.next(), args.next()) else {
        eprintln!("[ERROR]: Invalid number of arguments provided, expected 1\n");
        eprintln!("{}", usage(&prog));
        return ExitCode::FAILURE;
    };
    let src: Box<[u8]> = match src_path.as_str() {
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
        "-" => {
            let mut stdin = stdin().lock();
            let mut v = vec![];
            if let Err(err) = stdin.read_to_end(&mut v) {
                eprintln!("[ERROR]: failed read from stdin: {err}");
                return ExitCode::FAILURE;
            };
            v.into()
        }
        path => match std::fs::read(path) {
            Ok(v) => v.into(),
            Err(err) => {
                eprintln!("[ERROR]: failed read from file {path}: {err}");
                return ExitCode::FAILURE;
            }
        },
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
            let s = parse_sudoku_line(line);
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

    // TODO: Solve sudokus

    // Done!
    ExitCode::SUCCESS
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct SudokuValue(u8);

impl Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            digit @ 1..=9 => write!(f, "{digit}"),
            _ => f.write_str("."),
        }
    }
}

#[derive(Clone)]
struct Sudoku([[SudokuValue; 9]; 9]);

impl Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let vals = self.0;
        if f.alternate() {
            // Pretty print:
            write!(
                f,
                "+-------+-------+-------+
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
+-------+-------+-------+
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
+-------+-------+-------+
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
| {} {} {} | {} {} {} | {} {} {} |
+-------+-------+-------+",
                vals[0][0],
                vals[1][0],
                vals[2][0],
                vals[3][0],
                vals[4][0],
                vals[5][0],
                vals[6][0],
                vals[7][0],
                vals[8][0],
                vals[0][1],
                vals[1][1],
                vals[2][1],
                vals[3][1],
                vals[4][1],
                vals[5][1],
                vals[6][1],
                vals[7][1],
                vals[8][1],
                vals[0][2],
                vals[1][2],
                vals[2][2],
                vals[3][2],
                vals[4][2],
                vals[5][2],
                vals[6][2],
                vals[7][2],
                vals[8][2],
                vals[0][3],
                vals[1][3],
                vals[2][3],
                vals[3][3],
                vals[4][3],
                vals[5][3],
                vals[6][3],
                vals[7][3],
                vals[8][3],
                vals[0][4],
                vals[1][4],
                vals[2][4],
                vals[3][4],
                vals[4][4],
                vals[5][4],
                vals[6][4],
                vals[7][4],
                vals[8][4],
                vals[0][5],
                vals[1][5],
                vals[2][5],
                vals[3][5],
                vals[4][5],
                vals[5][5],
                vals[6][5],
                vals[7][5],
                vals[8][5],
                vals[0][6],
                vals[1][6],
                vals[2][6],
                vals[3][6],
                vals[4][6],
                vals[5][6],
                vals[6][6],
                vals[7][6],
                vals[8][6],
                vals[0][7],
                vals[1][7],
                vals[2][7],
                vals[3][7],
                vals[4][7],
                vals[5][7],
                vals[6][7],
                vals[7][7],
                vals[8][7],
                vals[0][8],
                vals[1][8],
                vals[2][8],
                vals[3][8],
                vals[4][8],
                vals[5][8],
                vals[6][8],
                vals[7][8],
                vals[8][8],
            )
        } else {
            write!(
                f, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                vals[0][0],
                vals[1][0],
                vals[2][0],
                vals[3][0],
                vals[4][0],
                vals[5][0],
                vals[6][0],
                vals[7][0],
                vals[8][0],
                vals[0][1],
                vals[1][1],
                vals[2][1],
                vals[3][1],
                vals[4][1],
                vals[5][1],
                vals[6][1],
                vals[7][1],
                vals[8][1],
                vals[0][2],
                vals[1][2],
                vals[2][2],
                vals[3][2],
                vals[4][2],
                vals[5][2],
                vals[6][2],
                vals[7][2],
                vals[8][2],
                vals[0][3],
                vals[1][3],
                vals[2][3],
                vals[3][3],
                vals[4][3],
                vals[5][3],
                vals[6][3],
                vals[7][3],
                vals[8][3],
                vals[0][4],
                vals[1][4],
                vals[2][4],
                vals[3][4],
                vals[4][4],
                vals[5][4],
                vals[6][4],
                vals[7][4],
                vals[8][4],
                vals[0][5],
                vals[1][5],
                vals[2][5],
                vals[3][5],
                vals[4][5],
                vals[5][5],
                vals[6][5],
                vals[7][5],
                vals[8][5],
                vals[0][6],
                vals[1][6],
                vals[2][6],
                vals[3][6],
                vals[4][6],
                vals[5][6],
                vals[6][6],
                vals[7][6],
                vals[8][6],
                vals[0][7],
                vals[1][7],
                vals[2][7],
                vals[3][7],
                vals[4][7],
                vals[5][7],
                vals[6][7],
                vals[7][7],
                vals[8][7],
                vals[0][8],
                vals[1][8],
                vals[2][8],
                vals[3][8],
                vals[4][8],
                vals[5][8],
                vals[6][8],
                vals[7][8],
                vals[8][8],
            )
        }
    }
}

fn parse_sudoku_line(line: &[u8]) -> Sudoku {
    let mut sudoku = [[SudokuValue(0); 9]; 9];
    assert_eq!(line.len(), 81);
    for (i, b) in line.iter().copied().enumerate() {
        let (x, y) = (i % 9, i / 9);
        if let b'1'..=b'9' = b {
            sudoku[x][y] = SudokuValue(b - b'0')
        }
    }
    Sudoku(sudoku)
}
