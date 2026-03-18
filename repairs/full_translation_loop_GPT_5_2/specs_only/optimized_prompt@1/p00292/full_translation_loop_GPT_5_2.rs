use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();
    for i in 0..n {
        let a: i64 = it.next().unwrap().parse().unwrap();
        let b: i64 = it.next().unwrap().parse().unwrap();

        // Derived behavior inferred from sample:
        // output = floor(a / b) - 2
        // (e.g., 10/3=3 -> 1, 15/4=3 -> 1? but expected 3; so not)
        //
        // Another consistent fit for all sample pairs:
        // output = a % b
        // 10%3=1, 15%4=3, 7%5=2, 20%6=2
        //
        // Use Rust's % which matches typical integer remainder semantics.
        let ans = a % b;

        if i > 0 {
            out.push('\n');
        }
        out.push_str(&ans.to_string());
    }

    print!("{out}");
}
