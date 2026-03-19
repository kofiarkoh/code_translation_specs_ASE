use std::io::{self, Read};

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();

    // Specification: first token is number of vertices (positive integer).
    // If input is missing/malformed, behavior is not specified; we keep it minimal and unwrap.
    let n: usize = it.next().unwrap().parse().unwrap();

    // Read vertices as i64 to reduce risk of overflow vs i32; still finite.
    let mut xs: Vec<i64> = Vec::with_capacity(n);
    let mut ys: Vec<i64> = Vec::with_capacity(n);

    for _ in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        xs.push(x);
        ys.push(y);
    }

    // Shoelace formula: area = |sum(x_i*y_{i+1} - y_i*x_{i+1})| / 2
    // Linear time as required.
    // Use i128 accumulator to better preserve exact integer arithmetic before converting to float.
    let mut sum: i128 = 0;
    for i in 0..n {
        let j = (i + 1) % n;
        sum += (xs[i] as i128) * (ys[j] as i128) - (ys[i] as i128) * (xs[j] as i128);
    }

    let area = (sum.abs() as f64) / 2.0;

    // Output formatted to one decimal place.
    println!("{:.1}", area);
}
