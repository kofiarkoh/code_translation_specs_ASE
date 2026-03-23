use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: i64 = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..t {
        let value: i128 = it.next().unwrap().parse().unwrap();
        let modulus: i128 = it.next().unwrap().parse().unwrap();

        // Preconditions state modulus > 0; if not, behavior is unspecified.
        // We implement the specified normalization into 1..=modulus:
        // r = ((value - 1) mod modulus) + 1, using Euclidean remainder.
        let r = (value - 1).rem_euclid(modulus) + 1;

        out.push_str(&r.to_string());
        out.push('\n');
    }

    print!("{out}");
}
