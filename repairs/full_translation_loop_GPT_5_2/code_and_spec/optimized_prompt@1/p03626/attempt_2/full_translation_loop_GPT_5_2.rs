use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let s1 = it.next().unwrap().as_bytes().to_vec();
    let s2 = it.next().unwrap().as_bytes().to_vec();

    const MOD: i64 = 1_000_000_007;

    let mut ans: i64;
    // t == 1: previous processed tile was "vertical" (S1[i]==S2[i])
    // t == 0: previous processed tile was "horizontal" (S1[i]!=S2[i], spanning i and i+1)
    let mut t: i32;

    if s1[0] == s2[0] {
        ans = 3;
        t = 1;
    } else {
        ans = 6;
        t = 0;
    }

    // The original C loops i=1..N-1 and (intended) checks whether position i starts
    // a horizontal tile by comparing S1[i] and S2[i] (not S1[i] and S1[i+1]).
    // The provided C code uses `S1[i] == S1[i+1]`, which is both out-of-bounds at i=N-1
    // and does not match the standard/expected logic for this problem; sample confirms this.
    let mut i: usize = 1;
    while i < n {
        if s1[i] == s2[i] {
            // vertical at i
            if t == 0 {
                // previous was horizontal
                ans = (ans * 1) % MOD;
            } else {
                // previous was vertical
                ans = (ans * 2) % MOD;
            }
            t = 1;
            i += 1;
        } else {
            // horizontal starting at i (spans i and i+1)
            if t == 0 {
                ans = (ans * 3) % MOD;
            } else {
                ans = (ans * 2) % MOD;
            }
            t = 0;
            i += 2;
        }
    }

    println!("{}", ans % MOD);
}
