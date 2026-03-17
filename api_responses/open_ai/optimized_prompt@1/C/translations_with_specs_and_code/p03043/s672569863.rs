use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().expect("N must be a positive integer");
    let k: usize = iter.next().unwrap().parse().expect("K must be a non-negative integer");

    // Preconditions: N must be positive and K must be non-negative
    assert!(n > 0, "N must be a positive integer");
    assert!(k >= 0, "K must be a non-negative integer");

    let mut multi = 1;
    let mut denomi = vec![0; n];
    let mut nume = 0.0;

    for i in (0..n).rev() {
        denomi[i] = 0;
        while k > multi * (i + 1) {
            multi *= 2;
        }
        denomi[i] += multi * n;
    }

    for j in 0..n {
        nume += denomi[0] as f64 / denomi[j] as f64;
    }

    // Output the result formatted to twelve decimal places
    println!("{:.12}", nume / denomi[0] as f64);
}
