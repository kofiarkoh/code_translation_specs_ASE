use std::collections::HashSet;
use std::cmp::Ordering;

fn third_smallest_combination(n: usize, a: Vec<i32>) -> i32 {
    // Ensure that we have at least two distinct integers
    if n < 2 {
        panic!("N must be greater than 1");
    }

    // Use a HashSet to store unique combinations
    let mut combinations = HashSet::new();

    // Generate combinations of two distinct integers
    for i in 0..n {
        for j in (i + 1)..n {
            // Concatenate the two integers in the specified manner
            let combination1 = a[i] * 10_i32.pow(10) + a[j]; // a[i] followed by a[j]
            let combination2 = a[j] * 10_i32.pow(10) + a[i]; // a[j] followed by a[i]
            combinations.insert(combination1);
            combinations.insert(combination2);
        }
    }

    // Collect unique combinations and sort them
    let mut unique_combinations: Vec<i32> = combinations.into_iter().collect();
    unique_combinations.sort_unstable();

    // Return the third smallest unique combination if it exists
    if unique_combinations.len() < 3 {
        panic!("Not enough unique combinations found");
    }

    // Print the result as specified
    let result = unique_combinations[2];
    println!("{}", result);
    result
}

fn main() {
    // Example usage
    let n = 5;
    let a = vec![1, 2, 3, 4, 5];
    third_smallest_combination(n, a);
}
