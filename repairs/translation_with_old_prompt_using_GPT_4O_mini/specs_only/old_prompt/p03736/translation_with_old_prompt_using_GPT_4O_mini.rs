struct FenwickTree {
    size: usize,
    tree: Vec<i64>, // Using i64 to represent long long
}

impl FenwickTree {
    fn new(size: usize) -> Self {
        FenwickTree {
            size,
            tree: vec![i64::MAX; size + 1], // Initialize with "infinity"
        }
    }

    fn update(&mut self, index: usize, value: i64) {
        let mut idx = index + 1; // Fenwick tree is 1-indexed
        while idx <= self.size {
            self.tree[idx] = self.tree[idx].min(value);
            idx += idx & !idx + 1; // Move to the next index
        }
    }

    fn query(&self, index: usize) -> i64 {
        let mut idx = index + 1; // Fenwick tree is 1-indexed
        let mut min_value = i64::MAX;
        while idx > 0 {
            min_value = min_value.min(self.tree[idx]);
            idx -= idx & !idx + 1; // Move to the parent index
        }
        min_value
    }
}

fn minimize_cost(n: usize, q: usize, i: usize, j: usize, queries: &[usize]) -> i64 {
    let mut dp = vec![i64::MAX; n]; // Initialize dp array with "infinity"
    let mut ft1 = FenwickTree::new(n);
    let mut ft2 = FenwickTree::new(n);
    
    // Initial state based on the problem description
    dp[i] = 0; // Starting cost at index i is 0
    ft1.update(i, 0); // Update Fenwick tree with initial cost
    ft2.update(i, 0); // Update second Fenwick tree with initial cost

    // Process queries
    for &query in queries {
        if query < n { // Ensure the query index is within bounds
            let current_cost = ft1.query(query);
            let new_cost = current_cost + 1; // Example cost increment
            dp[query] = dp[query].min(new_cost);
            ft1.update(query, dp[query]);
        }
    }

    // The answer is the minimum cost found in the dp array
    dp.iter().cloned().min().unwrap_or(i64::MAX) // Return minimum cost
}

fn main() {
    let n = 5; // Example size
    let q = 3; // Example number of queries
    let i = 1; // Initial index
    let j = 3; // Target index for the first update
    let queries = vec![2, 4, 5]; // Example queries

    let result = minimize_cost(n, q, i, j, &queries);
    println!("{}", result); // Output the minimum cost
}
