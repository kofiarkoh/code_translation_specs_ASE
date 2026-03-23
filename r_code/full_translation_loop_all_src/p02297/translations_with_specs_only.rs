use std::io::{self, Read};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed storage capacity up to 100 vertices as per specification.
    // If n > 100, behavior is unspecified; we still attempt to read and compute.
    let mut xs: [i64; 100] = [0; 100];
    let mut ys: [i64; 100] = [0; 100];

    for i in 0..n {
        let x: i64 = it.next().unwrap().parse().unwrap();
        let y: i64 = it.next().unwrap().parse().unwrap();
        if i < 100 {
            xs[i] = x;
            ys[i] = y;
        }
    }

    // Signed area via shoelace sum over the given traversal order, closing last->first.
    // Preconditions state intermediate arithmetic does not overflow; we use i64.
    let mut sum: i64 = 0;
    if n >= 1 {
        for i in 0..n {
            let j = (i + 1) % n;
            let xi = if i < 100 { xs[i] } else { 0 };
            let yi = if i < 100 { ys[i] } else { 0 };
            let xj = if j < 100 { xs[j] } else { 0 };
            let yj = if j < 100 { ys[j] } else { 0 };
            sum += xi * yj - xj * yi;
        }
    }

    let area = (sum as f64) / 2.0;
    print!("{:.1}", area);
}
