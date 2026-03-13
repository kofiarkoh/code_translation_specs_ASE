// Translated from the provided C program.
// Preserves the C program's semantics exactly as implemented there.

use std::io::{self, Read};

fn main() {
    // Read entire stdin and iterate over whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");
    let mut toks = input.split_whitespace();

    // Helper to parse next token as i32
    let mut parse_next_i32 = |toks: &mut std::str::SplitWhitespace| -> Option<i32> {
        toks.next().and_then(|s| s.parse::<i32>().ok())
    };

    loop {
        // Attempt to read h and w. If either is missing, stop (like scanf failure).
        let h = match parse_next_i32(&mut toks) {
            Some(v) => v,
            None => break,
        };
        let w = match parse_next_i32(&mut toks) {
            Some(v) => v,
            None => break,
        };

        // The original C loop condition is: while (scanf("%d %d",&h,&w) * h * w)
        // That exits when h*w == 0 (or when scanf fails). We mirror that: stop if h==0 or w==0.
        if h == 0 || w == 0 {
            break;
        }

        // Convert to usize for indexing
        let h_us = h as usize;
        let w_us = w as usize;

        // Build mat[h][w] where mat[i][j] is the length of the run of '.' ending at column j in row i.
        let mut mat: Vec<Vec<i32>> = vec![vec![0; w_us]; h_us];

        for i in 0..h_us {
            // The original C code used scanf("\n") then w times scanf("%c",&c),
            // effectively reading the next non-whitespace sequence of characters for the row.
            // Here we take the next whitespace-separated token as the row.
            let row_tok = match toks.next() {
                Some(s) => s,
                None => {
                    // If input ends unexpectedly, behave like scanf failing: stop processing.
                    return;
                }
            };

            // Ensure we have at least w characters; in typical inputs this is true.
            // We'll index bytes; the input is expected to be ASCII-like ('.' / other single-byte chars).
            let row_bytes = row_tok.as_bytes();
            let mut run = 0i32;
            for j in 0..w_us {
                // If the token is shorter than w, treat missing as non-'.' (like whatever character not '.')
                let is_dot = if j < row_bytes.len() {
                    row_bytes[j] == b'.'
                } else {
                    false
                };
                if is_dot {
                    run += 1;
                } else {
                    run = 0;
                }
                mat[i][j] = run;
            }
        }

        // Replicate the exact rectangle-area computation from the C code.
        let mut ans: i32 = 0;
        for j in (0..w_us).rev() {
            for k in (0..h_us).rev() {
                let mut min = mat[k][j];
                let mut i = 1usize;
                while k + i < h_us {
                    if min > mat[k + i][j] {
                        min = mat[k + i][j];
                    }
                    if min == 0 {
                        break;
                    }
                    // area = min * (i+1)
                    let area = min * ((i as i32) + 1);
                    if area > ans {
                        ans = area;
                    }
                    i += 1;
                }
            }
        }

        // Print result for this case, preserving newline formatting.
        println!("{}", ans);
    }
}
