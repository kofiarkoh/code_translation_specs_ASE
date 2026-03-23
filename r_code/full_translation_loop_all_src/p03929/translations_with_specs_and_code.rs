use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse two i64 values (behavior undefined if missing/invalid, per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();
    let mut n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    // Match C semantics: signed 64-bit arithmetic without overflow checks.
    // In Rust, use wrapping ops to avoid panics in debug builds and to mimic unchecked behavior.
    n = n.wrapping_mul(7);

    let mut ans: i64 = 0;

    // Remainder block: i = 1..= (n % 693)
    // Note: In C, if n is negative, n % 693 is negative and the loop does not execute.
    let rem = n % 693;
    for i in 1..=rem {
        if i % 7 > 1 {
            if (9i64.wrapping_mul(i)) % 11 == k {
                ans = ans.wrapping_add(1);
            }
        }
    }

    // Full blocks count over 1..=693
    let mut count: i64 = 0;
    for i in 1..=693i64 {
        if i % 7 > 1 {
            if (9i64.wrapping_mul(i)) % 11 == k {
                count = count.wrapping_add(1);
            }
        }
    }
    ans = ans.wrapping_add(count.wrapping_mul(n / 693));

    // Subtract window 1..=7
    for i in 1..=7i64 {
        if i % 7 > 1 {
            if (9i64.wrapping_mul(i)) % 11 == k {
                ans = ans.wrapping_sub(1);
            }
        }
    }

    // Subtract window (n-6)..=n
    // In C, if n-6 > n (impossible for integers), loop wouldn't run; otherwise it runs inclusive.
    let start = n.wrapping_sub(6);
    for i in start..=n {
        if i % 7 > 1 {
            if (i.wrapping_mul(9)) % 11 == k {
                ans = ans.wrapping_sub(1);
            }
        }
    }

    print!("{ans}\n");
}
