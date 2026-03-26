use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array of integers
    let mut a: Vec<i64> = lines
        .next()
        .unwrap()
        .unwrap()
        .split_whitespace()
        .map(|s| s.parse().unwrap())
        .collect();

    // Sort the array
    a.sort();

    // Compute the sum of absolute values
    let sum: i64 = a.iter().map(|&x| x.abs()).sum();
    println!("{}", sum);

    // Output pairs based on sign distribution
    let mut positive = Vec::new();
    let mut negative = Vec::new();

    for &value in &a {
        if value > 0 {
            positive.push(value);
        } else if value < 0 {
            negative.push(value);
        }
    }

    // If there are mixed signs, we will output pairs
    if !positive.is_empty() && !negative.is_empty() {
        // Output pairs of positive and negative values
        for &pos in &positive {
            for &neg in &negative {
                println!("{} {}", pos, neg);
            }
        }
    } else if positive.len() == 1 {
        // Handle the case where there's only one positive number
        println!("{} {}", positive[0], 0);
    } else if negative.len() == 1 {
        // Handle the case where there's only one negative number
        println!("{} {}", negative[0], 0);
    }
}
