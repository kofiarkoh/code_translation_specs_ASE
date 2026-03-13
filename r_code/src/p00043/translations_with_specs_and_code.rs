// Transliteration of the described C program according to the provided SPEC.
// Note: The original C code's macros had inverted logic compared to the SPEC.
// The SPEC requires:
//  - result[0] != result[1]
//  - For each of the four triplets (positions 2..4,5..7,8..10,11..13) the triplet
//    must NOT be all-equal and must NOT be strictly descending-by-one.
// The C code's macros appeared to test the opposite; this Rust translation follows
// the SPEC (and intentionally differs from the buggy/inverted checks in the C code).

use std::io::{self, BufRead};

fn triplet_bad(a: u8, b: u8, c: u8) -> bool {
    // Return true if the triplet is one of the forbidden patterns:
    //  - all equal (a == b == c)
    //  - strictly descending by 1 (a == b + 1 && b == c + 1)
    (a == b && b == c) || (a == b + 1 && b == c + 1)
}

fn exists_permutation_with_constraints(p: &[u8; 14]) -> bool {
    // DFS over permutations of indices 0..13 selecting values p[i].
    // Enforce:
    //  - after placing positions 0 and 1: result[0] != result[1]
    //  - after filling positions 0..4: triplet at 2..4 not bad
    //  - after filling positions 0..7: triplet at 5..7 not bad
    //  - after filling positions 0..10: triplet at 8..10 not bad
    //  - after full fill: triplet at 11..13 not bad
    let mut used = [false; 14];
    let mut result = [0u8; 14];
    let mut solved = false;

    fn dfs(
        n: usize,
        p: &[u8; 14],
        used: &mut [bool; 14],
        result: &mut [u8; 14],
        solved: &mut bool,
    ) {
        if *solved {
            return;
        }
        // After placing indices 0 and 1 (i.e., next index n == 2),
        // enforce result[0] != result[1]
        if n == 2 {
            if result[0] == result[1] {
                return;
            }
        }
        // After placing up to index 4 (n == 5), check triplet at 2..4
        if n == 5 {
            if triplet_bad(result[2], result[3], result[4]) {
                return;
            }
        }
        // After placing up to index 7 (n == 8), check triplet at 5..7
        if n == 8 {
            if triplet_bad(result[5], result[6], result[7]) {
                return;
            }
        }
        // After placing up to index 10 (n == 11), check triplet at 8..10
        if n == 11 {
            if triplet_bad(result[8], result[9], result[10]) {
                return;
            }
        }

        if n < 14 {
            for i in 0..14 {
                if used[i] {
                    continue;
                }
                used[i] = true;
                result[n] = p[i];
                dfs(n + 1, p, used, result, solved);
                used[i] = false;
                if *solved {
                    break;
                }
            }
        } else {
            // n == 14, full permutation constructed; check final triplet 11..13
            if triplet_bad(result[11], result[12], result[13]) {
                return;
            }
            *solved = true;
        }
    }

    dfs(0, p, &mut used, &mut result, &mut solved);
    solved
}

fn main() {
    let stdin = io::stdin();
    for line_res in stdin.lock().lines() {
        match line_res {
            Ok(line) => {
                let token = line.trim();
                // SPEC expects exactly 13-digit input per line; behavior for other lengths is unspecified.
                // Here we only process tokens of length 13; others are skipped (no output).
                if token.len() != 13 {
                    // Skip lines that don't meet the precondition (unspecified behavior in SPEC).
                    continue;
                }
                // Build initial p[0..12] from token chars minus '0'
                // p[13] will be set for each candidate 1..9
                let mut p = [0u8; 14];
                let mut valid_input = true;
                for (i, ch) in token.chars().take(13).enumerate() {
                    if let Some(d) = ch.to_digit(10) {
                        p[i] = d as u8;
                    } else {
                        valid_input = false;
                        break;
                    }
                }
                if !valid_input {
                    // Input contains non-digit; unspecified per SPEC -> skip
                    continue;
                }

                let mut ans_buf: Vec<u8> = Vec::new();
                // For candidates 1..9 inclusive
                for cand in 1u8..=9u8 {
                    // dupCh: count occurrences among initial 13 positions only
                    let mut count = 0;
                    for i in 0..13 {
                        if p[i] == cand {
                            count += 1;
                        }
                    }
                    // Skip candidate silently if it already appears at least 4 times
                    if count >= 4 {
                        continue;
                    }
                    p[13] = cand;
                    if exists_permutation_with_constraints(&p) {
                        ans_buf.push(cand);
                    }
                }

                if ans_buf.is_empty() {
                    println!("0");
                } else {
                    // Print digits separated by single spaces
                    let mut out = String::new();
                    for (i, &d) in ans_buf.iter().enumerate() {
                        if i > 0 {
                            out.push(' ');
                        }
                        out.push((d + b'0') as char);
                    }
                    println!("{}", out);
                }
            }
            Err(_) => break,
        }
    }
}
