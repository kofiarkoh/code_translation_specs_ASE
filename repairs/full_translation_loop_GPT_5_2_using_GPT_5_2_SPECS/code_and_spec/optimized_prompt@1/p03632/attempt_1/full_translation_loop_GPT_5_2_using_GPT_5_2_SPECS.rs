use std::io::{self, Read};

fn main() {
    // Read all input from stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let a: i32 = it.next().unwrap().parse().unwrap();
    let b: i32 = it.next().unwrap().parse().unwrap();
    let c: i32 = it.next().unwrap().parse().unwrap();
    let d: i32 = it.next().unwrap().parse().unwrap();

    // Per spec: compute K = number of integers in intersection of [A,B] and [C,D]
    // Then output 0 if K==0 else K-1.
    let left = a.max(c);
    let right = b.min(d);

    let k: i32 = if left <= right { right - left + 1 } else { 0 };
    let out: i32 = if k == 0 { 0 } else { k - 1 };

    // Match C behavior: print integer with no trailing newline.
    print!("{out}");
}
