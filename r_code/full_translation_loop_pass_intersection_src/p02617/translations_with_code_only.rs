use std::io::{self, Read};

fn main() {
    // Read entire stdin (scanf-style whitespace separated integers)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // In C: int N; scanf("%d",&N);
    // If input is missing, C would have undefined behavior; here we default to 0.
    let n: i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

    let mut ans: i64 = 0;

    // for (i = 1; i <= N; i++) ans += (long long)(N - i + 2) * (N - i + 1) / 2;
    for i in 1..=n {
        let a = n - i + 2;
        let b = n - i + 1;
        ans += (a * b) / 2;
    }

    // for (i = 1; i <= N - 1; i++) { scanf("%d %d", &u, &w); ... }
    // If N <= 1, loop does not execute (same as C).
    for _ in 1..=n.saturating_sub(1) {
        let mut u: i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);
        let mut w: i64 = it.next().and_then(|s| s.parse::<i64>().ok()).unwrap_or(0);

        // C uses XOR swap if u > w; we can just swap.
        if u > w {
            std::mem::swap(&mut u, &mut w);
        }

        // ans -= (long long)u * (N - w + 1);
        ans -= u * (n - w + 1);
    }

    // printf("%lld\n", ans);
    print!("{ans}\n");
    // fflush(stdout); (print! without buffering guarantees isn't needed; Rust stdout flush is optional here)
}
