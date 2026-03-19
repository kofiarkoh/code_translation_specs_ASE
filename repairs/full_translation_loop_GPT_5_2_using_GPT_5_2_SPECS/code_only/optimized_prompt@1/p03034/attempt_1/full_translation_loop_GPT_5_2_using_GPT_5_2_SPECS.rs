use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as whitespace-separated tokens (matches scanf behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return, // No input: C would have undefined behavior; we just exit.
    };

    // C uses a fixed global array a[100010]; we only need first n elements.
    let mut a: Vec<i64> = vec![0; n.max(0) as usize];
    for i in 0..(n.max(0) as usize) {
        if let Some(t) = it.next() {
            a[i] = t.parse::<i64>().unwrap();
        } else {
            // Missing input: C would read uninitialized/undefined; we keep 0.
            break;
        }
    }

    let mut ans: i64 = 0;

    // for (int d=1; d*2 < n-1; d++)
    // Use i64 for loop arithmetic to avoid overflow in d*2 and comparisons.
    let n_i64 = n as i64;
    let mut d: i64 = 1;
    while d * 2 < n_i64 - 1 {
        let mut s: i64 = 0;
        let mut g: i64 = n_i64 - 1;
        let mut crr: i64 = 0;

        loop {
            let ns = s + d;
            let ng = g - d;
            if ng <= d || ns == ng || ns == g {
                break;
            }
            s = ns;
            g = ng;

            // C signed overflow is UB; use wrapping to emulate typical two's-complement behavior.
            let add = a[s as usize].wrapping_add(a[g as usize]);
            crr = crr.wrapping_add(add);

            if crr > ans {
                ans = crr;
            }
        }

        d += 1;
    }

    // Match C: no trailing newline.
    print!("{}", ans);
}
