use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Unspecified behavior; exit quietly if missing.
    };
    let k: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // Unspecified behavior; exit quietly if missing.
    };

    let mut out = String::new();

    for _ in 0..k {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        // da = min(a, n + 1 - a), db = min(b, n + 1 - b)
        let da = a.min(n + 1 - a);
        let db = b.min(n + 1 - b);

        let m = da.min(db);

        // Output ((m - 1) mod 3) + 1
        // Using Euclidean remainder to keep result in 0..2 even if inputs violate intended bounds.
        let ans = (m - 1).rem_euclid(3) + 1;

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
