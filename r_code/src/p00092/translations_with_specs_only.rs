// largest_square.rs
//
// Translates the provided specification into Rust.
// Reads multiple test cases from stdin. Each test case starts with an integer N (map_size).
// If N == 0 the program terminates. Otherwise N rows follow (each read as a token, like C's scanf("%s", ...)).
// '*' denotes blocked cells; any other character denotes a free cell.
// For each grid the program prints the side length of the largest axis-aligned square containing no '*'.
//
// Notes on faithful choices:
// - Row input is read as whitespace-separated tokens (mirrors typical C scanf("%s", ...)).
// - If a provided row token has fewer than N characters, this code treats missing characters as blocked ('*').
//   The specification declares such inputs as unspecified; this choice avoids accidentally claiming extra free cells.
// - Uses an O(N^2) algorithm: for each row we maintain a histogram of consecutive free cells above (including current row),
//   and for each row compute the largest square that can be formed using a monotonic stack (histogram-based).
// - The program assumes well-formed input as per the specification.

use std::cmp;
use std::io::{self, Read};

fn main() {
    // Read entire stdin into a string buffer.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // If reading stdin fails, exit gracefully (spec says environment has stdin).
        return;
    }

    let bytes = input.as_bytes();
    let mut pos: usize = 0;
    let len = bytes.len();

    // Helper: skip ASCII whitespace
    fn skip_ws(bytes: &[u8], pos: &mut usize) {
        let len = bytes.len();
        while *pos < len {
            match bytes[*pos] {
                b' ' | b'\n' | b'\r' | b'\t' | b'\x0c' | b'\x0b' => *pos += 1,
                _ => break,
            }
        }
    }

    // Helper: read next non-whitespace token (like scanf("%s"))
    fn read_token(bytes: &[u8], pos: &mut usize) -> Option<String> {
        skip_ws(bytes, pos);
        if *pos >= bytes.len() {
            return None;
        }
        let start = *pos;
        while *pos < bytes.len() {
            match bytes[*pos] {
                b' ' | b'\n' | b'\r' | b'\t' | b'\x0c' | b'\x0b' => break,
                _ => *pos += 1,
            }
        }
        // Convert slice to String; input may be ASCII per specification, but handle UTF-8 safely.
        match std::str::from_utf8(&bytes[start..*pos]) {
            Ok(s) => Some(s.to_string()),
            Err(_) => {
                // If not valid UTF-8, replace invalid sequences with the replacement character.
                Some(String::from_utf8_lossy(&bytes[start..*pos]).to_string())
            }
        }
    }

    // Helper: read next integer token, parsed as i64
    fn read_int(bytes: &[u8], pos: &mut usize) -> Option<i64> {
        read_token(bytes, pos).and_then(|tok| tok.parse::<i64>().ok())
    }

    // Main processing loop: read map sizes and grids
    loop {
        // Read next integer N (map_size)
        let n_opt = read_int(bytes, &mut pos);
        let n_val = match n_opt {
            Some(v) => v,
            None => break, // EOF or no more integers
        };

        if n_val == 0 {
            // Termination signal: program ends without producing output for this case.
            break;
        }
        // If negative, specification says behavior unspecified. We treat negative as termination.
        if n_val < 0 {
            break;
        }

        // Convert to usize for indexing. If extremely large, this may fail at allocation; spec limits to <=1000.
        let n_usize = n_val as usize;

        // Read N row tokens. Per specification, each row token should have at least N characters.
        // We read them as tokens (whitespace-separated), similar to C's scanf("%s", ...).
        let mut rows: Vec<String> = Vec::with_capacity(n_usize);
        for _ in 0..n_usize {
            // If token missing (EOF/malformed), treat as empty string (spec marks behavior unspecified).
            let row = read_token(bytes, &mut pos).unwrap_or_else(|| String::new());
            rows.push(row);
        }

        // heights[c] = number of consecutive free (non-'*') cells up to current row at column c.
        let mut heights: Vec<usize> = vec![0; n_usize];
        let mut best: usize = 0;

        for r in 0..n_usize {
            let row_bytes = rows[r].as_bytes();
            // Update heights for this row.
            for c in 0..n_usize {
                // If the row token is shorter than N, treat missing characters as blocked ('*').
                let is_free = if c < row_bytes.len() {
                    row_bytes[c] != b'*'
                } else {
                    // Choice: treat missing as blocked, conservative behavior.
                    false
                };
                if is_free {
                    heights[c] += 1;
                } else {
                    heights[c] = 0;
                }
            }

            // Compute largest square for this histogram (heights) using monotonic increasing stack.
            // Stack elements are (start_index, height)
            let mut stack: Vec<(usize, usize)> = Vec::with_capacity(n_usize);
            for c in 0..n_usize {
                let h = heights[c];
                let mut start = c;
                // Pop taller heights and compute possible square sides.
                while let Some(&(idx, ht)) = stack.last() {
                    if ht > h {
                        stack.pop();
                        let width = c - idx;
                        let side = cmp::min(ht, width);
                        if side > best {
                            best = side;
                        }
                        start = idx;
                    } else {
                        break;
                    }
                }
                if h > 0 {
                    // If equal height exists at top, we don't need to push duplicate; but pushing is harmless.
                    // Use the earliest start for this height.
                    stack.push((start, h));
                }
            }
            // Clean up remaining stack entries
            while let Some((idx, ht)) = stack.pop() {
                let width = n_usize - idx;
                let side = cmp::min(ht, width);
                if side > best {
                    best = side;
                }
            }
        }

        // Print the maximal side length for this test case.
        println!("{}", best);
    }
}
