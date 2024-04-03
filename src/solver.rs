#![allow(dead_code)]
use std::{
    num::NonZeroU8,
    ops::{Index, IndexMut},
};

#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
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

    pub fn all() -> [SudokuValue; 9] {
        [
            unsafe { Self::new_unchecked(1) },
            unsafe { Self::new_unchecked(2) },
            unsafe { Self::new_unchecked(3) },
            unsafe { Self::new_unchecked(4) },
            unsafe { Self::new_unchecked(5) },
            unsafe { Self::new_unchecked(6) },
            unsafe { Self::new_unchecked(7) },
            unsafe { Self::new_unchecked(8) },
            unsafe { Self::new_unchecked(9) },
        ]
    }
}

#[repr(transparent)]
#[derive(Default, Debug, Clone, Copy)]
pub struct SudokuCell(Option<SudokuValue>);

impl SudokuCell {
    pub fn filled(val: SudokuValue) -> Self {
        Self(Some(val))
    }

    pub fn empty() -> Self {
        Self(None)
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
pub struct Sudoku([[SudokuCell; 9]; 9]);

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
}

impl Index<(usize, usize)> for Sudoku {
    type Output = SudokuCell;

    fn index(&self, (x, y): (usize, usize)) -> &Self::Output {
        &self.0[y][x]
    }
}

impl IndexMut<(usize, usize)> for Sudoku {
    fn index_mut(&mut self, (x, y): (usize, usize)) -> &mut Self::Output {
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
                self[(0, 1)],
                self[(0, 2)],
                self[(0, 3)],
                self[(0, 4)],
                self[(0, 5)],
                self[(0, 6)],
                self[(0, 7)],
                self[(0, 8)],
                self[(1, 0)],
                self[(1, 1)],
                self[(1, 2)],
                self[(1, 3)],
                self[(1, 4)],
                self[(1, 5)],
                self[(1, 6)],
                self[(1, 7)],
                self[(1, 8)],
                self[(2, 0)],
                self[(2, 1)],
                self[(2, 2)],
                self[(2, 3)],
                self[(2, 4)],
                self[(2, 5)],
                self[(2, 6)],
                self[(2, 7)],
                self[(2, 8)],
                self[(3, 0)],
                self[(3, 1)],
                self[(3, 2)],
                self[(3, 3)],
                self[(3, 4)],
                self[(3, 5)],
                self[(3, 6)],
                self[(3, 7)],
                self[(3, 8)],
                self[(4, 0)],
                self[(4, 1)],
                self[(4, 2)],
                self[(4, 3)],
                self[(4, 4)],
                self[(4, 5)],
                self[(4, 6)],
                self[(4, 7)],
                self[(4, 8)],
                self[(5, 0)],
                self[(5, 1)],
                self[(5, 2)],
                self[(5, 3)],
                self[(5, 4)],
                self[(5, 5)],
                self[(5, 6)],
                self[(5, 7)],
                self[(5, 8)],
                self[(6, 0)],
                self[(6, 1)],
                self[(6, 2)],
                self[(6, 3)],
                self[(6, 4)],
                self[(6, 5)],
                self[(6, 6)],
                self[(6, 7)],
                self[(6, 8)],
                self[(7, 0)],
                self[(7, 1)],
                self[(7, 2)],
                self[(7, 3)],
                self[(7, 4)],
                self[(7, 5)],
                self[(7, 6)],
                self[(7, 7)],
                self[(7, 8)],
                self[(8, 0)],
                self[(8, 1)],
                self[(8, 2)],
                self[(8, 3)],
                self[(8, 4)],
                self[(8, 5)],
                self[(8, 6)],
                self[(8, 7)],
                self[(8, 8)],
            )
        } else {
            write!(
                f, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                self[(0, 0)],
                self[(0, 1)],
                self[(0, 2)],
                self[(0, 3)],
                self[(0, 4)],
                self[(0, 5)],
                self[(0, 6)],
                self[(0, 7)],
                self[(0, 8)],
                self[(1, 0)],
                self[(1, 1)],
                self[(1, 2)],
                self[(1, 3)],
                self[(1, 4)],
                self[(1, 5)],
                self[(1, 6)],
                self[(1, 7)],
                self[(1, 8)],
                self[(2, 0)],
                self[(2, 1)],
                self[(2, 2)],
                self[(2, 3)],
                self[(2, 4)],
                self[(2, 5)],
                self[(2, 6)],
                self[(2, 7)],
                self[(2, 8)],
                self[(3, 0)],
                self[(3, 1)],
                self[(3, 2)],
                self[(3, 3)],
                self[(3, 4)],
                self[(3, 5)],
                self[(3, 6)],
                self[(3, 7)],
                self[(3, 8)],
                self[(4, 0)],
                self[(4, 1)],
                self[(4, 2)],
                self[(4, 3)],
                self[(4, 4)],
                self[(4, 5)],
                self[(4, 6)],
                self[(4, 7)],
                self[(4, 8)],
                self[(5, 0)],
                self[(5, 1)],
                self[(5, 2)],
                self[(5, 3)],
                self[(5, 4)],
                self[(5, 5)],
                self[(5, 6)],
                self[(5, 7)],
                self[(5, 8)],
                self[(6, 0)],
                self[(6, 1)],
                self[(6, 2)],
                self[(6, 3)],
                self[(6, 4)],
                self[(6, 5)],
                self[(6, 6)],
                self[(6, 7)],
                self[(6, 8)],
                self[(7, 0)],
                self[(7, 1)],
                self[(7, 2)],
                self[(7, 3)],
                self[(7, 4)],
                self[(7, 5)],
                self[(7, 6)],
                self[(7, 7)],
                self[(7, 8)],
                self[(8, 0)],
                self[(8, 1)],
                self[(8, 2)],
                self[(8, 3)],
                self[(8, 4)],
                self[(8, 5)],
                self[(8, 6)],
                self[(8, 7)],
                self[(8, 8)],
            )
        }
    }
}

#[cfg(test)]
mod test {
    use super::Sudoku;

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
}
