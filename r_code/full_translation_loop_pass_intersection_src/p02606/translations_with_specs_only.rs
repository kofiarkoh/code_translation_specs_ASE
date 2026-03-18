use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let l: i128 = it.next().unwrap().parse().unwrap();
    let r: i128 = it.next().unwrap().parse().unwrap();
    let d: i128 = it.next().unwrap().parse().unwrap();

    // Preconditions per specification:
    // - l <= r
    // - d > 0
    // If d == 0, behavior is undefined; we do not add explicit handling.

    // Count of multiples of d in [l, r] is:
    // floor_div(r, d) - floor_div(l - 1, d)
    // Use Euclidean division to match mathematical floor for positive d,
    // including when l or r are negative.
    let count = r.div_euclid(d) - (l - 1).div_euclid(d);

    println!("{}", count);
}
