use std::io::{self, Read};

fn main() {
    // Read entire stdin; undefined behavior on invalid input per spec, so we unwrap parsing.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let position: i64 = it.next().unwrap().parse().unwrap();
    let rate_percent: f64 = it.next().unwrap().parse().unwrap();

    // Probability rate is given as a percentage; convert to [0,1] scale.
    // Spec does not define behavior outside [0,100]; we do not clamp.
    let p = rate_percent / 100.0;

    // Specification does not provide the exact formula for "expected value".
    // Narrowest reasonable interpretation: expected value = position * probability.
    let expected = (position as f64) * p;

    // Output formatted to seven decimal places.
    println!("{:.7}", expected);
}
