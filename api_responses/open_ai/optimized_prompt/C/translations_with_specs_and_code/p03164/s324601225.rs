use std::io::{self, BufRead};

const MEW: usize = 112345;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of items and maximum weight
    let first_line = lines.next().unwrap().unwrap();
    let mut inputs = first_line.split_whitespace();
    let n: usize = inputs.next().unwrap().parse().unwrap();
    let w: usize = inputs.next().unwrap().parse().unwrap();

    // Validate number of items
    if n > 128 {
        eprintln!("error!");
        return;
    }

    let mut weights = vec![0; n];
    let mut values = vec![0; n];

    // Read weights and values of items
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut item_inputs = line.split_whitespace();
        weights[i] = item_inputs.next().unwrap().parse().unwrap();
        values[i] = item_inputs.next().unwrap().parse().unwrap();
    }

    let mut ayaneru = vec![w + 1; MEW];
    ayaneru[0] = 0;

    // Dynamic programming to compute minimum weights for each possible value
    for i in 0..n {
        for j in (values[i]..MEW).rev() {
            if ayaneru[j - values[i]] + weights[i] < ayaneru[j] {
                ayaneru[j] = ayaneru[j - values[i]] + weights[i];
            }
        }
    }

    // Find the maximum value that can be achieved without exceeding the weight limit
    for i in (0..MEW).rev() {
        if ayaneru[i] <= w {
            println!("{}", i);
            return;
        }
    }

    // If no valid configuration exists
    println!("error!");
}
