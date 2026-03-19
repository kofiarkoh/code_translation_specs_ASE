use std::io::{self, Read};

fn main() {
    // Read entire stdin; if input cannot be read correctly, behavior is undefined per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // If n is missing or not an integer, behavior is undefined per spec; unwrap to mirror that.
    let n_i64: i64 = it.next().unwrap().parse().unwrap();

    // If n is negative, behavior is not defined per spec; we do not add extra handling.
    let n = n_i64 as usize;

    // Read exactly n integers; if fewer/malformed, behavior is undefined per spec.
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Longest non-increasing contiguous subsequence length.
    // For n == 0, the maximum length is 0 (n is non-negative; empty sequence).
    let mut best: usize = 0;
    let mut cur: usize = 0;

    for i in 0..n {
        if i == 0 {
            cur = 1;
        } else if a[i - 1] >= a[i] {
            cur += 1;
        } else {
            cur = 1;
        }
        if cur > best {
            best = cur;
        }
    }

    print!("{best}");
}
