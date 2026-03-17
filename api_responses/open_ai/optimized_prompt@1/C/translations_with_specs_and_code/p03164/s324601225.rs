use std::io::{self, Write};

const MEW: usize = 112345;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("Invalid input for number of items");
        std::process::exit(1);
    });
    
    let w_capacity: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
        eprintln!("Invalid input for weight capacity");
        std::process::exit(1);
    });

    if n == 0 || w_capacity == 0 {
        println!("error!");
        return;
    }

    let mut weights = vec![0; n];
    let mut values = vec![0; n];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut item_iter = input.split_whitespace();
        
        weights[i] = item_iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
            eprintln!("Invalid input for weight");
            std::process::exit(1);
        });
        
        values[i] = item_iter.next().and_then(|s| s.parse().ok()).unwrap_or_else(|| {
            eprintln!("Invalid input for value");
            std::process::exit(1);
        });
    }

    let mut ayaneru = vec![w_capacity + 1; MEW];
    ayaneru[0] = 0;

    for i in 0..n {
        for j in (values[i]..MEW).rev() {
            if ayaneru[j - values[i]] + weights[i] < ayaneru[j] {
                ayaneru[j] = ayaneru[j - values[i]] + weights[i];
            }
        }
    }

    for i in (0..MEW).rev() {
        if ayaneru[i] <= w_capacity {
            println!("{}", i);
            return;
        }
    }

    println!("error!");
}
