use std::io::{self, BufRead};

const MAX_NODES: usize = 100_000;

#[derive(Debug, Clone)]
struct Node {
    parent: i32,
    children: Vec<i32>, // Changed to a vector to accommodate any number of children
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    assert!(n <= MAX_NODES, "Number of nodes exceeds maximum limit");

    // Initialize tree and depth arrays
    let mut tree: Vec<Node> = vec![Node { parent: -1, children: Vec::new() }; n];
    let mut depth: Vec<i32> = vec![-1; n];

    // Read each node's information
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let id = parts[0] as usize;
        let num_children = parts[1];
        let children_ids = &parts[2..];

        // Set parent for children and store the node's children
        for &child_id in children_ids.iter() {
            tree[id].children.push(child_id);
            tree[child_id as usize].parent = id as i32;
        }
    }

    // Calculate depths
    for i in 0..n {
        calculate_depth(&tree, &mut depth, i as i32);
    }

    // Output the tree structure
    for i in 0..n {
        let node = &tree[i];
        let children_ids: Vec<i32> = node.children.clone(); // Use the children vector
        let node_type = if children_ids.is_empty() {
            "leaf"
        } else if children_ids.len() == 1 {
            "internal node"
        } else {
            "root"
        };
        println!(
            "node {}: parent = {}, depth = {}, {}, [{}]",
            i,
            node.parent,
            depth[i],
            node_type,
            children_ids.iter().map(|id| id.to_string()).collect::<Vec<_>>().join(", ")
        );
    }
}

// Recursive function to calculate the depth of each node
fn calculate_depth(tree: &[Node], depth: &mut Vec<i32>, node_id: i32) {
    if depth[node_id as usize] != -1 {
        return; // Already calculated
    }
    let parent_id = tree[node_id as usize].parent;
    if parent_id != -1 {
        calculate_depth(tree, depth, parent_id);
        depth[node_id as usize] = depth[parent_id as usize] + 1;
    } else {
        depth[node_id as usize] = 0; // Root node
    }
}
