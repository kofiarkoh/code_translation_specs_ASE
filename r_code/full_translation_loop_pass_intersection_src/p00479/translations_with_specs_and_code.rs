use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return, // unspecified behavior; just exit if no input
    };
    let k: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..k {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        // SPEC: da = min(a, n + 1 - a), db = min(b, n + 1 - b)
        let da = a.min(n + 1 - a);
        let db = b.min(n + 1 - b);

        // SPEC: m = min(da, db), output = ((m - 1) mod 3) + 1
        let m = da.min(db);
        let c = ((m - 1).rem_euclid(3)) + 1;

        out.push_str(&format!("{c}\n"));
    }

    print!("{out}");
}
