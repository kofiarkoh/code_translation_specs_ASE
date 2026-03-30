fn process_array(n: usize, arr: &mut Vec<i64>) -> (i64, Vec<(i64, i64)>) {
    // Preconditions: n is non-negative and arr contains n long long integers.
    
    // Step 1: Sort the array in non-decreasing order
    arr.sort_unstable();

    // Step 2: Compute the sum of absolute values
    let sum: i64 = arr.iter().map(|&x| x.abs()).sum();

    // Step 3: Prepare to collect pairs based on relationships
    let mut pairs = Vec::new();

    // Collect pairs based on the sorted array
    for i in 0..(n + 1) / 2 {
        pairs.push((arr[n - 1 - i], arr[i])); // Pairing from both ends
    }

    // Return the computed sum and the collected pairs
    (sum, pairs)
}

fn main() {
    // Example usage
    let n = 7;
    let mut arr = vec![-3, -2, -1, 0, 1, 2, 3];
    let (computed_sum, pairs) = process_array(n, &mut arr);

    // Output the computed sum
    println!("{}", computed_sum);
    
    // Output the pairs
    for (a, b) in pairs {
        println!("{} {}", a, b);
    }
}
