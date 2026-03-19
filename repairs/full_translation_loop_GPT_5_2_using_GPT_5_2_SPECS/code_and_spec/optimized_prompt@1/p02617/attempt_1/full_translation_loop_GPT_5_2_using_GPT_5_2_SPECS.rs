use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();

    // SPEC baseline: B = sum_{t=1..N} t(t+1)/2
    // (Matches the C code's equivalent computation via reversed indexing.)
    let mut ans: i64 = 0;
    for t in 1..=n {
        ans += t * (t + 1) / 2;
    }

    // Read exactly (N-1) pairs and subtract contributions after normalizing order.
    for _ in 0..(n - 1) {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();
        let (u, w) = if a <= b { (a, b) } else { (b, a) };
        ans -= u * (n - w + 1);
    }

    print!("{ans}\n");
}
