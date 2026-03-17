fn process_pairs(n: i32, m: usize, a: &mut Vec<i32>, b: &mut Vec<i32>) -> i32 {
    // Ensure that the length of a and b is at least m + 1
    assert!(a.len() >= m + 1 && b.len() >= m + 1, "Arrays must have at least m + 1 elements");

    let mut count = 0;

    // Iterate through the pairs
    for i in 0..m {
        // Check if the first element exceeds the threshold
        if a[i as usize] > n {
            // If it does, add the second element to the count
            count += b[i as usize];
        }
    }

    // Sort the second elements of the pairs where the first element exceeds the threshold
    let mut selected_b: Vec<i32> = a.iter()
        .enumerate()
        .filter_map(|(i, &first)| if first > n { Some(b[i]) } else { None })
        .collect();

    // Sort the selected values
    selected_b.sort();

    // Calculate the sum of the selected values
    let sum: i32 = selected_b.iter().sum();

    // Print the result
    println!("{}", sum);

    // Return the computed count
    sum
}

fn main() {
    // Example usage
    let n = 5; // threshold
    let m = 3; // number of pairs
    let mut a = vec![1, 6, 3, 8]; // first elements
    let mut b = vec![2, 4, 5, 7]; // second elements

    // Call the function with the provided parameters
    process_pairs(n, m, &mut a, &mut b);
}
