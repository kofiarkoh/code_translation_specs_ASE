use std::io::{self, Write};

fn f(n: u64, k: u64) -> u64 {
    // Base case: if n is 0, return 1 if k is 0, otherwise return 0
    if n == 0 {
        return if k == 0 { 1 } else { 0 };
    }
    
    // Calculate the total number of nodes in a complete binary tree of depth n
    let total_nodes = (1 << n) - 1; // This is 2^n - 1

    // If k is greater than the total number of nodes, return 0
    if k > total_nodes {
        return 0;
    }

    // Recursive case: compute the value based on the position k
    // The left subtree has (total_nodes + 1) / 2 nodes
    let left_subtree_size = (total_nodes + 1) / 2;

    // If k is in the left subtree
    if k <= left_subtree_size {
        return f(n - 1, k);
    } else {
        // If k is in the right subtree, adjust k to reflect its position
        return f(n - 1, k - left_subtree_size);
    }
}

fn main() {
    let mut input = String::new();
    // Read input from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into n and k
    let mut iter = input.split_whitespace();
    let n: u64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: u64 = iter.next().unwrap().parse().expect("Invalid input for k");

    // Call the function and print the result
    let result = f(n, k);
    println!("{}", result);
}
