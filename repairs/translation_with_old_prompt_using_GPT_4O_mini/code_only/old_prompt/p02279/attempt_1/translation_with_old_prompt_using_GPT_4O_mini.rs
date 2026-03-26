use std::io::{self, BufRead};

struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

fn getdip(tree: &mut [Node], dipth: &mut [i32], r: usize, d: i32) {
    dipth[r] = d;
    if tree[r].right != -1 {
        getdip(tree, dipth, tree[r].right as usize, d);
    }
    if tree[r].left != -1 {
        getdip(tree, dipth, tree[r].left as usize, d + 1);
    }
}

fn printtree(tree: &[Node], dipth: &[i32], i: usize) {
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

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();
    
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut tree = vec![Node { parent: -1, left: -1, right: -1 }; n];
    let mut dipth = vec![0; n];

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let id: usize = parts.next().unwrap().parse().unwrap();
        let num: usize = parts.next().unwrap().parse().unwrap();
        
        let mut l = -1;
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
            root = i;
            break;
        }
    }

    getdip(&mut tree, &mut dipth, root, 0);
    
    for i in 0..n {
        printtree(&tree, &dipth, i);
    }
}
