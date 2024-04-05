#![allow(dead_code)]
use std::{
    collections::HashSet,
    num::NonZeroU8,
    ops::{Index, IndexMut},
};

pub trait Solver {
    type Error: std::fmt::Debug;

    /// Solve a [`Sudoku`].
    ///
    /// # Panics
    ///
    /// This function will panic if no solution is found. If you want to catch this error, use
    /// [`try_solve`] instead.
    ///
    /// [`try_solve`]: Solver::try_solve
    fn solve(&self, sudoku: Sudoku) -> SolvedSudoku {
        self.try_solve(sudoku).expect("couldn't find a solution")
    }

    /// Solve a [`Sudoku`]
    ///
    /// # Errors
    ///
    /// This function will return an error if the [`Solver`] encounters an error trying to solve
    /// this [`Sudoku`]. See the solver documentation for possible errors.
    fn try_solve(&self, sudoku: Sudoku) -> Result<SolvedSudoku, Self::Error>;
}

#[derive(Debug, Clone, Copy)]
pub struct IterativeDFS;

#[derive(Debug)]
pub struct ExhaustedAllPossibilities(pub Sudoku);

impl Solver for IterativeDFS {
    type Error = ExhaustedAllPossibilities;

    fn try_solve(&self, mut sudoku: Sudoku) -> Result<SolvedSudoku, Self::Error> {
        // Get the indexes of all empty cells
        let mut empty_cells: Vec<_> = sudoku
            .indexed_values()
            .filter_map(|(ix, cell)| cell.is_empty().then_some(ix))
            .collect();
        // Keeps track of the cells that have been set, and the value they were set to
        let mut state: Vec<([usize; 2], SudokuValues)> = Vec::with_capacity(empty_cells.len());
        // All values that affect the cell at `ix`
        fn all_affecting(sudoku: &Sudoku, ix: [usize; 2]) -> HashSet<SudokuValue> {
            let row = sudoku
                .row(Sudoku::row_from_ix(ix))
                .filter_map(|cell| SudokuValue::try_from(*cell).ok());
            let column = sudoku
                .column(Sudoku::column_from_ix(ix))
                .filter_map(|cell| SudokuValue::try_from(*cell).ok());
            let cell = sudoku
                .cell(Sudoku::cell_from_ix(ix))
                .filter_map(|cell| SudokuValue::try_from(*cell).ok());
            let mut all = HashSet::with_capacity(9);
            all.extend(row);
            all.extend(column);
            all.extend(cell);
            all
        }
        // Main solver
        'main: loop {
            // println!("state={}", {
            //     let mut s = String::with_capacity(state.len() * 2);
            //     for (_, v) in state.iter() {
            //         write!(s, "{},", v.0).unwrap();
            //     }
            //     s
            // });
            // Fetch the empty cell we will try to solve
            if let Some(ix) = empty_cells.pop() {
                // Fetch current values that affect the current empty cell
                let all = all_affecting(&sudoku, ix);
                // Find the first value that is not contained in `all`
                if let Some(val) = SudokuValue::all_values().find(|v| !all.contains(v)) {
                    // Save the state of the cell
                    state.push((ix, val.into_iter()));
                    sudoku[ix] = SudokuCell::filled(val);
                    // Go back to the top
                    continue 'main;
                }
                // No values are valid for this position. Set the current cell to empty and push it
                // back to the stack of empty cells.
                sudoku[ix] = SudokuCell::empty();
                empty_cells.push(ix);
            } else {
                // There are no more empty cells remaining. We have solved the Sudoku!
                return Ok(
                    SolvedSudoku::try_from(sudoku).expect("sudoku was solved by IterativeDFS")
                );
            }
            // We failed to find a valid value for the current cell; backtrack to the previous cell
            while let Some((ix, mut values)) = state.pop() {
                // Set the current cell to empty, the value we set previously was wrong
                sudoku[ix] = SudokuCell::empty();
                // Fetch current values that affect the current empty cell
                let all = all_affecting(&sudoku, ix);
                // From the values we have yet to try, find the first value which is also valid
                if let Some(val) = values.find(|v| !all.contains(v)) {
                    // We found another candidate value, save current state and continue solving
                    state.push((ix, val.into_iter()));
                    sudoku[ix] = SudokuCell::filled(val);
                    continue 'main;
                }
                // No other values are valid for this position; continue backtracking
                sudoku[ix] = SudokuCell::empty();
                empty_cells.push(ix);
            }
            // We checked all values exhaustively. No more solutions are available (or we got the
            // implementation wrong).
            return Err(ExhaustedAllPossibilities(sudoku));
        }
    }
}

