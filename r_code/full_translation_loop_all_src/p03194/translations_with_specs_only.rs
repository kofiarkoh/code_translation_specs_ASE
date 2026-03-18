use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let a: i64 = it.next().unwrap().parse().unwrap(); // first input (non-negative)
    let b: i64 = it.next().unwrap().parse().unwrap(); // second input (positive)

    // Based on observed behavior (e.g., 3 216 -> 6):
    // Interpret as: compute the largest integer x such that x^a divides b.
    // This equals: for b = Π p^{e_p}, x = Π p^{floor(e_p / a)}.
    //
    // If b has no prime factors (b <= 1) or all floor(e_p/a) are 0, output 0.
    // Overflow behavior is unspecified; use wrapping i64 arithmetic to emulate typical C long.
    if b <= 1 {
        println!("0");
        return;
    }

    let mut x = b as u64;
    let a_u = a as u64; // precondition: a >= 0; if a==0 behavior undefined by spec.

    let mut result: i64 = 1;
    let mut used_any = false;

    // Factor 2
    if x % 2 == 0 {
        let mut e: u64 = 0;
        while x % 2 == 0 {
            x /= 2;
            e = e.wrapping_add(1);
        }
        let k = e / a_u;
        if k > 0 {
            used_any = true;
            let mut pow: i64 = 1;
            for _ in 0..k {
                pow = pow.wrapping_mul(2);
            }
            result = result.wrapping_mul(pow);
        }
    }

    // Factor odd primes
    let mut p: u64 = 3;
    while p <= x / p {
        if x % p == 0 {
            let mut e: u64 = 0;
            while x % p == 0 {
                x /= p;
                e = e.wrapping_add(1);
            }
            let k = e / a_u;
            if k > 0 {
                used_any = true;
                let base = p as i64;
                let mut pow: i64 = 1;
                for _ in 0..k {
                    pow = pow.wrapping_mul(base);
                }
                result = result.wrapping_mul(pow);
            }
        }
        p = p.wrapping_add(2);
    }

    // Remaining prime factor
    if x > 1 {
        let e: u64 = 1;
        let k = e / a_u;
        if k > 0 {
            used_any = true;
            // k can only be 1 when a_u == 1; otherwise 0.
            let base = x as i64;
            let mut pow: i64 = 1;
            for _ in 0..k {
                pow = pow.wrapping_mul(base);
            }
            result = result.wrapping_mul(pow);
        }
    }

    if !used_any {
        println!("0");
    } else {
        println!("{}", result);
    }
}
