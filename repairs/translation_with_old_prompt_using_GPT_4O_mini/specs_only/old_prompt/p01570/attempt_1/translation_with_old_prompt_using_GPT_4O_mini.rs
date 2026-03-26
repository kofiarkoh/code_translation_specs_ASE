use std::io::{self, BufRead};

fn main() {
    // Read the number of nodes
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Ensure N is greater than 0
    assert!(n > 0, "N must be greater than 0");

    // Initialize arrays
    let mut m = vec![0; n]; // Node values
    let mut parent = vec![-1; n]; // Parent indices
    let mut children = vec![Vec::new(); n]; // Child nodes

    // Read node values and parent indices
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        m[i] = parts[0]; // Node value
        parent[i] = parts[1]; // Parent index

        // If the parent index is valid, add this node to its parent's children
        if parent[i] != -1 {
            let parent_index = (parent[i] as usize);
            children[parent_index].push(i);
        }
    }

    // Memoization array to store computed scores
    let mut memo = vec![None; n];

    // Function to compute the score for a node
    fn compute_score(node: usize, m: &[i32], children: &[Vec<usize>], memo: &mut Vec<Option<i64>>) -> i64 {
        // If the score is already computed, return it
        if let Some(score) = memo[node] {
            return score;
        }

        // Start with the node's own value
        let mut score = m[node] as i64;

        // Add scores from child nodes
        for &child in &children[node] {
            score += compute_score(child, m, children, memo);
        }

        // Store the computed score in memo
        memo[node] = Some(score);
        score
    }

    // Compute the total score starting from the root nodes (those with no parent)
    let mut total_score = 0;
    for i in 0..n {
        if parent[i] == -1 {
            total_score += compute_score(i, &m, &children, &mut memo);
        }
    }

    // Output the total score
    println!("{}", total_score);
}
