#[repr(transparent)]
#[derive(Debug, Clone, Copy)]
struct SudokuValue(u8);

#[derive(Clone)]
pub struct Sudoku([[SudokuValue; 9]; 9]);

impl Sudoku {
    pub fn from_line(line: &[u8]) -> Self {
        assert_eq!(line.len(), 81);
        let mut sudoku = [[SudokuValue(0); 9]; 9];
        for (b, val) in line.iter().copied().zip(sudoku.iter_mut().flatten()) {
            if (b'1'..=b'9').contains(&b) {
                *val = SudokuValue(b - b'0')
            } else if b == b'.' {
                continue;
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

impl std::fmt::Display for SudokuValue {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self.0 {
            digit @ 1..=9 => write!(f, "{digit}"),
            _ => f.write_str("."),
        }
    }
}

impl std::fmt::Debug for Sudoku {
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
                vals[0][1],
                vals[0][2],
                vals[0][3],
                vals[0][4],
                vals[0][5],
                vals[0][6],
                vals[0][7],
                vals[0][8],
                vals[1][0],
                vals[1][1],
                vals[1][2],
                vals[1][3],
                vals[1][4],
                vals[1][5],
                vals[1][6],
                vals[1][7],
                vals[1][8],
                vals[2][0],
                vals[2][1],
                vals[2][2],
                vals[2][3],
                vals[2][4],
                vals[2][5],
                vals[2][6],
                vals[2][7],
                vals[2][8],
                vals[3][0],
                vals[3][1],
                vals[3][2],
                vals[3][3],
                vals[3][4],
                vals[3][5],
                vals[3][6],
                vals[3][7],
                vals[3][8],
                vals[4][0],
                vals[4][1],
                vals[4][2],
                vals[4][3],
                vals[4][4],
                vals[4][5],
                vals[4][6],
                vals[4][7],
                vals[4][8],
                vals[5][0],
                vals[5][1],
                vals[5][2],
                vals[5][3],
                vals[5][4],
                vals[5][5],
                vals[5][6],
                vals[5][7],
                vals[5][8],
                vals[6][0],
                vals[6][1],
                vals[6][2],
                vals[6][3],
                vals[6][4],
                vals[6][5],
                vals[6][6],
                vals[6][7],
                vals[6][8],
                vals[7][0],
                vals[7][1],
                vals[7][2],
                vals[7][3],
                vals[7][4],
                vals[7][5],
                vals[7][6],
                vals[7][7],
                vals[7][8],
                vals[8][0],
                vals[8][1],
                vals[8][2],
                vals[8][3],
                vals[8][4],
                vals[8][5],
                vals[8][6],
                vals[8][7],
                vals[8][8],
            )
        } else {
            write!(
                f, "{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}{}",
                vals[0][0],
                vals[0][1],
                vals[0][2],
                vals[0][3],
                vals[0][4],
                vals[0][5],
                vals[0][6],
                vals[0][7],
                vals[0][8],
                vals[1][0],
                vals[1][1],
                vals[1][2],
                vals[1][3],
                vals[1][4],
                vals[1][5],
                vals[1][6],
                vals[1][7],
                vals[1][8],
                vals[2][0],
                vals[2][1],
                vals[2][2],
                vals[2][3],
                vals[2][4],
                vals[2][5],
                vals[2][6],
                vals[2][7],
                vals[2][8],
                vals[3][0],
                vals[3][1],
                vals[3][2],
                vals[3][3],
                vals[3][4],
                vals[3][5],
                vals[3][6],
                vals[3][7],
                vals[3][8],
                vals[4][0],
                vals[4][1],
                vals[4][2],
                vals[4][3],
                vals[4][4],
                vals[4][5],
                vals[4][6],
                vals[4][7],
                vals[4][8],
                vals[5][0],
                vals[5][1],
                vals[5][2],
                vals[5][3],
                vals[5][4],
                vals[5][5],
                vals[5][6],
                vals[5][7],
                vals[5][8],
                vals[6][0],
                vals[6][1],
                vals[6][2],
                vals[6][3],
                vals[6][4],
                vals[6][5],
                vals[6][6],
                vals[6][7],
                vals[6][8],
                vals[7][0],
                vals[7][1],
                vals[7][2],
                vals[7][3],
                vals[7][4],
                vals[7][5],
                vals[7][6],
                vals[7][7],
                vals[7][8],
                vals[8][0],
                vals[8][1],
                vals[8][2],
                vals[8][3],
                vals[8][4],
                vals[8][5],
                vals[8][6],
                vals[8][7],
                vals[8][8],
            )
        }
    }
}
