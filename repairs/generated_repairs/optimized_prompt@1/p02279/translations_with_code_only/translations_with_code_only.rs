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
    let mut dipth: Vec<i32> = vec![0; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let id: usize = parts.next().unwrap().parse().unwrap();
        let num: usize = parts.next().unwrap().parse().unwrap();
        
        let mut l = -1; // To track the last inserted node
        for j in 0..num {
            let p: i32 = parts.next().unwrap().parse().unwrap();
            if j == 0 {
                tree[id].left = p;
            } else {
                tree[l as usize].right = p;
            }
            tree[p as usize].parent = id as i32;
            l = p;
        }
    }

    let mut root = 0;
    for i in 0..n {
        if tree[i].parent == -1 {
            root = i as i32;
            break;
        }
    }

    getdip(root, 0, &tree, &mut dipth);

    for i in 0..n {
        printtree(i, &tree, &dipth);
    }
}

fn getdip(r: i32, d: i32, tree: &[Node], dipth: &mut Vec<i32>) {
    dipth[r as usize] = d;
    if tree[r as usize].right != -1 {
        getdip(tree[r as usize].right, d, tree, dipth);
    }
    if tree[r as usize].left != -1 {
        getdip(tree[r as usize].left, d + 1, tree, dipth);
    }
}

fn printtree(i: usize, tree: &[Node], dipth: &[i32]) {
    let mut j = 0;
    print!("node {}: parent = {}, depth = {}, ", i, tree[i].parent, dipth[i]);
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
