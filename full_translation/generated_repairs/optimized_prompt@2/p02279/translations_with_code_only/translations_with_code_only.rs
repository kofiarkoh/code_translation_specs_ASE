use std::io::{self, BufRead};

#[derive(Clone)]
struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

const MAX_NODES: usize = 100_000;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut tree: Vec<Node> = vec![Node { parent: -1, left: -1, right: -1 }; n];
    let mut depth: Vec<i32> = vec![0; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let id: usize = parts.next().unwrap().parse().unwrap();
        let num: usize = parts.next().unwrap().parse().unwrap();
        
        let mut last_child = -1; // To track the last inserted node
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

    let mut root = 0;
    for i in 0..n {
        if tree[i].parent == -1 {
            root = i as i32;
            break;
        }
    }

    get_depth(root, 0, &tree, &mut depth);

    for i in 0..n {
        print_tree(i, &tree, &depth);
    }
}

fn get_depth(r: i32, d: i32, tree: &[Node], depth: &mut Vec<i32>) {
    depth[r as usize] = d;
    if tree[r as usize].left != -1 {
        get_depth(tree[r as usize].left, d + 1, tree, depth);
    }
    if tree[r as usize].right != -1 {
        get_depth(tree[r as usize].right, d, tree, depth);
    }
}

fn print_tree(i: usize, tree: &[Node], depth: &[i32]) {
    let mut j = 0;
    print!("node {}: parent = {}, depth = {}, ", i, tree[i].parent, depth[i]);
    if tree[i].parent == -1 {
        print!("root, [");
    } else if tree[i].left == -1 {
        print!("leaf, [");
    } else {
        print!("internal node, [");
    }
    
    let mut m = tree[i].left;
    while m != -1 {
        if j != 0 {
            print!(", ");
        }
        print!("{}", m);
        m = tree[m as usize].right;
        j += 1;
    }
    println!("]");
}
