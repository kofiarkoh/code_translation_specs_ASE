// spiral_pattern.rs
// Translated from the provided C code (Aizu Vol-1 0141: Spiral Pattern).
// Preserves the original semantics and I/O behavior.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and iterate over lines similar to C's gets behavior.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut lines = input.lines();

    // Helper to mimic atoi behavior on a line: parse integer, or 0 on empty/invalid.
    let mut parse_next = || -> i32 {
        match lines.next() {
            Some(s) => s.trim().parse::<i32>().unwrap_or(0),
            None => 0,
        }
    };

    let cn = parse_next();
    for ci in 0..cn {
        let n = parse_next();

        // Create a (n+4) x (n+4) array filled with spaces, as in the C code.
        let size = (n + 4) as usize;
        let mut a = vec![vec![b' '; size]; size];

        // Define constants for directions to match C code.
        const U: i32 = 1;
        const R: i32 = 2;
        const D: i32 = 3;
        const L: i32 = 4;

        // Set the border rows (r = 0 and r = n+3) to '#'
        let step = (n + 3) as usize;
        let mut r_idx = 0usize;
        while r_idx < size {
            for c in 0..size {
                a[r_idx][c] = b'#';
            }
            // increment by n+3 as in C: r += n+3
            r_idx = r_idx.saturating_add(step);
            // prevent infinite loop if step == 0, but step == 0 only if n == -3 (impossible here)
        }

        // Set the border columns (c = 0 and c = n+3) to '#'
        let mut c_idx = 0usize;
        while c_idx < size {
            for r in 0..size {
                a[r][c_idx] = b'#';
            }
            c_idx = c_idx.saturating_add(step);
        }

        // Starting position and initial mark
        // Use signed ints for r,c to match C's arithmetic (pre/post increments).
        let mut r = (n + 1) as isize;
        let mut c = 2isize;
        let mut d = U;
        let mut stop = 0i32;

        // mark initial position
        a[r as usize][c as usize] = b'#';

        // Main walking loop: note separate ifs (not else-if) to preserve C behavior
        while stop < 4 {
            if d == U {
                // if (a[r-2][c] == '#' || a[r-1][c] == '#' || a[r-1][c+1] == '#') d = R, stop++;
                if a[(r - 2) as usize][c as usize] == b'#'
                    || a[(r - 1) as usize][c as usize] == b'#'
                    || a[(r - 1) as usize][(c + 1) as usize] == b'#'
                {
                    d = R;
                    stop += 1;
                } else {
                    r -= 1; // --r
                    a[r as usize][c as usize] = b'#';
                    stop = 0;
                }
            }
            if d == R {
                if a[r as usize][(c + 2) as usize] == b'#'
                    || a[r as usize][(c + 1) as usize] == b'#'
                    || a[(r + 1) as usize][(c + 1) as usize] == b'#'
                {
                    d = D;
                    stop += 1;
                } else {
                    c += 1; // ++c
                    a[r as usize][c as usize] = b'#';
                    stop = 0;
                }
            }
            if d == D {
                if a[(r + 2) as usize][c as usize] == b'#'
                    || a[(r + 1) as usize][c as usize] == b'#'
                    || a[(r + 1) as usize][(c - 1) as usize] == b'#'
                {
                    d = L;
                    stop += 1;
                } else {
                    r += 1; // ++r
                    a[r as usize][c as usize] = b'#';
                    stop = 0;
                }
            }
            if d == L {
                if a[r as usize][(c - 2) as usize] == b'#'
                    || a[r as usize][(c - 1) as usize] == b'#'
                    || a[(r - 1) as usize][(c - 1) as usize] == b'#'
                {
                    d = U;
                    stop += 1;
                } else {
                    c -= 1; // --c
                    a[r as usize][c as usize] = b'#';
                    stop = 0;
                }
            }
        }

        // Print a blank line between cases, like the C code does (putchar('\n') if ci > 0).
        if ci > 0 {
            println!();
        }

        // For r = 2; r < n+2; r++: print a[r][2 .. n+2]
        for rr in 2..(n + 2) {
            let rr_us = rr as usize;
            let start = 2usize;
            let end = (n + 2) as usize; // exclusive end
            // Convert the slice of bytes to a String to print.
            let row_slice = &a[rr_us][start..end];
            // Safe because contents are ASCII space or '#'
            let line = String::from_utf8_lossy(row_slice);
            println!("{}", line);
        }
    }
}
