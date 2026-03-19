use std::io::{self, Read};

fn main() {
    // Read entire stdin, parse as integers (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let k: i64 = it.next().unwrap().parse().unwrap();

    let mut out = String::new();

    for _ in 0..k {
        let mut a: i64 = it.next().unwrap().parse().unwrap();
        let mut b: i64 = it.next().unwrap().parse().unwrap();

        // d(p) = min(p, n + 1 - p)
        let na = n + 1 - a;
        if na < a {
            a = na;
        }
        let nb = n + 1 - b;
        if nb < b {
            b = nb;
        }

        let m = if a < b { a } else { b };
        let c = ((m - 1).rem_euclid(3)) + 1;

        out.push_str(&format!("{c}\n"));
    }

    print!("{out}");
}
