use std::io::{self, Read};

fn main() {
    // Read entire stdin; behavior is unspecified if input is missing/invalid (per spec),
    // so we keep parsing simple and assume preconditions hold.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: i64 = it.next().unwrap().parse().unwrap();
    let mut out = String::new();

    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap(); // modulus; spec says y > 0

        // Normalize into 1..=y, congruent to x mod y.
        // Use Euclidean remainder to avoid negative results when x <= 0.
        let r = (x - 1).rem_euclid(y) + 1;

        out.push_str(&format!("{r}\n"));
    }

    print!("{out}");
}
