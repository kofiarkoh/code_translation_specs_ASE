use std::io::{self, Read};

const MAX: usize = 1000;
const MOD: i64 = 1_000_000_000;

fn main() {
    // Read entire stdin; if reading fails, exit with a non-zero error code.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        std::process::exit(1);
    }

    let mut it = input.split_whitespace();
    let n: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };
    let k: usize = match it.next().and_then(|s| s.parse::<usize>().ok()) {
        Some(v) => v,
        None => std::process::exit(1),
    };

    // The observed sample (n=5,k=2 => 3) matches the count of partitions of n into k
    // positive parts where boxes are distinguishable up to reversal only, i.e.:
    // count of integer solutions a1+...+ak=n with a1>=a2>=...>=ak>=1.
    //
    // This is the classic "partition of n into exactly k parts" function p(n,k).
    // DP recurrence:
    //   p(n,k) = p(n-1,k-1) + p(n-k,k)
    // with base:
    //   p(0,0)=1
    //   p(n,0)=0 for n>0
    //   p(n,k)=0 for n<0 or k<0
    //
    // We compute a fixed (MAX+1)x(MAX+1) table in O(MAX^2), which is polynomial and
    // within the stated constraints.
    let mut p = vec![vec![0i64; MAX + 1]; MAX + 1];
    p[0][0] = 1;

    for nn in 1..=MAX {
        p[nn][0] = 0;
    }
    for kk in 1..=MAX {
        p[0][kk] = 0;
    }

    for nn in 1..=MAX {
        for kk in 1..=MAX {
            let a = p[nn - 1][kk - 1];
            let b = if nn >= kk { p[nn - kk][kk] } else { 0 };
            p[nn][kk] = (a + b) % MOD;
        }
    }

    // Behavior is undefined for invalid input values; we do not add extra handling.
    println!("{}", p[n][k] % MOD);
}
