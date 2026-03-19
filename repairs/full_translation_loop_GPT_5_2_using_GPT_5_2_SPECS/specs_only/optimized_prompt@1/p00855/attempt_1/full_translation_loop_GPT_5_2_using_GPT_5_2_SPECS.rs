use std::io::{self, Read};

fn is_prime(n: i64) -> bool {
    if n < 2 {
        return false;
    }
    if n == 2 || n == 3 {
        return true;
    }
    if n % 2 == 0 {
        return false;
    }
    // Trial division up to floor(sqrt(n)) without risking i*i overflow.
    let mut d: i64 = 3;
    while d <= n / d {
        if n % d == 0 {
            return false;
        }
        d += 2;
    }
    true
}

fn prev_prime_leq(x: i64) -> i64 {
    // Preconditions guarantee existence and termination within range.
    let mut n = x;
    while !is_prime(n) {
        n -= 1;
    }
    n
}

fn next_prime_geq(x: i64) -> i64 {
    // Preconditions guarantee existence and termination within range.
    let mut n = x;
    while !is_prime(n) {
        n += 1;
    }
    n
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for tok in input.split_whitespace() {
        let x: i64 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break, // terminate on invalid input
        };

        let l = prev_prime_leq(x);
        let u = next_prime_geq(x);

        // Under the stated preconditions, u >= l and the result is non-negative.
        let diff = u - l;
        out.push_str(&diff.to_string());
        out.push('\n');
    }

    print!("{out}");
}
