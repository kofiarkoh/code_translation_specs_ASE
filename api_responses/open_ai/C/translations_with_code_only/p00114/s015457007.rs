use std::io::{self, Read};

/// Compute gcd using the same algorithm as the C source:
/// do { t = a; a = b % a; b = t; } while (a);
/// We guard the a == 0 case to avoid a panic (C would have undefined behavior
/// on a == 0 due to division by zero). For the valid inputs this matches the C behavior.
fn gcd(mut a: i64, mut b: i64) -> i64 {
    if a == 0 {
        return b;
    }
    loop {
        let t = a;
        a = b % a;
        b = t;
        if a == 0 {
            break;
        }
    }
    b
}

/// Compute lcm as a * b / gcd(a, b).
/// Use i128 for the intermediate multiplication to avoid overflow differences
/// between typical C implementations and Rust; this follows the mathematical
/// intent of the original code.
fn lcm(a: i64, b: i64) -> i64 {
    let g = gcd(a, b);
    let res = (a as i128 / g as i128) * (b as i128);
    res as i64
}

fn main() {
    // Read entire stdin and parse whitespace-separated i64 values,
    // mirroring scanf "%lld" behaviour.
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).expect("read stdin");
    let mut it = s.split_whitespace().map(|tok| tok.parse::<i64>()).peekable();

    loop {
        // Attempt to read six i64 values; if fewer remain, exit (similar to EOF).
        // This mirrors the C program which repeatedly scanf six values per iteration.
        let a1 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let m1 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let a2 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let m2 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let a3 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };
        let m3 = match it.next() {
            Some(Ok(v)) => v,
            _ => break,
        };

        // Equivalent to: if (!(a1 | m1 | a2 | m2 | a3 | m3)) break;
        if (a1 | m1 | a2 | m2 | a3 | m3) == 0 {
            break;
        }

        // Compute cycle lengths using the same loop semantics as the C code:
        // c1 = 1; for (r1 = a1; r1 != 1; r1 = r1 * a1 % m1) ++c1;
        // We perform the multiplication in i128 and then take remainder to reduce
        // chances of overflow differences; this matches the mathematical intent.
        let mut c1: i64 = 1;
        let mut r1: i64 = a1;
        while r1 != 1 {
            // Emulate r1 = (r1 * a1) % m1 with i128 intermediates.
            r1 = ((r1 as i128 * a1 as i128) % m1 as i128) as i64;
            c1 += 1;
        }

        let mut c2: i64 = 1;
        let mut r2: i64 = a2;
        while r2 != 1 {
            r2 = ((r2 as i128 * a2 as i128) % m2 as i128) as i64;
            c2 += 1;
        }

        let mut c3: i64 = 1;
        let mut r3: i64 = a3;
        while r3 != 1 {
            r3 = ((r3 as i128 * a3 as i128) % m3 as i128) as i64;
            c3 += 1;
        }

        let ans = lcm(lcm(c1, c2), c3);
        println!("{}", ans);
    }
}
