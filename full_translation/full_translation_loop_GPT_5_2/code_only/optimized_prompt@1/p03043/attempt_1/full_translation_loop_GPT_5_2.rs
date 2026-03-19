use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two integers like scanf("%d%d", &N, &K)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n_opt = it.next();
    let k_opt = it.next();
    if n_opt.is_none() || k_opt.is_none() {
        // If scanf fails, C would leave N/K uninitialized (UB). We choose to do nothing.
        return;
    }

    // C `int` is typically 32-bit; emulate with i32 and wrapping where overflow could occur.
    let n: i32 = n_opt.unwrap().parse().unwrap();
    let mut k: i32 = k_opt.unwrap().parse().unwrap();

    let mut multi: i32 = 1;
    let mut i: i32 = n;

    // C VLA `int denomi[N];` is UB if N <= 0. We emulate by creating an empty vec for N<=0.
    let len = if n > 0 { n as usize } else { 0usize };
    let mut denomi: Vec<i32> = vec![0; len];

    let mut nume: f64 = 0.0; // use f64 to approximate C long double in a portable way

    while i > 0 {
        let idx = (i - 1) as usize;
        denomi[idx] = 0;

        // while (K > multi*i) { multi *= 2; }
        // In C, `multi*i` and `multi*=2` can overflow (UB). We use wrapping to emulate typical behavior.
        while k > multi.wrapping_mul(i) {
            multi = multi.wrapping_mul(2);
        }

        // denomi[i-1] += multi*N;
        denomi[idx] = denomi[idx].wrapping_add(multi.wrapping_mul(n));

        i -= 1;
    }

    // for (int j = 0; j < N; j++) { nume += denomi[0]/denomi[j]; }
    // In C, this is integer division, then promoted to long double.
    // Also, if denomi[j] == 0, C has UB (division by zero). We avoid panicking and mimic a "do nothing" fallback.
    if n > 0 {
        let d0 = denomi[0];
        for j in 0..(n as usize) {
            let dj = denomi[j];
            if dj == 0 {
                // Avoid Rust panic; C would be UB here.
                // We choose to skip contribution to keep program running.
                continue;
            }
            let q: i32 = d0 / dj; // truncates toward zero like C
            nume += q as f64;
        }

        // printf("%.12Lf", nume/denomi[0]);
        // If denomi[0] == 0, C would be UB (division by zero). Avoid panic.
        if d0 != 0 {
            let out = nume / (d0 as f64);
            print!("{:.12}", out);
        } else {
            // UB in C; print nothing.
        }
    } else {
        // N <= 0: C would have UB due to VLA and denomi[0] access; print nothing.
    }
}
