fn main() {
    use std::io::{self, BufRead};

    // Read the number of nodes
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line for the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is greater than 0
    if n == 0 {
        return; // Undefined behavior as per specification
    }

    // Initialize arrays
    let mut m = vec![0; n]; // Node values
    let mut to: Vec<Vec<usize>> = vec![Vec::new(); n]; // Adjacency list for child nodes
    let mut memo = vec![-1; n]; // Memoization array for computed scores

    // Read node values and parent indices
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for node value or parent index"))
            .collect();
        m[i] = parts[0]; // Node value
        let parent_index = parts[1];

        // If parent_index is not -1, add this node to its parent's children
        if parent_index != -1 {
            let parent_index = (parent_index as usize);
            to[parent_index].push(i);
        }
    }

    // Function to compute the score for a node
    fn compute_score(node: usize, m: &Vec<i32>, to: &Vec<Vec<usize>>, memo: &mut Vec<i64>) -> i64 {
        // If the score has already been computed, return it
        if memo[node] != -1 {
            return memo[node];
        }

        // Start with the node's own value
        let mut score = m[node] as i64;

        // Add scores from child nodes
        for &child in &to[node] {
            score += compute_score(child, m, to, memo);
        }

        // Store the computed score in memo
        memo[node] = score;
        score
    }

    // Compute the total score starting from the root nodes (those with no parent)
    let mut total_score = 0;
    for i in 0..n {
        if !to.iter().any(|children| children.contains(&i)) { // Check if it's a root node
            total_score += compute_score(i, &m, &to, &mut memo);
        }
    }

    // Output the total score
    println!("{}", total_score);
}
