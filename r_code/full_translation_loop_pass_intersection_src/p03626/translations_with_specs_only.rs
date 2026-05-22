use std::io::{self, Read};

const MOD: i64 = 1_000_000_007;

fn main() {
    // Read entire stdin, split by whitespace (matches "whitespace-delimited token" behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s1 = it.next().unwrap().as_bytes().to_vec();
    let s2 = it.next().unwrap().as_bytes().to_vec();

    // Preconditions are assumed true; no validation is performed.
    // If violated, behavior is unspecified by the spec; we keep the code simple.

    let mut ans: i64;
    let mut t: i32; // 1 = vertical, 0 = horizontal

    if s1[0] == s2[0] {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    // Loop exactly as specified: i from 1 to N-1 inclusive, with two independent checks in order.
    for i in 1..n {
        // (a) Vertical-at-i check
        if s1[i] == s2[i] {
            if t == 1 {
                ans = (ans * 2) % MOD;
            }
            t = 1;
        }

        // (b) Horizontal-starting-at-i check
        // Note: This intentionally uses s1[i] == s1[i+1] and does not skip indices.
        // Under the tiling-validity assumptions, this matches the intended recurrence.
        if i + 1 < n && s1[i] == s1[i + 1] {
            if t == 1 {
                ans = (ans * 2) % MOD;
            } else {
                ans = (ans * 3) % MOD;
            }
            t = 0;
        }
    }

    println!("{}", ans % MOD);
}
