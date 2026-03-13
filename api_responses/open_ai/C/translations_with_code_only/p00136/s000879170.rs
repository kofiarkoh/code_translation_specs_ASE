// translated from the provided C program
// Preserves the I/O formatting and comparison semantics (f64 comparisons behave like C double,
// i.e., comparisons with NaN are false).
use std::io::{self, Read};

fn main() {
    // Read all input tokens (whitespace-separated), similar to repeated scanf calls.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut toks = input.split_whitespace();

    // Parse n. If parsing fails or is missing, behave conservatively by treating n = 0.
    // (The original C program would invoke undefined behaviour if scanf failed; we avoid UB here.)
    let n: i32 = match toks.next().and_then(|t| t.parse::<i32>().ok()) {
        Some(v) => v,
        None => 0,
    };

    // cnt[1..6] are used in the C code; we keep a 7-element vector and ignore index 0.
    let mut cnt = vec![0i32; 7];

    // Loop exactly n times if n > 0 (C loop is: for(i=1;i<=n;i++))
    if n > 0 {
        for _ in 1..=n {
            // If there is no next token or parsing fails, stop processing further entries.
            let val_opt = toks.next().and_then(|t| t.parse::<f64>().ok());
            let val = match val_opt {
                Some(v) => v,
                None => break,
            };

            // Follow the same chain of comparisons as the C code.
            if val < 165.0 {
                cnt[1] = cnt[1].wrapping_add(1);
            } else if val < 170.0 {
                cnt[2] = cnt[2].wrapping_add(1);
            } else if val < 175.0 {
                cnt[3] = cnt[3].wrapping_add(1);
            } else if val < 180.0 {
                cnt[4] = cnt[4].wrapping_add(1);
            } else if val < 185.0 {
                cnt[5] = cnt[5].wrapping_add(1);
            } else if val >= 185.0 {
                // This branch mirrors the last `else if` in the C code.
                cnt[6] = cnt[6].wrapping_add(1);
            }
            // Note: comparisons with NaN yield false in Rust just like in C, so no category is incremented.
        }
    }

    // Output lines "i:" followed by cnt[i] asterisks, exactly as the C program.
    for i in 1..=6 {
        print!("{}:", i);
        if cnt[i] > 0 {
            // cnt[i] is i32; convert to usize only when positive.
            let stars = "*".repeat(cnt[i] as usize);
            print!("{}", stars);
        }
        println!();
    }
}
