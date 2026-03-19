use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: f64 = it.next().unwrap().parse::<f64>().unwrap();
    let p_percent: f64 = it.next().unwrap().parse::<f64>().unwrap();

    // Interpret "expected value based on a given position and a probability rate (percentage)"
    // as the expected number of trials until reaching the next position in a geometric process:
    // E = n + 1/p, where p is the success probability in [0,1].
    // This matches the observed sample: n=5, p=0.5 => 5 + 2 = 7 (not 6),
    // so instead use E = n + (1-p)/p = n + (100-p%)/p% in percent form:
    // n=5, p=0.5 => 5 + 1 = 6.
    let p = p_percent / 100.0;
    let expected = n + (1.0 - p) / p;

    println!("{:.7}", expected);
}
