use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    
    // Initialize a vector to hold the integers
    let mut numbers: Vec<i64> = Vec::with_capacity(n);

    // Read the integers into the vector
    for _ in 0..n {
        let num: i64 = lines.next().unwrap().unwrap().parse().unwrap();
        numbers.push(num);
    }

    // Sort the numbers in non-decreasing order
    numbers.sort();

    // Compute the absolute sum
    let absolute_sum: i64 = numbers.iter().map(|&x| x.abs()).sum();

    // Output the absolute sum
    println!("{}", absolute_sum);

    // Categorize the input values and produce output pairs
    let (positive_count, negative_count) = numbers.iter().fold((0, 0), |(pos, neg), &x| {
        if x > 0 {
            (pos + 1, neg)
        } else if x < 0 {
            (pos, neg + 1)
        } else {
            (pos, neg)
        }
    });

    // Output pairs based on categorization
    if positive_count > 0 && negative_count > 0 {
        // Mixed case
        println!("Mixed: {} positives, {} negatives", positive_count, negative_count);
    } else if positive_count > 0 {
        // All positive case
        println!("All positive: {}", positive_count);
    } else if negative_count > 0 {
        // All negative case
        println!("All negative: {}", negative_count);
    } else {
        // All zeros case (not specified, but handled for completeness)
        println!("All zeros");
    }
}
