// This module verifies Sudoku puzzles using curl

use crate::Sudoku;
use curl::easy;
use curl::multi;
use std::io::Write;
use curl::multi::Easy2Handle;

const URL: &str = "54.209.48.141:4590/verify";  // the verification server
const MATRIX_LENGTH: usize = 202;

// Callback handler for the curl easy handles
struct SudokuHandler {
    pub result: bool,
    puzzle: Box<Sudoku>,
}

impl SudokuHandler {
    pub fn new(puzzle: Box<Sudoku>) -> Self {
        return Self {
            puzzle,
            result: false,
        };
    }
}

impl easy::Handler for SudokuHandler {  // this defines callbacks for curl to use
    // this function is called by curl when data has arrived from the server
    fn write(&mut self, data: &[u8]) -> Result<usize, easy::WriteError> {
        match std::str::from_utf8(data) {
            Ok(resp) => {
                self.result = resp == "1";  // "1" means verification succeeded
                // println!("Server returned: {}", resp);
            }
            Err(_) => println!("Garbage server response"),
        }
        return Ok(data.len());  // tell curl how many bytes we processed from data
    }

    // this function is called by curl when it wants more data to send to the server
    fn read(&mut self, mut data: &mut [u8]) -> Result<usize, easy::ReadError> {
        let start = data.as_ptr();  // keep track of current offset in data
        write_puzzle_to_json(&self.puzzle, &mut data).expect("JSON writing error");
        let end = data.as_ptr();  // find data offset
        let len = end as usize - start as usize;
        return Ok(len);  // tell curl how many bytes we wrote into data
    }
}

// helper function for setting up a curl "easy" handle
fn create_easy(puzzle: Box<Sudoku>) -> Result<easy::Easy2<SudokuHandler>, curl::Error> {
    let handler = SudokuHandler::new(puzzle);
    let mut easy = easy::Easy2::new(handler);
    let mut headers = easy::List::new();  // HTTP headers

    headers.append("Content-Type: application/json")?;
    headers.append("Expect:")?;

    easy.http_headers(headers)?;
    easy.url(URL)?;
    easy.post(true)?;  // we use HTTP "POST" instead of "GET"
    easy.post_field_size(MATRIX_LENGTH as u64)?;
    return Ok(easy);  // result is a curl easy handle
}

fn create_multi(max_total_connections: usize) -> Result<multi::Multi, curl::MultiError> {
    let mut multi = multi::Multi::new();
    multi.pipelining(true, true)?;
    multi.set_max_total_connections(max_total_connections)?;
    return Ok(multi);
}

// convert a puzzle into JSON format to send to the server
fn write_puzzle_to_json(puzzle: &Sudoku, writer: &mut impl Write) -> std::io::Result<()> {
    write!(writer, "{{\"content\": [")?;

    for (i, row) in puzzle.iter().enumerate() {
        write!(writer, "[")?;

        for (j, elem) in row.iter().enumerate() {
            let val = elem.map(|e| e.get()).unwrap_or(0);
            write!(writer, "{}", val)?;
            if j < 8 {
                write!(writer, ",")?;
            }
        }

        write!(writer, "]")?;
        if i < 8 {
            write!(writer, ", ")?;
        }
    }

    write!(writer, "]}}")?;
    return Ok(());
}

// This function is called from main to verify all of the puzzles
pub fn verify_puzzles(puzzles: impl Iterator<Item = Box<Sudoku>>, max_total_connections: usize) {
    verify_puzzles_multi_poll(puzzles, max_total_connections);
    // verify_puzzles_easy(puzzles, max_total_connections);
}

// Use a multi handle - poll the easy handles using curl_multi_wait and curl_multi_perform
fn verify_puzzles_multi_poll(puzzles: impl Iterator<Item = Box<Sudoku>>, max_total_connections: usize) {
    let mut total = 0;
    let mut verified = 0;

    let mut owned_easies: Vec<Easy2Handle<SudokuHandler>> = vec![];
    let multi = create_multi(max_total_connections).unwrap();

    // Add all easy handles to the multi
    for puzzle in puzzles {
        let easy = create_easy(puzzle).unwrap();
        let mut owned_easy = multi.add2(easy).unwrap();
        owned_easy.set_token(owned_easies.len());
        owned_easies.push(owned_easy);
        total += 1;
    }

    // Wait until they're all done
    while multi.perform().unwrap() > 0 {
        multi.wait(&mut[], std::time::Duration::from_secs(30));
    }

    for owned_easy in owned_easies.into_iter() {
        let mut easy = multi.remove2(owned_easy).unwrap();
        if easy.get_ref().result {
            verified += 1;
        }
    }

    println!("Verified {} out of {}", verified, total);
}

// Use easy handles in a single thread
fn verify_puzzles_easy(puzzles: impl Iterator<Item = Box<Sudoku>>, num_connections: usize) {
    let mut total = 0;
    let mut verified = 0;

    // the following is the single-threaded version
    for puzzle in puzzles {
        let easy = create_easy(puzzle).unwrap();
        easy.perform().unwrap();
        if easy.get_ref().result { verified += 1; }
        total += 1;
    }

    println!("Verified {} out of {}", verified, total);
}
