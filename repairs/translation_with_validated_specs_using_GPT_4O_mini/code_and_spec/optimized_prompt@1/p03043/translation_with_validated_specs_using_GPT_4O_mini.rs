use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().unwrap().parse().expect("Invalid input for N");
    let k: usize = iter.next().unwrap().parse().expect("Invalid input for K");

    // Preconditions: N and K must be positive
    assert!(n > 0, "N must be positive");
    assert!(k > 0, "K must be positive");

    let mut multi = 1;
    let mut denomi = vec![0; n];
    let mut nume = 0.0;

    for i in (0..n).rev() {
        // Initialize denominator to zero
        denomi[i] = 0;

        // Compute the denominator based on the multiplier
        while k > multi * (i + 1) {
            multi *= 2;
        }
        denomi[i] += multi * n;
    }

    // Calculate the cumulative sum of ratios
    for j in 0..n {
        // Avoid division by zero; if denomi[0] is zero, the output will be undefined
        nume += denomi[0] as f64 / denomi[j] as f64;
    }

    // Output the result formatted to twelve decimal places
    if denomi[0] != 0 {
        println!("{:.12}", nume / denomi[0] as f64);
    } else {
        // Handle the case where the first denominator is zero
        println!("undefined");
    }
}
