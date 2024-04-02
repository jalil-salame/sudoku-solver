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

I recommend using Abhinav Sarkar's great [set of 17 clues sudokus][4]. You can obtain it like so:

```console
$ curl -LO https://abhinavsarkar.net/files/sudoku17.txt.bz2
...
$ bunzip2 sudoku17.txt.bz2
```

[1]: https://jalil-salame.github.io
[2]: https://abhinavsarkar.net/
[3]: https://abhinavsarkar.net/posts/fast-sudoku-solver-in-haskell-1/
[4]: https://abhinavsarkar.net/files/sudoku17.txt.bz2
