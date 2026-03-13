use std::io::{self, Read};

fn main() {
    // Fixed pattern c as specified.
    let c: [i32; 16] = [1, 2, 1, 2, 1, 4, 1, 4, 1, 2, 1, 2, 1, 4, 1, 4];

    // Read entire stdin, then parse whitespace-separated tokens as integers.
    let mut input = String::new();
    // If reading fails, exit (no output). This mirrors the C program terminating on read failure.
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let mut iter = input.split_whitespace();

    // For each full group of 8 integers, process and print one line.
    loop {
        // Attempt to read 8 integers; if fewer than 8 tokens or a parse fails, stop processing.
        let mut p: [i32; 8] = [0; 8];
        let mut ok = true;
        for j in 0..8 {
            match iter.next() {
                Some(tok) => match tok.parse::<i32>() {
                    Ok(v) => p[j] = v,
                    Err(_) => {
                        ok = false;
                        break;
                    }
                },
                None => {
                    ok = false;
                    break;
                }
            }
        }
        if !ok {
            break;
        }

        // Compute S(0) first, then evaluate other k in the explicit order
        // 1,7,3,5,8,6,2,4, updating only on strictly smaller sums.
        //
        // Note: The original C had uninitialized variables (UB). Here we follow the
        // specification's intended behavior: initialize current sum and min index = 0.
        let mut sum0: i64 = 0;
        for j in 0..8 {
            let diff = (p[j] as i64) - (c[j] as i64);
            if diff > 0 {
                sum0 += diff;
            }
        }
        let mut min_s = sum0;
        let mut min_p: usize = 0;

        // Evaluation order after 0:
        let order = [1usize, 7, 3, 5, 8, 6, 2, 4];
        for &k in &order {
            let mut cur_s: i64 = 0;
            for j in 0..8 {
                let diff = (p[j] as i64) - (c[k + j] as i64);
                if diff > 0 {
                    cur_s += diff;
                }
            }
            if cur_s < min_s {
                min_s = cur_s;
                min_p = k;
            }
        }

        // Build output line: c[min_p..min_p+8] separated by single spaces, then newline.
        let mut out = String::new();
        for i in min_p..(min_p + 8) {
            if !out.is_empty() {
                out.push(' ');
            }
            out.push_str(&c[i].to_string());
        }
        println!("{}", out);
    }
}
