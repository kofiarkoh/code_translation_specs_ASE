use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let a: i128 = it.next().unwrap().parse().unwrap();
    let b: i128 = it.next().unwrap().parse().unwrap();

    // Specification assumes b > 0 and b != 0; no explicit error handling.
    // Use the standard integer-division identity for positive divisors:
    // ceil(a / b) = a / b + (a % b != 0 ? 1 : 0)
    // This matches the described behavior for positive b, including negative a.
    let q = a / b;
    let r = a % b;
    let ans = if r == 0 { q } else { q + 1 };

    println!("{ans}");
}
