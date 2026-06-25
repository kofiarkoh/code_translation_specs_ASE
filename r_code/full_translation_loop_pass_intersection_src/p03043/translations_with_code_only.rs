use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers like scanf("%d%d", &N, &K)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // If no input, C would leave N/K uninitialized; we just exit.
    };
    let mut k: i32 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut multi: i32 = 1;
    let mut i: i32 = n;

    // C uses a VLA: int denomi[N];
    // In Rust, use Vec with length N. If N <= 0, C VLA is invalid/UB; we avoid panics by exiting.
    if n <= 0 {
        // C behavior is undefined here; choose a benign behavior (no output) rather than panic.
        return;
    }
    let nn = n as usize;
    let mut denomi: Vec<i32> = vec![0; nn];

    let mut nume: f64 = 0.0; // long double -> use f64 (Rust has no stable long double)

    while i > 0 {
        let idx = (i - 1) as usize;
        denomi[idx] = 0;

        // while (K > multi*i) { multi *= 2; }
        // In C, signed overflow is UB; we emulate typical 2's-complement wraparound using wrapping ops.
        while k > multi.wrapping_mul(i) {
            multi = multi.wrapping_mul(2);
        }

        // denomi[i-1] += multi*N;
        denomi[idx] = denomi[idx].wrapping_add(multi.wrapping_mul(n));

        i -= 1;
    }

    for j in 0..nn {
        // C: nume += denomi[0] / denomi[j];
        // Integer division in C truncates toward zero; Rust matches that for i32.
        // Division by zero would be UB in C; in Rust it panics. We avoid panic by mimicking
        // a "do nothing" behavior if it occurs (still not well-defined, but prevents crash).
        let denom = denomi[j];
        if denom == 0 {
            // Undefined in C; choose to skip to avoid panic.
            continue;
        }
        nume += (denomi[0] / denom) as f64;
    }

    // printf("%.12Lf", nume/denomi[0]);
    // If denomi[0] == 0, C would have UB (division by zero). Avoid panic similarly.
    if denomi[0] == 0 {
        return;
    }
    let out = nume / (denomi[0] as f64);
    print!("{:.12}", out);
}
