use std::io::{self, Read};

fn main() {
    // Read entire stdin as whitespace-separated tokens, like scanf with "%d"
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut iter = s.split_whitespace();

    // Helper to parse next token as i32. If missing or invalid, return None.
    let mut next_i32 = || -> Option<i32> {
        iter.next()?.parse::<i32>().ok()
    };

    // Loop mirrors: while(1){ scanf three ints; if any zero break; scanf n; ... }
    loop {
        // Read t[0], t[1], t[2]; if any missing, stop.
        let mut t0 = match next_i32() {
            Some(v) => v,
            None => break,
        };
        let mut t1 = match next_i32() {
            Some(v) => v,
            None => break,
        };
        let mut t2 = match next_i32() {
            Some(v) => v,
            None => break,
        };

        // If any is 0, exit (like the C code's break)
        if t0 == 0 || t1 == 0 || t2 == 0 {
            break;
        }

        // Read n
        let n = match next_i32() {
            Some(v) => v,
            None => break,
        };

        // Replicate exact C conditional semantics and evaluation order.
        if t0 >= t2 {
            t0 = t2;
            t2 = -1;
        }
        // Note: evaluation order (left then right) and short-circuiting of && in Rust matches C here.
        if t1 >= t2 && t2 != -1 {
            t1 = t2;
        }

        // Use wrapping arithmetic to emulate typical 32-bit two's-complement behavior
        // and avoid potential panics in Rust debug builds on overflow.
        let hakari = t0.wrapping_mul(t0).wrapping_add(t1.wrapping_mul(t1));

        // In C, if n <= 0 the for loop does 0 iterations. Preserve that behavior.
        if n > 0 {
            // n is i32; convert to usize for loop bound (fits on typical platforms).
            let count = n as usize;
            for _ in 0..count {
                let mut data = match next_i32() {
                    Some(v) => v,
                    None => return, // input ended unexpectedly; stop like EOF
                };
                // data = data * 2; data = data * data; with wrapping behavior
                data = data.wrapping_mul(2);
                data = data.wrapping_mul(data);

                if hakari < data {
                    println!("OK");
                } else {
                    println!("NA");
                }
            }
        }
        // then loop repeats to read next triple
    }
}
