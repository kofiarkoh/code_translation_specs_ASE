fn main() {
    // Example input values; in a real scenario, these would be read from standard input.
    let n = 5; // Number of items
    let m = 3; // Number of selections
    let k = [0, 2, 3, 4, 5, 6]; // Weights (1-indexed)
    let t = [0, 1, 2, 3]; // Costs (1-indexed)

    // Initialize the cost matrix `hi` with a large value (infinity equivalent)
    let inf = i32::MAX; // Use maximum integer value to represent infinity
    let mut hi = vec![vec![inf; m + 1]; n + 1];

    // Base case: cost of selecting 0 items is 0
    for j in 0..=m {
        hi[0][j] = 0;
    }

    // Fill the cost matrix using dynamic programming
    for i in 1..=n {
        for j in 1..=m {
            // If the weight of the current item is less than or equal to the current selection
            if k[i] <= j as i32 {
                // Choose the minimum between not taking the item or taking it
                hi[i][j] = hi[i - 1][j].min(hi[i - 1][j - k[i]] + t[j]);
            } else {
                // If the weight exceeds the current selection, we cannot take this item
                hi[i][j] = hi[i - 1][j];
            }
        }
    }

    // The result is the minimum cost for selecting items
    let result = hi[n][m];
    
    // Output the result
    println!("{}", result);
}
