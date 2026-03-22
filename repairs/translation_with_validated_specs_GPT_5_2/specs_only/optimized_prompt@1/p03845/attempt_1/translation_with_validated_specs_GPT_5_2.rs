use std::io::{self, Read};

fn main() {
    // Read entire stdin; input is assumed well-formed per specification.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // The specification says behavior is undefined if preconditions are violated.
    // We therefore parse directly and index without extra checks.
    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i128> = Vec::with_capacity(n);
    let mut s: i128 = 0;
    for _ in 0..n {
        let v: i128 = it.next().unwrap().parse().unwrap();
        a.push(v);
        s += v;
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    let mut out = String::new();
    for _ in 0..m {
        let p: usize = it.next().unwrap().parse().unwrap(); // 1-based
        let x: i128 = it.next().unwrap().parse().unwrap();

        let orig = a[p - 1];
        let ans = s - orig + x;
        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
