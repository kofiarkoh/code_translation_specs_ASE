use std::io::{self, Read};

fn main() {
    const MOD: i64 = 1_000_000_007;

    // Read all input as whitespace-delimited tokens (matches scanf("%d %s %s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s1 = it.next().unwrap().as_bytes().to_vec();
    let s2 = it.next().unwrap().as_bytes().to_vec();

    // Per spec: inputs are well-formed; no validation performed.
    // Note: The original C code may read S1[i+1] when i == N-1 (out of bounds).
    // Under the spec's tiling-validity assumptions, that access is not needed for
    // correctness; in Rust we avoid out-of-range access by guarding i+1 < N.
    let mut ans: i64;
    let mut t: i32;

    if s1[0] == s2[0] {
        ans = 3;
        t = 1; // previous orientation: vertical
    } else {
        ans = 6;
        t = 0; // previous orientation: horizontal
    }

    for i in 1..n {
        // (a) Vertical-at-i check
        if s1[i] == s2[i] {
            if t == 1 {
                ans = (ans * 2) % MOD;
            }
            t = 1;
        }

        // (b) Horizontal-starting-at-i check, driven by adjacency in S1
        if i + 1 < n && s1[i] == s1[i + 1] {
            if t == 1 {
                ans = (ans * 2) % MOD;
            } else {
                ans = (ans * 3) % MOD;
            }
            t = 0;
        }
    }

    println!("{ans}");
}
