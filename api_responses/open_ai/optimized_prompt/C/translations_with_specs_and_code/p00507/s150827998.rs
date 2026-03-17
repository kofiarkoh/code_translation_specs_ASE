use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Preconditions: N must be a positive integer
    if n < 2 {
        return; // Not enough integers to form a valid output
    }

    // Read the integers into a vector
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let num: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        a.push(num);
    }

    // Use a HashSet to store unique concatenated results
    let mut unique_concats: HashSet<i64> = HashSet::new();

    // Generate unique concatenations
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Calculate the concatenation of a[i] and a[j]
                let k = 10i64.pow(num_digits(a[j]));
                let concat = (a[i] as i64) * k + (a[j] as i64);
                unique_concats.insert(concat);
            }
        }
    }

    // Collect unique concatenations and sort them
    let mut sorted_concats: Vec<i64> = unique_concats.into_iter().collect();
    sorted_concats.sort_unstable();

    // Postconditions: Output the third smallest unique concatenation
    if sorted_concats.len() >= 3 {
        println!("{}", sorted_concats[2]);
    }
}

// Helper function to count the number of digits in a number
fn num_digits(mut num: i32) -> u32 {
    let mut count = 0;
    while num > 0 {
        num /= 10;
        count += 1;
    }
    count
}
