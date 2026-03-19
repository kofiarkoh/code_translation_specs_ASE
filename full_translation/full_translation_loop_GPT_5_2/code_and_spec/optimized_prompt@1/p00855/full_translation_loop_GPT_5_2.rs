use std::io::{self, Read};

fn is_prime(n: i32) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    let mut d = 3i32;
    while (d as i64) * (d as i64) <= n as i64 {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let l0: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => continue, // unspecified on invalid input
        };

        // The original C loop terminates when scanf fails OR when l == 0 (since u=l and `for(; ... , u=l; )` tests u).
        // Therefore, an input value of 0 is a sentinel and produces no output.
        if l0 == 0 {
            break;
        }

        let mut l = l0;
        while !is_prime(l) {
            l -= 1;
        }

        let mut u = l0;
        while !is_prime(u) {
            u += 1;
        }

        out.push_str(&(u - l).to_string());
        out.push('\n');
    }

    print!("{out}");
}
