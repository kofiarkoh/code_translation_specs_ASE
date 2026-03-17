use std::io::{self, BufRead};

const MAX_NODES: usize = 100_000;

#[derive(Debug)]
struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of nodes
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize tree and depth arrays
    let mut tree: Vec<Node> = vec![Node { parent: -1, left: -1, right: -1 }; MAX_NODES];
    let mut depth: Vec<i32> = vec![-1; MAX_NODES];

    // Read each node's information
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace().map(|s| s.parse().unwrap()).collect();
        
        let id = parts[0];
        let num = parts[1];
        let children = &parts[2..];

        // Set parent for each child and store the children in the tree
        for (i, &child) in children.iter().enumerate() {
            if i == 0 {
                tree[id as usize].left = child;
            } else if i == 1 {
                tree[id as usize].right = child;
            }
            tree[child as usize].parent = id;
        }
    }

    // Calculate depth for each node
    for i in 0..n {
        calculate_depth(&tree, &mut depth, i as i32);
    }

    // Output the tree structure
    for i in 0..n {
        let node = &tree[i];
        let node_depth = depth[i];
        let children_ids: Vec<i32> = vec![node.left, node.right].into_iter().filter(|&x| x != -1).collect();
        println!(
            "node {}: parent = {}, depth = {}, node_type, [{}]",
            i, node.parent, node_depth, children_ids.iter().map(|x| x.to_string()).collect::<Vec<_>>().join(", ")
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
