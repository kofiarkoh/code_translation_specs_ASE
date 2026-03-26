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

    // Separate positive and negative values
    let mut positive = Vec::new();
    let mut negative = Vec::new();

    for &value in &a {
        if value > 0 {
            positive.push(value);
        } else if value < 0 {
            negative.push(value);
        }
    }

    // Output pairs based on sign distribution
    if !positive.is_empty() && !negative.is_empty() {
        // Output pairs of positive and negative values
        for &pos in &positive {
            for &neg in &negative {
                println!("{} {}", pos, neg);
            }
        }
    }

    // Handle the case where there's only one positive or one negative number
    if positive.len() == 1 && negative.len() == 1 {
        // If there's only one positive and one negative number, output their pair
        println!("{} {}", positive[0], negative[0]);
    }

    // Handle the case where there are multiple positives or negatives
    if positive.len() > 1 {
        for i in 0..positive.len() {
            for j in 0..negative.len() {
                let pair_sum = positive[i] + negative[j];
                println!("{} {}", pair_sum, negative[j]);
            }
        }
    }
}
