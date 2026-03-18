use std::io::{self, Read};

fn main() {
    // Read entire stdin (C version effectively reads one line; spec says read two integers).
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // Spec: if input reading fails, return an error code.
        std::process::exit(1);
    }

    let mut it = input.split_whitespace();
    let n_opt = it.next().and_then(|s| s.parse::<u32>().ok());
    let k_opt = it.next().and_then(|s| s.parse::<u32>().ok());

    let (n, k) = match (n_opt, k_opt) {
        (Some(n), Some(k)) => (n, k),
        _ => {
            // Spec: if input reading fails, return an error code.
            std::process::exit(1);
        }
    };

    // Spec preconditions: n,k non-negative and k <= n. If invalid, produce no output.
    if k > n {
        return;
    }

    // C code uses (1<<n) with int; behavior is undefined/overflow for large n.
    // To preserve intended semantics safely, we only iterate if 2^n fits in u64.
    if n >= 64 {
        // No safe way to enumerate all patterns; also would be infeasible.
        // Spec allows "limited by maximum size of input buffer"; we choose to produce no output.
        return;
    }

    let limit: u64 = 1u64 << n;
    for pat in 0..limit {
        if pat.count_ones() != k {
            continue;
        }
        // C prints pattern as decimal integer.
        print!("{}:", pat);
        for i in 0..n {
            if (pat & (1u64 << i)) == 0 {
                continue;
            }
            print!(" {}", i);
        }
        println!();
    }
}
