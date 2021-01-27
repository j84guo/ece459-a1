// Library routines for reading and solving Sudoku puzzles

#![warn(clippy::all)]
pub mod verify;

use std::io::Read;
use std::num::NonZeroU8;

// Type definition for a 9x9 array that will represent a Sudoku puzzle.
// Entries with None represent unfilled positions in the puzzle.
type Sudoku = [[Option<NonZeroU8>; 9]; 9];

// This function is called by main. It calls solve() to recursively find the solution.
// The puzzle is modified in-place.
pub fn solve_puzzle(board: &mut Sudoku) {
    let mut row_vals = [[false; 9]; 9];
    let mut col_vals = [[false; 9]; 9];
    let mut grid_vals = [[false; 9]; 9];

    for r in 0usize..=8 {
        for c in 0usize..=8 {
            match board[r][c] {
                Some(d) => {
                    let g = grid_num(r, c);
                    let i = (d.get() - 1) as usize;
                    if row_vals[r][i] || col_vals[c][i] || grid_vals[g][i] {
                        panic!("Invalid initial board!");
                    }
                    row_vals[r][i] = true;
                    col_vals[c][i] = true;
                    grid_vals[g][i] = true;
                },
                None => continue,
            }
        }
    }

    // Expect true, although we don't check
    solve_sudoku_from(board, 0, 0, &mut row_vals, &mut col_vals, &mut grid_vals);
}

fn grid_num(r: usize, c: usize) -> usize {
    // We could compute the grid num each time it's needed, but it's faster to make the grid nums
    // statically allocated - there are only 9x9=81 of them anyways.
    //
    // If we wanted to compute the grid nums, the formula is:
    // let mut g = r / 3;
    // g *= 3;
    // g += c / 3;
    // return g;
    static GRID_NUMS: [[usize; 9]; 9] = [
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [0, 0, 0, 1, 1, 1, 2, 2, 2],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [3, 3, 3, 4, 4, 4, 5, 5, 5],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
        [6, 6, 6, 7, 7, 7, 8, 8, 8],
    ];
    return GRID_NUMS[r][c];
}

fn solve_sudoku_from(board: &mut Sudoku, r: usize, c: usize, row_vals: &mut [[bool; 9]; 9],
                     col_vals: &mut [[bool; 9]; 9], grid_vals: &mut [[bool; 9]; 9]) -> bool {
    match board[r][c] {
        Some(_d) => {
            if c + 1 < board[0].len() {
                return solve_sudoku_from(board, r, c + 1, row_vals, col_vals, grid_vals);
            } else if r + 1 < board.len() {
                return solve_sudoku_from(board, r + 1, 0, row_vals, col_vals, grid_vals);
            }
            return true;
        },
        None => {
            let g = grid_num(r, c);
            for i in 0usize..=8 {
                if row_vals[r][i] || col_vals[c][i] || grid_vals[g][i] {
                    continue;
                }
                board[r][c] = NonZeroU8::new((i + 1) as u8);
                row_vals[r][i] = true;
                col_vals[c][i] = true;
                grid_vals[g][i] = true;
                if c + 1 < board[0].len() {
                    if solve_sudoku_from(board, r, c + 1, row_vals, col_vals, grid_vals) {
                        return true;
                    }
                } else if r + 1 < board.len() {
                    if solve_sudoku_from(board, r + 1, 0, row_vals, col_vals, grid_vals) {
                        return true;
                    }
                } else {
                    return true;
                }
                board[r][c] = None;
                row_vals[r][i] = false;
                col_vals[c][i] = false;
                grid_vals[g][i] = false;
            }
            return false;
        }
    }
}

// Helper for printing a sudoku puzzle to stdout for debugging.
pub fn print_puzzle(puzzle: &Sudoku) {
    for row in puzzle.iter() {
        for elem in row.iter() {
            print!("{}", elem.map(|e| (e.get() + b'0') as char).unwrap_or('.'));
        }
        print!("\n");
    }
    print!("\n");
}

// Read the input byte by byte until a complete Sudoku puzzle has been
// read or EOF is reached.  Assumes the input follows the correct format
// (i.e. matching the files in the input folder).
pub fn read_puzzle(reader: &mut impl Read) -> Option<Box<Sudoku>> {
    // Turn the input stream into an iterator of bytes
    let mut bytes = reader.bytes().map(|b| b.expect("input error")).peekable();
    // Go thru the input until we find a puzzle or EOF (None)
    loop {
        match bytes.peek() {
            Some(b'1'..=b'9') | Some(b'.') => break,
            None => return None,
            _ => {
                bytes.next();
            }
        }
    }

    let mut puzzle = Box::new([[None; 9]; 9]);
    // Fill in the puzzle matrix. Ignore the non-puzzle input bytes.
    for i in 0..9 {
        let mut j = 0;
        while j < 9 {
            let b = bytes.next().expect("unexpected EOF");

            let elem = match b {
                b'1'..=b'9' => NonZeroU8::new(b - b'0'),
                b'.' => None,
                _ => continue,
            };
            puzzle[i][j] = elem;
            j += 1;
        }
    }
    return Some(puzzle);
}

// Do a simple check that the puzzle is valid.
// Returns true if it is valid, false if it is not.
// (The verifier server doesn't tell you what's wrong so this function can also help you track
// down an error if your puzzles are not being solved correctly.)
pub fn check_puzzle(puzzle: &Sudoku) -> bool {
    // Check that each row is valid
    for r in 0..9 {
        let mut row_vals = [false; 9];
        for c in 0..9 {
            match puzzle[r][c] {
                None => return false,
                Some(val) => {
                    let val = val.get() as usize;
                    if row_vals[val - 1] {
                        return false;
                    }
                    row_vals[val - 1] = true;
                }
            }
        }
    }

    // Check that each column is valid
    for c in 0..9 {
        let mut col_vals = [false; 9];
        for r in 0..9 {
            match puzzle[r][c] {
                None => return false,
                Some(val) => {
                    let val = val.get() as usize;
                    if col_vals[val - 1] {
                        return false;
                    }
                    col_vals[val - 1] = true;
                }
            }
        }
    }

    // Check that each 3x3 box is valid
    for i in (0..7).step_by(3) {
        for j in (0..7).step_by(3) {
            let mut grid_vals = [false; 9];
            let mut r = i;
            while r < i + 3 {
                let mut c = j;
                while c < j + 3 {
                    match puzzle[r][c] {
                        None => return false,
                        Some(val) => {
                            let val = val.get() as usize;
                            if grid_vals[val - 1] {
                                return false;
                            }
                            grid_vals[val - 1] = true;
                        }
                    }
                    c += 1;
                }
                r += 1;
            }
        }
    }

    return true;
}