#[derive(Debug, Clone)]
pub struct SudokuValues(u8);

impl Iterator for SudokuValues {
    type Item = SudokuValue;

    fn next(&mut self) -> Option<Self::Item> {
        if self.0 >= 9 {
            return None;
        }
        self.0 += 1;
        Some(unsafe { SudokuValue::new_unchecked(self.0) })
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = 9usize.saturating_sub(self.0.into());
        (left, Some(left))
    }
}

#[repr(transparent)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub struct SudokuValue(NonZeroU8);

impl SudokuValue {
    pub fn new(val: u8) -> Option<Self> {
        (1..=9)
            .contains(&val)
            .then_some(SudokuValue(NonZeroU8::new(val)?))
    }

    pub unsafe fn new_unchecked(val: u8) -> Self {
        SudokuValue(NonZeroU8::new_unchecked(val))
    }

    pub fn all_values() -> SudokuValues {
        SudokuValues(0)
    }
}

impl IntoIterator for SudokuValue {
    type Item = SudokuValue;

    type IntoIter = SudokuValues;

    fn into_iter(self) -> Self::IntoIter {
        SudokuValues(self.0.get())
    }
}

#[derive(Debug)]
pub struct EmptySudokuCell;

impl TryFrom<SudokuCell> for SudokuValue {
    type Error = EmptySudokuCell;

    fn try_from(value: SudokuCell) -> Result<Self, Self::Error> {
        value.0.ok_or(EmptySudokuCell)
    }
}

impl From<SudokuValue> for SudokuCell {
    fn from(value: SudokuValue) -> Self {
        Self::filled(value)
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy, PartialEq)]
pub struct SudokuCell(Option<SudokuValue>);

impl SudokuCell {
    pub fn filled(val: SudokuValue) -> Self {
        Self(Some(val))
    }

    pub fn is_filled(&self) -> bool {
        self.0.is_some()
    }

    pub fn empty() -> Self {
        Self(None)
    }

    pub fn is_empty(&self) -> bool {
        self.0.is_none()
    }

    pub fn from_ascci_char(val: u8) -> Option<Self> {
        if val == b'.' {
            Some(Self::empty())
        } else {
            Some(Self::filled(SudokuValue::new(val.wrapping_sub(b'0'))?))
        }
    }
}

#[derive(Clone)]
pub struct SolvedSudoku([[SudokuValue; 9]; 9]);

impl From<SolvedSudoku> for Sudoku {
    fn from(val: SolvedSudoku) -> Self {
        Self(val.0.map(|arr| arr.map(Into::into)))
    }
}

impl TryFrom<Sudoku> for SolvedSudoku {
    type Error = ();

    fn try_from(value: Sudoku) -> Result<Self, Self::Error> {
        value
            .solved()
            .then_some(Self(value.0.map(|r| {
                r.map(|c| SudokuValue::try_from(c).expect("a solved Sudoku has no empty cells"))
            })))
            .ok_or(())
    }
}

impl<Ix: Into<[usize; 2]>> Index<Ix> for SolvedSudoku {
    type Output = SudokuValue;

    fn index(&self, ix: Ix) -> &Self::Output {
        let [x, y] = ix.into();
        &self.0[y][x]
    }
}

impl<Ix: Into<[usize; 2]>> IndexMut<Ix> for SolvedSudoku {
    fn index_mut(&mut self, ix: Ix) -> &mut Self::Output {
        let [x, y] = ix.into();
        &mut self.0[y][x]
    }
}

