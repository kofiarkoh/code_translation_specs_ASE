use std::io::{self, Read};

const MAX_N: usize = 50_000;

fn build_is_prime() -> [bool; MAX_N + 1] {
    let mut is_prime = [true; MAX_N + 1];
    if MAX_N >= 0 {
        is_prime[0] = false;
    }
    if MAX_N >= 1 {
        is_prime[1] = false;
    }

    // Sieve of Eratosthenes
    let mut p = 2usize;
    while p * p <= MAX_N {
        if is_prime[p] {
            let mut m = p * p;
            while m <= MAX_N {
                is_prime[m] = false;
                m += p;
            }
        }
        p += 1;
    }

    is_prime
}

fn main() {
    // Read entire stdin; if it can't be read, terminate with no output.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }

    let is_prime = build_is_prime();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        // If input cannot be read as an integer, terminate with no further output.
        let n_i32: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        if n_i32 == 0 {
            break;
        }

        // Behavior is undefined for negative or > MAX_N in the specification.
        // We choose the narrowest reasonable interpretation: assume precondition holds.
        let n = n_i32 as usize;

        let mut count: i32 = 0;
        let half = n / 2;
        for p in 2..=half {
            if is_prime[p] {
                let q = n - p;
                if q <= MAX_N && is_prime[q] {
                    count += 1;
                }
            }
        }

        out.push_str(&count.to_string());
        out.push('\n');
    }

    print!("{out}");
}
