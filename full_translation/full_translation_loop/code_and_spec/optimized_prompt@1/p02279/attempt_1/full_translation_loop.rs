use std::io::{self, BufRead};

#[derive(Debug)]
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
    let mut depths: Vec<i32> = vec![-1; n];

    // Read the nodes and their children
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let id: usize = parts.next().unwrap().parse().unwrap();
        let num: usize = parts.next().unwrap().parse().unwrap();

        let mut last_child = -1;
        for (j, child_str) in parts.enumerate() {
            let child: i32 = child_str.parse().unwrap();
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
    get_depths(root as i32, 0, &tree, &mut depths);

    // Print the tree
    for i in 0..n {
        print_tree(i, &tree, &depths);
    }
}

fn get_depths(node: i32, depth: i32, tree: &[Node], depths: &mut Vec<i32>) {
    depths[node as usize] = depth;
    if tree[node as usize].right != -1 {
        get_depths(tree[node as usize].right, depth, tree, depths);
    }
    if tree[node as usize].left != -1 {
        get_depths(tree[node as usize].left, depth + 1, tree, depths);
    }
}

fn print_tree(i: usize, tree: &[Node], depths: &[i32]) {
    let mut children = Vec::new();
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
             depths[i], 
             node_type, 
             children_str);
}
