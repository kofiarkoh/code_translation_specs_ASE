use std::io::{self, BufRead};

const MEW: usize = 112345;

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();

    // Read N and W
    let first_line = input.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let w: usize = parts.next().unwrap().parse().unwrap();

    // Validate N
    if n > 128 {
        eprintln!("error!");
        std::process::exit(1);
    }

    let mut weights = vec![0; n];
    let mut values = vec![0; n];

    // Read weights and values
    for i in 0..n {
        let line = input.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        weights[i] = parts.next().unwrap().parse().unwrap();
        values[i] = parts.next().unwrap().parse().unwrap();
    }

    // Initialize the ayaneru array
    let mut ayaneru = vec![w + 1; MEW];
    ayaneru[0] = 0;

    // Process the items
    for i in 0..n {
        for j in (values[i]..MEW).rev() {
            if ayaneru[j - values[i]] + weights[i] < ayaneru[j] {
                ayaneru[j] = ayaneru[j - values[i]] + weights[i];
            }
        }
    }

    // Find the maximum value that can be achieved
    for i in (0..MEW).rev() {
        if ayaneru[i] <= w {
            println!("{}", i);
            return;
        }
    }

    // If no valid configuration exists
    println!("error!");
}
