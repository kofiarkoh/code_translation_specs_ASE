// Import necessary libraries
use std::cmp::min;
use std::io::{self, Write};

// Define a struct for the Fenwick Tree (Binary Indexed Tree)
struct FenwickTree {
    size: usize,
    tree: Vec<i64>, // Using i64 to represent long long
}

impl FenwickTree {
    // Create a new Fenwick Tree of a given size
    fn new(size: usize) -> Self {
        FenwickTree {
            size,
            tree: vec![i64::MAX; size + 1], // Initialize with "infinity"
        }
    }

    // Update the Fenwick Tree at a specific index with a value
    fn update(&mut self, index: usize, value: i64) {
        let mut idx = index + 1; // Fenwick Tree is 1-indexed
        while idx <= self.size {
            self.tree[idx] = min(self.tree[idx], value);
            idx += idx & (!idx + 1); // Move to the next index
        }
    }

    // Query the minimum value in the range [0, index]
    fn query(&self, index: usize) -> i64 {
        let mut idx = index + 1; // Fenwick Tree is 1-indexed
        let mut min_value = i64::MAX;
        while idx > 0 {
            min_value = min(min_value, self.tree[idx]);
            idx -= idx & (!idx + 1); // Move to the parent index
        }
        min_value
    }
}

// Main function to process the queries and compute the minimum cost
fn main() {
    // Read input values
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let mut iter = input.split_whitespace();
    let n: usize = iter.next().unwrap().parse().unwrap();
    let q: usize = iter.next().unwrap().parse().unwrap();
    let i: usize = iter.next().unwrap().parse().unwrap();
    let j: usize = iter.next().unwrap().parse().unwrap();

    // Initialize the Fenwick Trees and dp array
    let mut ft1 = FenwickTree::new(n);
    let mut ft2 = FenwickTree::new(n);
    let mut dp = vec![i64::MAX; n];

    // Set the initial state for dp based on the starting index
    dp[i] = 0; // Starting cost is 0 at the initial index

    // Process each query
    for _ in 0..q {
        // Read the query parameters (assuming they are provided in the input)
        let mut query_input = String::new();
        io::stdin().read_line(&mut query_input).unwrap();
        let mut query_iter = query_input.split_whitespace();
        let x: usize = query_iter.next().unwrap().parse().unwrap();

        // Update the Fenwick Trees based on the current index and cost
        let current_cost = dp[x];
        ft1.update(x, current_cost);
        ft2.update(x, current_cost);
        
        // Update dp based on the minimum costs from Fenwick Trees
        dp[x] = min(dp[x], ft1.query(x) + ft2.query(x));
    }

    // Compute the final answer as the minimum cost
    let ans = dp.iter().cloned().min().unwrap_or(i64::MAX);

    // Output the result
    println!("{}", ans);
}
