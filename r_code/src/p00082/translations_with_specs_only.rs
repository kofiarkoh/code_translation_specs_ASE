// translate_spec.rs
//
// Translation of the provided specification into Rust.
// - Preserves the intended/assumed semantics (initializing accumulators).
// - Uses i32 to represent C int (common platform assumption).
// - Uses wrapping addition when accumulating S(k) to provide deterministic behavior
//   in the presence of overflow (the C source did not check overflow).
// - Evaluation order for k is: 0, then 1,7,3,5,8,6,2,4 (as specified).
//
// The program reads whitespace-separated integers from stdin, in groups of 8.
// It stops processing on EOF or on the first non-integer token (mimicking scanf
// returning != 8). For each full group of 8 integers, it computes S(k) for
// k=0..8 and prints the selected window c[k..k+7] as eight integers separated
// by single spaces and terminated by a newline.

use std::io::{self, Read};

fn main() {
    // Fixed pattern array c (length 16)
    const C: [i32; 16] = [1, 2, 1, 2, 1, 4, 1, 4, 1, 2, 1, 2, 1, 4, 1, 4];

    // Read entire stdin as bytes and split on ASCII whitespace to mimic scanf behavior.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit normally (as per spec: no structured exceptions).
        return;
    }

    let mut tokens = input.split_whitespace();

    // Helper to parse the next token as i32. If parsing fails, return None to stop processing.
    let mut next_i32 = || -> Option<i32> {
        match tokens.next() {
            Some(tok) => match tok.parse::<i32>() {
                Ok(v) => Some(v),
                Err(_) => None, // stop on parse failure (mimic scanf behavior)
            },
            None => None, // EOF
        }
    };

    // Process groups of 8 integers
    loop {
        // Attempt to read 8 integers; stop if any missing or non-parseable.
        let mut p = [0i32; 8];
        let mut ok = true;
        for i in 0..8 {
            match next_i32() {
                Some(v) => p[i] = v,
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break; // stop processing on incomplete group
        }

        // Evaluation order as specified: first k=0, then sequence [1,7,3,5,8,6,2,4]
        let eval_order: [usize; 9] = [0, 1, 7, 3, 5, 8, 6, 2, 4];

        // Compute S(0) first and initialize minS and minK accordingly.
        // curS and minP are explicitly initialized as per intended semantics.
        fn compute_s(k: usize, p: &[i32; 8], c: &[i32; 16]) -> i32 {
            // Sum of positive excesses with wrapping addition to emulate unchecked C int behavior.
            let mut sum: i32 = 0;
            for j in 0..8 {
                // c index k+j is in 0..15 by construction since k in 0..8
                let diff = p[j].wrapping_sub(c[k + j]);
                // max(0, diff)
                if diff > 0 {
                    sum = sum.wrapping_add(diff);
                }
            }
            sum
        }

        let mut min_k = eval_order[0];
        let mut min_s = compute_s(min_k, &p, &C);

        // Evaluate remaining ks in the specified order and update only on strictly smaller S.
        for &k in &eval_order[1..] {
            let s = compute_s(k, &p, &C);
            // Strictly smaller only (ties do not replace previous minimum).
            if s < min_s {
                min_s = s;
                min_k = k;
            }
        }

        // Print c[min_k .. min_k+7] separated by single spaces and a newline.
        // Ensure no trailing space before newline.
        let mut out_line = String::new();
        for j in 0..8 {
            if j > 0 {
                out_line.push(' ');
            }
            out_line.push_str(&C[min_k + j].to_string());
        }
        println!("{}", out_line);
    }
}
