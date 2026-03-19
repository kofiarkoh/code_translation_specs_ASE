use std::io::{self, Read};

const MAX: usize = 50001;

fn make_prime_table() -> Vec<bool> {
    // In the C code: table[k] == 0 means "prime" (for k>=2), 1 means "composite".
    // We'll store "is_composite" with the same meaning for direct semantic translation.
    let mut is_composite = vec![false; MAX];

    for i in 2..MAX {
        if !is_composite[i] {
            let mut j = 2usize;
            while i * j < MAX {
                is_composite[i * j] = true;
                j += 1;
            }
        }
    }
    is_composite
}

fn main() {
    let is_composite = make_prime_table();

    let mut input = String::new();
    io::stdin().read_to_string(&mut input).ok();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let n: usize = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // stop processing on parse failure
        };

        if n == 0 {
            break;
        }

        // Spec guarantees behavior only for 0..=50000; we follow the spec and do not add checks.
        let mut cnt = 0i32;
        for i in 2..=n / 2 {
            if !is_composite[i] && !is_composite[n - i] {
                cnt += 1;
            }
        }
        out.push_str(&format!("{cnt}\n"));
    }

    print!("{out}");
}
