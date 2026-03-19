use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace-separated tokens)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In the C code, failing scanf would leave variables uninitialized (UB).
    // Here we assume input is well-formed as typical for such programs.
    let n: i32 = it.next().unwrap().parse().unwrap();

    let mut ans: i64 = 0;

    // for (i = 1; i <= N; i++) ans += (long long)(N - i + 2) * (N - i + 1) / 2;
    for i in 1..=n {
        let a = (n - i + 2) as i64;
        let b = (n - i + 1) as i64;
        ans += a * b / 2;
    }

    // for (i = 1; i <= N - 1; i++) { ... }
    for _i in 1..=n - 1 {
        let mut u: i32 = it.next().unwrap().parse().unwrap();
        let mut w: i32 = it.next().unwrap().parse().unwrap();

        // Preserve the XOR-swap behavior (works even if u == w).
        if u > w {
            u ^= w;
            w ^= u;
            u ^= w;
        }

        // ans -= (long long)u * (N - w + 1);
        ans -= (u as i64) * ((n - w + 1) as i64);
    }

    // Match C output formatting exactly: one integer and a trailing newline.
    print!("{ans}\n");
    // stdout is flushed on program exit; explicit flush not necessary for equivalence here.
}
