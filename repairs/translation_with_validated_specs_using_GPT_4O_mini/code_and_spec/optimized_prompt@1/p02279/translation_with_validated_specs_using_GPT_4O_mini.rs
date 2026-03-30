use std::io::{self, BufRead};

#[derive(Debug, Clone)] // Added Clone to satisfy the trait bound
struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of nodes
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut tree: Vec<Node> = vec![Node { parent: -1, left: -1, right: -1 }; n];
    let mut depth: Vec<i32> = vec![0; n];

    // Construct the tree
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let id: usize = parts.next().unwrap().parse().unwrap();
        let num: usize = parts.next().unwrap().parse().unwrap();

        let mut last_child = -1;
        for j in 0..num {
            let child: i32 = parts.next().unwrap().parse().unwrap();
            if j == 0 {
                tree[id].left = child;
            } else {
                tree[last_child as usize].right = child;
            }
            tree[child as usize].parent = id as i32;
            last_child = child;
        }
    }

    // Find the root node
    let root = tree.iter().position(|node| node.parent == -1).unwrap();

    // Calculate depths
    get_depth(root, 0, &tree, &mut depth);

    // Print the tree
    for i in 0..n {
        print_tree(i, &tree, &depth);
    }
}

fn get_depth(node: usize, d: i32, tree: &[Node], depth: &mut Vec<i32>) {
    depth[node] = d;
    if tree[node].right != -1 {
        get_depth(tree[node].right as usize, d, tree, depth);
    }
    if tree[node].left != -1 {
        get_depth(tree[node].left as usize, d + 1, tree, depth);
    }
}

fn print_tree(i: usize, tree: &[Node], depth: &[i32]) {
    let mut children = vec![];
    let mut m = tree[i].left;

    while m != -1 {
        children.push(m);
        m = tree[m as usize].right;
    }

    let node_type = if tree[i].parent == -1 {
        "root"
    } else if tree[i].left == -1 {
        "leaf"
    } else {
        "internal node"
    };

    let children_str = children.iter()
        .map(|&child| child.to_string())
        .collect::<Vec<String>>()
        .join(", ");

    println!("node {}: parent = {}, depth = {}, {}, [{}]", 
             i, 
             tree[i].parent, 
             depth[i], 
             node_type, 
             children_str);
}
