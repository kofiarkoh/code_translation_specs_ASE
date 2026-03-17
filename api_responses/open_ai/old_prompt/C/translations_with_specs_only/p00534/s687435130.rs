fn main() {
    // Read the number of items (n) and the number of selections (m)
    let (n, m) = read_input();

    // Initialize the weights and costs arrays
    let k = read_weights(n);
    let t = read_costs(m);

    // Create a 2D vector to hold the cost matrix
    let mut hi = vec![vec![0; m + 1]; n + 1];

    // Fill the hi matrix with initial values
    for i in 0..=n {
        for j in 0..=m {
            hi[i][j] = if j == 0 { 0 } else { i32::MAX }; // Initialize costs
        }
    }

    // Dynamic programming to compute the minimum costs
    for i in 1..=n {
        for j in 1..=m {
            // If we can select the item
            if k[i - 1] <= j as i32 {
                hi[i][j] = hi[i - 1][j].min(hi[i - 1][j - k[i - 1]] + t[j - 1]);
            } else {
                hi[i][j] = hi[i - 1][j]; // Cannot select the item
            }
        }
    }

    // The result is the minimum cost for selecting items
    let result = hi[n][m];
    if result == i32::MAX {
        println!("0"); // If no valid selection, output 0
    } else {
        println!("{}", result);
    }
}

// Function to read input values for n and m
fn read_input() -> (usize, usize) {
    // Placeholder for reading input; replace with actual input handling
    (1000, 1000) // Example values
}

// Function to read weights into an array
fn read_weights(n: usize) -> Vec<i32> {
    // Placeholder for reading weights; replace with actual input handling
    vec![1; n] // Example weights
}

// Function to read costs into an array
fn read_costs(m: usize) -> Vec<i32> {
    // Placeholder for reading costs; replace with actual input handling
    vec![1; m] // Example costs
}
