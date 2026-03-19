use std::io::{self, Read};

fn main() {
    // Read entire stdin, then parse as whitespace-separated integers.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let mut it = input.split_whitespace();

    // Fixed maximum capacity per spec (and C code).
    let mut a = [0i64; 100];

    loop {
        let n: usize = match it.next() {
            Some(tok) => match tok.parse::<usize>() {
                Ok(v) => v,
                Err(_) => return, // invalid integer => terminate immediately
            },
            None => return, // input exhausted
        };

        if n == 0 {
            return; // terminating length; no output
        }

        // Per spec, n is 1..=100; if outside, behavior is undefined in C.
        // We avoid panics and just stop if it would exceed our fixed buffer.
        if n > 100 {
            return;
        }

        for i in 0..n {
            let val = match it.next() {
                Some(tok) => match tok.parse::<i64>() {
                    Ok(v) => v,
                    Err(_) => return, // invalid integer => terminate immediately
                },
                None => return, // missing required integer => terminate immediately
            };
            a[i] = val;
        }

        // The C code counts swaps performed by bubble sort; this equals inversion count.
        let mut swap_count: i64 = 0;
        for u in (1..n).rev() {
            for i in 0..u {
                if a[i] > a[i + 1] {
                    a.swap(i, i + 1);
                    swap_count += 1;
                }
            }
        }

        println!("{swap_count}");
    }
}
