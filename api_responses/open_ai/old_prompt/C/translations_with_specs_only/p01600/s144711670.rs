fn main() {
    // Read the number of points
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().unwrap();

    // Ensure n is within the valid range
    if n == 0 {
        // According to the specification, behavior on invalid input is not defined.
        // However, we assume the program should not proceed if n is 0.
        return;
    }

    // Read the coordinates
    let mut x = vec![0; n];
    let mut y = vec![0; n];
    for i in 0..n {
        let mut coords = String::new();
        std::io::stdin().read_line(&mut coords).unwrap();
        let parts: Vec<i32> = coords.trim().split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        x[i] = parts[0];
        y[i] = parts[1];
    }

    // Initialize the dynamic programming table
    let mut dp = vec![vec![0; n]; n];

    // Fill the dp table with costs
    for i in 0..n {
        for j in 0..n {
            if i != j {
                dp[i][j] = (x[i] - x[j]).pow(2) + (y[i] - y[j]).pow(2);
            }
        }
    }

    // Compute the minimum cost using dynamic programming
    let mut min_cost = i32::MAX;

    // Iterate over all subsets of points
    for mask in 1..(1 << n) {
        let mut total_cost = 0;
        let mut last_point = 0;

        for i in 0..n {
            if mask & (1 << i) != 0 {
                if last_point != i {
                    total_cost += dp[last_point][i];
                }
                last_point = i;
            }
        }

        // Update the minimum cost
        min_cost = min_cost.min(total_cost);
    }

    // Output the minimum cost
    println!("{}", min_cost);
}
