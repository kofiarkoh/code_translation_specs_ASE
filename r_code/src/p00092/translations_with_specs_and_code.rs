// translated_program.rs
//
// Translation of the provided C program to Rust according to the SPEC.
// - Reads multiple test cases from stdin. Each test case: integer N, followed by N row strings.
// - For each N>0 grid, computes the maximum side length k of an axis-aligned k×k subgrid
//   that contains no '*' characters, and prints k on its own line.
// - Terminates when a test case with N == 0 is read.
//
// Notes on fidelity:
// - Behavior follows the SPEC. Input parsing mirrors C's scanf("%d") and scanf("%s") semantics
//   by splitting on whitespace.
// - Rows shorter than N are unlikely per SPEC; to avoid panics we pad them with '*' (safe fallback).
//   The original C code would have undefined behavior in that case; SPEC marks such inputs as unspecified.
// - All computations are done with safe Rust, no unsafe blocks are used.

use std::io::{self, Read};

fn is_ok(map_ruiseki: &Vec<Vec<usize>>, y: usize, target: i32, n: usize) -> bool {
    // If target == 0, C code treats this as immediately satisfiable (since comparisons >=0).
    // Preserve that behavior.
    if target <= 0 {
        return true;
    }
    let t = target as usize;
    let mut now_renzoku: usize = 0;
    for col in 0..n {
        if map_ruiseki[y][col] >= t {
            now_renzoku += 1;
        } else {
            now_renzoku = 0;
        }
        if now_renzoku >= t {
            return true;
        }
    }
    false
}

fn main() {
    // Read entire stdin and split by whitespace to emulate scanf("%d") / scanf("%s") semantics.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut toks = input.split_whitespace();
    loop {
        let n_str = match toks.next() {
            Some(s) => s,
            None => break, // EOF - stop processing
        };
        let n: usize = match n_str.parse() {
            Ok(v) => v,
            Err(_) => break, // malformed integer - stop (SPEC: behavior unspecified; we stop safely)
        };
        if n == 0 {
            break; // termination signal
        }

        // Read N row strings. Use first N characters of each token. If a token is shorter than N,
        // pad with '*' to avoid panics (SPEC marks such cases as unspecified; this is a safe fallback).
        let mut map: Vec<Vec<u8>> = Vec::with_capacity(n);
        for _ in 0..n {
            let s = match toks.next() {
                Some(sv) => sv,
                None => return, // unexpected EOF: SPEC considers this unspecified; exit early
            };
            let bytes = s.as_bytes();
            if bytes.len() < n {
                let mut row = vec![b'*'; n];
                for i in 0..bytes.len() {
                    row[i] = bytes[i];
                }
                map.push(row);
            } else {
                map.push(bytes[..n].to_vec());
            }
        }

        // Compute vertical cumulative free-cell counts (map_ruiseki),
        // where map_ruiseki[row][col] = count of consecutive non-'*' cells ending at (row,col).
        let mut map_ruiseki = vec![vec![0usize; n]; n];
        for col in 0..n {
            let mut now: usize = 0;
            for row in 0..n {
                if map[row][col] == b'*' {
                    now = 0;
                } else {
                    now += 1;
                }
                map_ruiseki[row][col] = now;
            }
        }

        // For each possible bottom row y, binary search the largest square size with bottom at y.
        // Keep a running global result to narrow the search (as in the C code).
        let mut result: usize = 0;
        for y in 0..n {
            // Use signed ints for binary search bounds to match C int behavior and avoid underflow issues.
            let mut left: i32 = result as i32;
            let mut right: i32 = n as i32;
            while left <= right {
                let mid = (left + right) / 2;
                if is_ok(&map_ruiseki, y, mid, n) {
                    left = mid + 1;
                } else {
                    right = mid - 1;
                }
            }
            let candidate = if left > 0 { (left - 1) as usize } else { 0usize };
            if candidate > result {
                result = candidate;
            }
        }

        println!("{}", result);
    }
}
