fn max_cumulative_value(n: usize, k: usize, p: &[usize], c: &[i32]) -> i64 {
    // Create a vector to store the maximum cumulative values for each node
    let mut dist = vec![i64::MIN; n];
    dist[0] = c[0] as i64; // Start from the first node, assuming it's indexed at 0

    // Iterate up to K steps
    for _ in 0..k {
        let mut tmp = dist.clone(); // Temporary storage for the next step's values
        for i in 0..n {
            if dist[i] != i64::MIN { // Only consider reachable nodes
                let next_index = p[i] - 1; // Convert to 0-based index
                if next_index < n {
                    // Update the maximum cumulative value for the next node
                    tmp[next_index] = tmp[next_index].max(dist[i] + c[next_index] as i64);
                }
            }
        }
        dist = tmp; // Move to the next step
    }

    // The result is the maximum value found in dist
    let result = *dist.iter().max().unwrap_or(&i64::MIN);
    println!("{}", result); // Print the result as specified
    result // Return the result
}

fn main() {
    // Example usage (this part can be modified or removed as needed)
    let n = 5;
    let k = 3;
    let p = vec![2, 3, 4, 5, 1]; // Example directed graph
    let c = vec![1, 2, 3, 4, 5]; // Example values

    max_cumulative_value(n, k, &p, &c);
}
