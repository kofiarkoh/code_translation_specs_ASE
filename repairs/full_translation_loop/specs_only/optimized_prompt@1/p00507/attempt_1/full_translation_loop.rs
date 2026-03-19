use std::collections::HashSet;
use std::cmp::Ordering;

fn find_third_smallest_concatenation(numbers: Vec<i32>) -> Option<String> {
    let n = numbers.len();
    
    // Check precondition: N must be at least 2
    if n < 2 {
        return None; // Not enough numbers to form a pair
    }

    let mut unique_concats = HashSet::new();

    // Generate unique concatenations of distinct pairs
    for i in 0..n {
        for j in 0..n {
            if i != j {
                // Concatenate numbers as strings
                let concat1 = format!("{}{}", numbers[i], numbers[j]);
                unique_concats.insert(concat1);
            }
        }
    }

    // Collect unique concatenations into a vector and sort them
    let mut sorted_concats: Vec<String> = unique_concats.into_iter().collect();
    sorted_concats.sort();

    // Check if we have at least three unique concatenations
    if sorted_concats.len() < 3 {
        return None; // Undefined behavior as per specification
    }

    // Return the third smallest unique concatenation
    Some(sorted_concats[2].clone())
}

fn main() {
    // Example usage
    let numbers = vec![12, 34, 56];
    match find_third_smallest_concatenation(numbers) {
        Some(result) => println!("{}", result),
        None => println!("Not enough unique concatenations."),
    }
}
