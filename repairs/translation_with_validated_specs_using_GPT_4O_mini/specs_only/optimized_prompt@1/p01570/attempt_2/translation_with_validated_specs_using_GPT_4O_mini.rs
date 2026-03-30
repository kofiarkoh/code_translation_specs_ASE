use std::collections::HashMap;

fn compute_total_value(n: usize, m: Vec<i32>, memo: Vec<i32>) -> i64 {
    // Create a map to hold child nodes for each parent
    let mut children: HashMap<i32, Vec<i32>> = HashMap::new();
    
    // Populate the children map based on the memo array
    for (child_index, &parent_index) in memo.iter().enumerate() {
        if parent_index != -1 {
            children.entry(parent_index).or_insert_with(Vec::new).push(child_index as i32);
        }
    }

    // Function to compute the value for a node and its children
    fn compute_node_value(node: i32, m: &Vec<i32>, children: &HashMap<i32, Vec<i32>>) -> i64 {
        let mut total_value = m[node as usize] as i64; // Start with the node's own value
        let mut child_values = Vec::new();

        // Compute values for all children
        if let Some(child_nodes) = children.get(&node) {
            for &child in child_nodes {
                let child_value = compute_node_value(child, m, children);
                child_values.push(child_value);
            }
        }

        // Sort child values in descending order to apply weights
        child_values.sort_unstable_by(|a, b| b.cmp(a));

        // Apply weights based on the order of processing
        for (i, &value) in child_values.iter().enumerate() {
            total_value += value * (i as i64 + 1); // Weighting by position (1-based)
        }

        total_value
    }

    let mut total_aggregate_value = 0;

    // Process each root node (where memo[i] == -1)
    for i in 0..n {
        if memo[i] == -1 {
            total_aggregate_value += compute_node_value(i as i32, &m, &children);
        }
    }

    total_aggregate_value
}

fn main() {
    // Example usage
    let n = 5;
    let m = vec![1, 2, 3, 4, 5];
    let memo = vec![-1, 0, 0, 1, 1]; // Example parent-child relationships
    let result = compute_total_value(n, m, memo);
    println!("{}", result); // Output the computed total value
}