pub struct Column<'a> {
    sudoku: &'a Sudoku,
    x: u8,
    y: u8,
}

impl<'a> Iterator for Column<'a> {
    type Item = &'a SudokuCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.y >= 9 {
            return None;
        }
        let ix = [self.x, self.y].map(Into::into);
        self.y += 1;
        Some(&self.sudoku[ix])
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let left = 9usize.saturating_sub(self.y.into());
        (left, Some(left))
    }
}

pub struct Row<'a> {
    sudoku: &'a Sudoku,
    x: u8,
    y: u8,
}

impl<'a> Iterator for Row<'a> {
    type Item = &'a SudokuCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.x >= 9 {
            return None;
        }
        let ix = [self.x, self.y].map(Into::into);
        self.x += 1;
        Some(&self.sudoku[ix])
    }
}

pub struct Cell<'a> {
    sudoku: &'a Sudoku,
    pos: u8,
    ix: u8,
}

impl<'a> Iterator for Cell<'a> {
    type Item = &'a SudokuCell;

    fn next(&mut self) -> Option<Self::Item> {
        if self.ix >= 9 {
            return None;
        }
        let (x, y) = (self.pos % 3, self.pos / 3);
        let (x_off, y_off) = (self.ix % 3, self.ix / 3);
        let ix = [3 * x + x_off, 3 * y + y_off].map(Into::into);
        self.ix += 1;
        Some(&self.sudoku[ix])
    }
}

#[derive(Clone)]
pub struct Sudoku([[SudokuCell; 9]; 9]);

fn unique<'a>(values: impl IntoIterator<Item = &'a SudokuCell>) -> bool {
    let values = values
        .into_iter()
        .copied()
        .filter_map(|c| SudokuValue::try_from(c).ok())
        .collect::<Vec<_>>();
    !values
        .iter()
        .copied()
        .enumerate()
        .any(|(ix, v)| values[ix + 1..].contains(&v))
}

impl Sudoku {
    pub fn from_line(line: &[u8]) -> Self {
        assert_eq!(line.len(), 81);
        let mut sudoku = [[SudokuCell::empty(); 9]; 9];
        for (b, val) in line.iter().copied().zip(sudoku.iter_mut().flatten()) {
            if let Some(v) = SudokuCell::from_ascci_char(b) {
                *val = v;
            } else {
                panic!(
                    "bad SudokuValue: b'{}' expected '.' or [1-9]",
                    b.escape_ascii()
                )
            }
        }
        Self(sudoku)
    }

    pub fn filled(&self) -> bool {
        self.values().all(SudokuCell::is_filled)
    }

    pub fn valid(&self) -> bool {
        (0..9u8).all(|ix| unique(self.row(ix)) && unique(self.column(ix)) && unique(self.cell(ix)))
    }

    pub fn solved(&self) -> bool {
        self.filled() && self.valid()
    }

    pub fn values(&self) -> impl Iterator<Item = &SudokuCell> {
        self.0.iter().flatten()
    }

    pub fn indexed_values(&self) -> impl Iterator<Item = ([usize; 2], &SudokuCell)> {
        self.0
            .iter()
            .flatten()
            .enumerate()
            .map(|(ix, cell)| ([ix % 9, ix / 9], cell))
    }

    pub fn cell(&self, ix: u8) -> Cell<'_> {
        assert!(ix < 9);
        Cell {
            sudoku: self,
            pos: ix,
            ix: 0,
        }
    }

    pub fn row(&self, ix: u8) -> Row<'_> {
        assert!(ix < 9);
        Row {
            sudoku: self,
            x: 0,
            y: ix,
        }
    }

    pub fn column(&self, ix: u8) -> Column<'_> {
        assert!(ix < 9);
        Column {
            sudoku: self,
            x: ix,
            y: 0,
        }
    }

    pub fn row_from_ix(ix: [usize; 2]) -> u8 {
        let [_x, y] = ix;
        y as u8
    }

    pub fn column_from_ix(ix: [usize; 2]) -> u8 {
        let [x, _y] = ix;
        x as u8
    }

    pub fn cell_from_ix(ix: [usize; 2]) -> u8 {
        let [x, y] = ix;
        (3 * (y / 3) + x / 3) as u8
    }
}

