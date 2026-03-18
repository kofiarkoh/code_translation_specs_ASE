use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    let mut ans: i64 = 0;

    // for (i = 1; i <= N; i++) ans += (N - i + 2) * (N - i + 1) / 2;
    for i in 1..=n {
        let a = n - i + 2;
        let b = n - i + 1;
        ans += a * b / 2;
    }

    // for (i = 1; i <= N - 1; i++) { read u,w; ensure u<=w; ans -= u*(N-w+1); }
    for _ in 1..=n - 1 {
        let mut u: i64 = it.next().unwrap().parse().unwrap();
        let mut w: i64 = it.next().unwrap().parse().unwrap();
        if u > w {
            std::mem::swap(&mut u, &mut w);
        }
        ans -= u * (n - w + 1);
    }

    print!("{ans}\n");
}
