use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let n: i128 = match s.split_whitespace().next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Interpret "derived from its divisibility by 11 and its remainder when divided by 11"
    // as: base = (n/11)*2, and res depends only on whether remainder is zero or not.
    // This matches the common pattern: add 0 if divisible by 11, else add 1.
    // Use Euclidean division to keep remainder non-negative.
    let q = n.div_euclid(11);
    let r = n.rem_euclid(11);

    let res: i128 = if r == 0 { 0 } else { 1 };
    let out = q * 2 + res;

    print!("{out}");
}
