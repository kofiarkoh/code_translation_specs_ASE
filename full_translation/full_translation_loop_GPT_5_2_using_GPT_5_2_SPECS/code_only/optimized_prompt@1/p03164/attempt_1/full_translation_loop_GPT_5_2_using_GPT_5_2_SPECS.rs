use std::io::{self, Read};

const MEW: usize = 112345;

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf("%d")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(x) => x,
        None => std::process::exit(1),
    };
    let w_limit: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
        Some(x) => x,
        None => std::process::exit(1),
    };

    // Arrays sized as in C (128). If N > 128, C would write out of bounds (UB).
    // We avoid UB in Rust by only storing the first 128 items; behavior for N>128
    // is not well-defined in C anyway.
    let mut w = [0i32; 128];
    let mut v = [0i32; 128];

    let n_usize = if n < 0 { 0 } else { n as usize };
    for i in 0..n_usize {
        let wi: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(x) => x,
            None => std::process::exit(1),
        };
        let vi: i32 = match it.next().and_then(|s| s.parse::<i32>().ok()) {
            Some(x) => x,
            None => std::process::exit(1),
        };
        if i < 128 {
            w[i] = wi;
            v[i] = vi;
        }
    }

    // DP array: initialize to W+1, then 0 at index 0.
    let mut ayaneru = vec![w_limit.wrapping_add(1); MEW];
    ayaneru[0] = 0;

    // 0/1 knapsack by value: minimize weight for each total value.
    // Use wrapping arithmetic to emulate C signed overflow behavior as closely as possible.
    for i in 0..n_usize.min(128) {
        let wi = w[i];
        let vi = v[i];

        // In C, loop is: for (j = MEW-1; j >= v[i]; j--)
        // If vi is negative, the condition is always true until j underflows (UB).
        // We conservatively only perform the loop when 0 <= vi < MEW.
        if vi < 0 || (vi as usize) >= MEW {
            continue;
        }
        let vi_u = vi as usize;

        for j in (vi_u..MEW).rev() {
            let prev = ayaneru[j - vi_u].wrapping_add(wi);
            if prev < ayaneru[j] {
                ayaneru[j] = prev;
            }
        }
    }

    // Find maximum value achievable within weight limit.
    for i in (0..MEW).rev() {
        if ayaneru[i] <= w_limit {
            print!("{}\n", i);
            return;
        }
    }

    // Matches puts("error!") which prints "error!\n"
    print!("error!\n");
}