impl<Ix: Into<[usize; 2]>> Index<Ix> for Sudoku {
    type Output = SudokuCell;

    fn index(&self, ix: Ix) -> &Self::Output {
        let [x, y] = ix.into();
        &self.0[y][x]
    }
}

impl<Ix: Into<[usize; 2]>> IndexMut<Ix> for Sudoku {
    fn index_mut(&mut self, ix: Ix) -> &mut Self::Output {
        let [x, y] = ix.into();
        &mut self.0[y][x]
    }
}

impl std::fmt::Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0.get() {
            digit @ 1..=9 => write!(f, "{digit}"),
            val => {
                unreachable!(
                    "invalid SudokuValue: {val}, this should not be reachable through safe code"
                )
            }
        }
    }
}

impl std::fmt::Display for SudokuCell {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if let Some(val) = self.0 {
            write!(f, "{val}")
        } else if f.alternate() {
            write!(f, " ")
        } else {
            write!(f, ".")
        }
    }
}

impl std::fmt::Display for SolvedSudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s: Sudoku = self.clone().into();
        write!(f, "{s:#?}")
    }
}

impl std::fmt::Debug for Sudoku {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        if f.alternate() {
            // Pretty print:
            write!(
                f,
                "+-------+-------+-------+
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
+-------+-------+-------+
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
+-------+-------+-------+
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
| {:#} {:#} {:#} | {:#} {:#} {:#} | {:#} {:#} {:#} |
+-------+-------+-------+",
                self[(0, 0)],
                self[(1, 0)],
                self[(2, 0)],
                self[(3, 0)],
                self[(4, 0)],
                self[(5, 0)],
                self[(6, 0)],
                self[(7, 0)],
                self[(8, 0)],
                self[(0, 1)],
                self[(1, 1)],
                self[(2, 1)],
                self[(3, 1)],
                self[(4, 1)],
                self[(5, 1)],
                self[(6, 1)],
                self[(7, 1)],
                self[(8, 1)],
                self[(0, 2)],
                self[(1, 2)],
                self[(2, 2)],
                self[(3, 2)],
                self[(4, 2)],
                self[(5, 2)],
                self[(6, 2)],
                self[(7, 2)],
                self[(8, 2)],
                self[(0, 3)],
                self[(1, 3)],
                self[(2, 3)],
                self[(3, 3)],
                self[(4, 3)],
                self[(5, 3)],
                self[(6, 3)],
                self[(7, 3)],
                self[(8, 3)],
                self[(0, 4)],
                self[(1, 4)],
                self[(2, 4)],
                self[(3, 4)],
                self[(4, 4)],
                self[(5, 4)],
                self[(6, 4)],
                self[(7, 4)],
                self[(8, 4)],
                self[(0, 5)],
                self[(1, 5)],
                self[(2, 5)],
                self[(3, 5)],
                self[(4, 5)],
                self[(5, 5)],
                self[(6, 5)],
                self[(7, 5)],
                self[(8, 5)],
                self[(0, 6)],
                self[(1, 6)],
                self[(2, 6)],
                self[(3, 6)],
                self[(4, 6)],
                self[(5, 6)],
                self[(6, 6)],
                self[(7, 6)],
                self[(8, 6)],
                self[(0, 7)],
                self[(1, 7)],
                self[(2, 7)],
                self[(3, 7)],
                self[(4, 7)],
                self[(5, 7)],
                self[(6, 7)],
                self[(7, 7)],
                self[(8, 7)],
                self[(0, 8)],
                self[(1, 8)],
                self[(2, 8)],
                self[(3, 8)],
                self[(4, 8)],
                self[(5, 8)],
                self[(6, 8)],
                self[(7, 8)],
                self[(8, 8)],
            )
        } else {
            write!(
                f, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                self[(0, 0)],
                self[(1, 0)],
                self[(2, 0)],
                self[(3, 0)],
                self[(4, 0)],
                self[(5, 0)],
                self[(6, 0)],
                self[(7, 0)],
                self[(8, 0)],
                self[(0, 1)],
                self[(1, 1)],
                self[(2, 1)],
                self[(3, 1)],
                self[(4, 1)],
                self[(5, 1)],
                self[(6, 1)],
                self[(7, 1)],
                self[(8, 1)],
                self[(0, 2)],
                self[(1, 2)],
                self[(2, 2)],
                self[(3, 2)],
                self[(4, 2)],
                self[(5, 2)],
                self[(6, 2)],
                self[(7, 2)],
                self[(8, 2)],
                self[(0, 3)],
                self[(1, 3)],
                self[(2, 3)],
                self[(3, 3)],
                self[(4, 3)],
                self[(5, 3)],
                self[(6, 3)],
                self[(7, 3)],
                self[(8, 3)],
                self[(0, 4)],
                self[(1, 4)],
                self[(2, 4)],
                self[(3, 4)],
                self[(4, 4)],
                self[(5, 4)],
                self[(6, 4)],
                self[(7, 4)],
                self[(8, 4)],
                self[(0, 5)],
                self[(1, 5)],
                self[(2, 5)],
                self[(3, 5)],
                self[(4, 5)],
                self[(5, 5)],
                self[(6, 5)],
                self[(7, 5)],
                self[(8, 5)],
                self[(0, 6)],
                self[(1, 6)],
                self[(2, 6)],
                self[(3, 6)],
                self[(4, 6)],
                self[(5, 6)],
                self[(6, 6)],
                self[(7, 6)],
                self[(8, 6)],
                self[(0, 7)],
                self[(1, 7)],
                self[(2, 7)],
                self[(3, 7)],
                self[(4, 7)],
                self[(5, 7)],
                self[(6, 7)],
                self[(7, 7)],
                self[(8, 7)],
                self[(0, 8)],
                self[(1, 8)],
                self[(2, 8)],
                self[(3, 8)],
                self[(4, 8)],
                self[(5, 8)],
                self[(6, 8)],
                self[(7, 8)],
                self[(8, 8)],
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::{IterativeDFS, Solver, Sudoku};

    const TEST_SUDOKU: &[u8; 81] =
        b".......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...";

    const TEST_SUDOKUS: &[&[u8; 81]; 10] = &[
        b".......1.4.........2...........5.4.7..8...3....1.9....3..4..2...5.1........8.6...",
        b".......1.4.........2...........5.6.4..8...3....1.9....3..4..2...5.1........8.7...",
        b".......12....35......6...7.7.....3.....4..8..1...........12.....8.....4..5....6..",
        b".......12..36..........7...41..2.......5..3..7.....6..28.....4....3..5...........",
        b".......12..8.3...........4.12.5..........47...6.......5.7...3.....62.......1.....",
        b".......12.4..5.........9....7.6..4.....1............5.....875..6.1...3..2........",
        b".......12.5.4............3.7..6..4....1..........8....92....8.....51.7.......3...",
        b".......123......6.....4....9.....5.......1.7..2..........35.4....14..8...6.......",
        b".......124...9...........5..7.2.....6.....4.....1.8....18..........3.7..5.2......",
        b".......125....8......7.....6..12....7.....45.....3.....3....8.....5..7...2.......",
    ];

    #[test]
    fn encode_roundtrip_sudoku() {
        for &sudoku in TEST_SUDOKUS {
            let decoded = Sudoku::from_line(sudoku);
            let encoded = format!("{decoded:?}");
            assert_eq!(sudoku, encoded.as_bytes())
        }
    }

    #[test]
    fn solve_sudoku_iterative_dfs() {
        let sudoku = Sudoku::from_line(TEST_SUDOKU);
        let solver = IterativeDFS;
        solver.solve(sudoku);
    }
}
