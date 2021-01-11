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
pub fn solve_puzzle(puzzle: &mut Sudoku) {
    solve(puzzle, 0, 0);
}

// Fills in the empty positions in the puzzle with the right values, using a
// recursive brute force approach. Modify the puzzle in place. Return true if
// solved successfully, false if unsuccessful. You may modify the function signature
// if you need/wish.
fn solve(puzzle: &mut Sudoku, mut row: usize, mut col: usize) -> bool {
    false
}

// Helper that checks if a specific square in the puzzle can take on
// a given value. Return true if that value is allowed in that square, false otherwise.
// You can choose not to use this if you prefer.
fn check_square(puzzle: &Sudoku, row: usize, col: usize, val: NonZeroU8) -> bool {
    false
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


    Some(puzzle)
}

// Do a simple check that the puzzle is valid.
// Returns true if it is valid, false if it is not.
// (The verifier server doesn't tell you what's wrong so this function can also help you track
// down an error if your puzzles are not being solved correctly.)
pub fn check_puzzle(puzzle: &Sudoku) -> bool {
    // Check that each row is valid

    // Check that each column is valid

    // Check that each subgrid is valid

    true
}
