use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    parent: i32,
    left: i32,  // first child
    right: i32, // next sibling
}

fn get_depth(r: usize, d: i32, tree: &Vec<Node>, depth: &mut Vec<i32>) {
    depth[r] = d;
    // Note: This follows the C code's traversal order:
    // visit next sibling at same depth, then first child at depth+1.
    let right = tree[r].right;
    if right != -1 {
        get_depth(right as usize, d, tree, depth);
    }
    let left = tree[r].left;
    if left != -1 {
        get_depth(left as usize, d + 1, tree, depth);
    }
}

fn print_tree(i: usize, tree: &Vec<Node>, depth: &Vec<i32>) {
    print!(
        "node {}: parent = {}, depth = {}, ",
        i, tree[i].parent, depth[i]
    );

    if tree[i].parent == -1 {
        print!("root, [");
    } else if tree[i].left == -1 {
        print!("leaf, [");
    } else {
        print!("internal node, [");
    }

    let mut m = tree[i].left;
    let mut j = 0;
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
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Initialize nodes: parent/left/right = -1
    let mut tree = vec![
        Node {
            parent: -1,
            left: -1,
            right: -1
        };
        n
    ];

    // Depth array (C code leaves it uninitialized for unreachable nodes; we default to 0).
    // Per spec, behavior is undefined for disconnected components; this is acceptable.
    let mut depth = vec![0i32; n];

    // Read node descriptions
    for _ in 0..n {
        let id: usize = it.next().unwrap().parse().unwrap();
        let num: usize = it.next().unwrap().parse().unwrap();
        let mut last_child: i32 = -1;

        for j in 0..num {
            let p: i32 = it.next().unwrap().parse().unwrap();
            if j == 0 {
                tree[id].left = p;
            } else {
                tree[last_child as usize].right = p;
            }
            tree[p as usize].parent = id as i32;
            last_child = p;
        }
    }

    // Choose root: last node with parent == -1 (i.e., greatest identifier among them)
    let mut root: usize = 0;
    for i in 0..n {
        if tree[i].parent == -1 {
            root = i;
        }
    }

    // Compute depths from chosen root
    get_depth(root, 0, &tree, &mut depth);

    // Print summaries for nodes 0..n-1
    for i in 0..n {
        print_tree(i, &tree, &depth);
    }
}
