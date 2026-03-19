use std::io::{self, Read};

fn main() {
    // Read entire stdin; specification provides no explicit error handling.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed storage capacity up to 100 points (precondition).
    let mut xs = [0i64; 100];
    let mut ys = [0i64; 100];

    for i in 0..n {
        xs[i] = it.next().unwrap().parse().unwrap();
        ys[i] = it.next().unwrap().parse().unwrap();
    }

    // Accumulate sum over edges with wraparound successor.
    // Use i64 for intermediate integer arithmetic; specification assumes no overflow.
    let mut acc: i64 = 0;
    for i in 0..n {
        let j = if i + 1 == n { 0 } else { i + 1 };
        let term = (xs[i] - xs[j]) * (ys[i] + ys[j]);
        acc += term;
    }

    // Output signed area divided by 2, as real with one digit after decimal.
    let area = (acc as f64) / 2.0;
    println!("{:.1}", area);
}
