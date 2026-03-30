use std::cmp::Ordering;

fn process_array(n: usize, arr: &mut Vec<i64>) -> (i64, Vec<(i64, i64)>) {
    // Preconditions: n is non-negative and arr contains n long long integers.
    
    // Step 1: Sort the array in non-decreasing order
    arr.sort_unstable();

    // Step 2: Compute the sum of absolute values
    let sum: i64 = arr.iter().map(|&x| x.abs()).sum();

    // Step 3: Prepare to collect pairs based on relationships
    let mut pairs = Vec::new();

    // Recursive depth-first search function to explore relationships
    fn dfs(arr: &[i64], index: usize, pairs: &mut Vec<(i64, i64)>) {
        if index >= arr.len() {
            return;
        }

        // Example relationship: pair current element with the next one
        if index + 1 < arr.len() {
            pairs.push((arr[index], arr[index + 1]));
        }

        // Continue the search
        dfs(arr, index + 1, pairs);
    }

    // Start the depth-first search from the first index
    dfs(arr, 0, &mut pairs);

    // Return the computed sum and the collected pairs
    (sum, pairs)
}

fn main() {
    // Example usage
    let n = 5;
    let mut arr = vec![3, -1, 2, -4, 0];
    let (computed_sum, pairs) = process_array(n, &mut arr);

    // Output the computed sum
    println!("{}", computed_sum);
    
    // Output the pairs
    for (a, b) in pairs {
        println!("({}, {})", a, b);
    }
}
