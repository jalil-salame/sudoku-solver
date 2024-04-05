use iai_callgrind::{library_benchmark, library_benchmark_group, main};
use libsolver::solver::{IterativeDFS, Solver, Sudoku};

const SUDOKU: &[u8; 81] =
    b".......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...";

#[library_benchmark]
#[bench::first(Sudoku::from_line(SUDOKU))]
fn solve_sudoku(sudoku: Sudoku) {
    std::hint::black_box(IterativeDFS.solve(sudoku));
}

library_benchmark_group!(
    name = solve_sudoku_group;
    benchmarks = solve_sudoku,
);

main!(library_benchmark_groups = solve_sudoku_group);
