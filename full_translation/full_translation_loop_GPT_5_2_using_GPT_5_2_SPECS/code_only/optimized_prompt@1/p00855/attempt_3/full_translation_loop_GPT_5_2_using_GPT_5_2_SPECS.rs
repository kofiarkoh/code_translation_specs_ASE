use std::io::{self, Read};

fn is_composite_like_c(n: i32) -> bool {
    // C's p(n) returns 0 for "prime-like" values and nonzero for composites.
    // It treats n <= 3 (and also n <= 1, negatives) as prime-like because the loop never runs.
    if n <= 3 {
        return false;
    }
    if n < 0 {
        return false;
    }
    if n % 2 == 0 {
        return true;
    }
    let mut i: i32 = 3;
    while (i as i64) * (i as i64) <= n as i64 {
        if n % i == 0 {
            return true;
        }
        i += 2;
    }
    false
}

fn prev_prime_like_c(mut l: i32) -> i32 {
    while is_composite_like_c(l) {
        l = l.wrapping_sub(1);
    }
    l
}

fn next_prime_like_c(mut u: i32) -> i32 {
    while is_composite_like_c(u) {
        u = u.wrapping_add(1);
    }
    u
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();
    while let Some(tok) = it.next() {
        let l0: i32 = match tok.parse() {
            Ok(v) => v,
            Err(_) => break,
        };

        // C loop condition: for (; scanf("%d",&l), u=l; )
        // Terminates when l == 0 (since u=l is the loop condition).
        if l0 == 0 {
            break;
        }

        let l = prev_prime_like_c(l0);
        let u = next_prime_like_c(l0);
        let diff = u.wrapping_sub(l);
        out.push_str(&format!("{diff}\n"));
    }

    print!("{out}");
}
