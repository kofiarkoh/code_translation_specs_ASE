use std::io::{self, Read};

fn main() {
    // Read all input (no explicit error handling per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i32 = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Collect S = { n - x | x < n } (only x matters; y is ignored for result).
    let mut diffs: Vec<i32> = Vec::new();
    for _ in 0..m {
        let x: i32 = it.next().unwrap().parse().unwrap();
        let _y: i32 = it.next().unwrap().parse().unwrap();
        if x < n {
            diffs.push(n - x);
        }
    }

    // Spec: if q <= 1 => 0; else sum of smallest (q - 1) values.
    let q = diffs.len();
    let result: i32 = if q <= 1 {
        0
    } else {
        diffs.sort(); // ascending
        diffs[..(q - 1)].iter().sum()
    };

    print!("{result}\n");
}
