# Sudoku Solver

A sudoku-solver I am developing for my [blog][1].

I got the idea from Abhinav Sarkar's [blog][2] and his great [series][3] about a fast sudoku solver in Haskell.

## Building

If you have a rust toolchain then:

```console
$ cargo build
...
```

Alternatively, if you have nix:

```console
$ nix build -L .#
...
```

## Testing

The sudoku solver expects a sudoku puzzle through a file (or stdin) in the following format:

- All cells are provided as a flat array of ascii characters (81 elements).
- Every nine characters represents a single row.
- Empty cells are denoted by `.`.
- Filled cells are denoted by the digit they contain (e.g. `9`).
- Multiple sudokus can be provided by delimiting them with ascii whitespace.

For example:

```console
$ cargo run -- -
.......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...
[INFO]: Reading the file took 0.021ms
[INFO]: Parsing the 1 Sudokus took 0.018ms
        that is 18.234us per sudoku
[INFO]: Total time 0.000063548s
```

You can run the tests through cargo. **Warning:** the tests are very slow to run so we recommend running them in release mode:

```console
$ cargo test --release
```

### Testing data

I recommend using Abhinav Sarkar's great [set of 17 clues sudokus][4]. You can obtain it like so:

```console
$ curl -LO https://abhinavsarkar.net/files/sudoku17.txt.bz2
...
$ bunzip2 sudoku17.txt.bz2
```

Alternatively, it is provided by nix in the devshell:

```console
$ nix develop --command "$SHELL"
$ head $SUDOKU17
.......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...
.......1.4.........2...........5.6.4..8...3....1.9....3..4..2...5.1........8.7...
.......12....35......6...7.7.....3.....4..8..1...........12.....8.....4..5....6..
.......12..36..........7...41..2.......5..3..7.....6..28.....4....3..5...........
.......12..8.3...........4.12.5..........47...6.......5.7...3.....62.......1.....
.......12.4..5.........9....7.6..4.....1............5.....875..6.1...3..2........
.......12.5.4............3.7..6..4....1..........8....92....8.....51.7.......3...
.......123......6.....4....9.....5.......1.7..2..........35.4....14..8...6.......
.......124...9...........5..7.2.....6.....4.....1.8....18..........3.7..5.2......
.......125....8......7.....6..12....7.....45.....3.....3....8.....5..7...2.......
```

And as a nix package:

```console
$ nix build .#sudoku17
$ head result
.......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...
.......1.4.........2...........5.6.4..8...3....1.9....3..4..2...5.1........8.7...
.......12....35......6...7.7.....3.....4..8..1...........12.....8.....4..5....6..
.......12..36..........7...41..2.......5..3..7.....6..28.....4....3..5...........
.......12..8.3...........4.12.5..........47...6.......5.7...3.....62.......1.....
.......12.4..5.........9....7.6..4.....1............5.....875..6.1...3..2........
.......12.5.4............3.7..6..4....1..........8....92....8.....51.7.......3...
.......123......6.....4....9.....5.......1.7..2..........35.4....14..8...6.......
.......124...9...........5..7.2.....6.....4.....1.8....18..........3.7..5.2......
.......125....8......7.....6..12....7.....45.....3.....3....8.....5..7...2.......
```

[1]: https://jalil-salame.github.io
[2]: https://abhinavsarkar.net/
[3]: https://abhinavsarkar.net/posts/fast-sudoku-solver-in-haskell-1/
[4]: https://abhinavsarkar.net/files/sudoku17.txt.bz2
