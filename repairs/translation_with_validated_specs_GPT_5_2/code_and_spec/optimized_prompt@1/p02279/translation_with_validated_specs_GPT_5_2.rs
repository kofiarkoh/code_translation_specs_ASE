use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Node {
    parent: i32,
    left: i32,
    right: i32,
}

fn main() {
    // Read entire stdin, parse as whitespace-separated integers (like scanf).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut tree = vec![
        Node {
            parent: -1,
            left: -1,
            right: -1
        };
        n
    ];

    // Depth values are only specified for nodes reachable from the chosen root.
    // The C code uses a global array that is zero-initialized; we mirror that.
    let mut depth = vec![0i32; n];

    for _ in 0..n {
        let id: usize = it.next().unwrap().parse().unwrap();
        let num: usize = it.next().unwrap().parse().unwrap();

        let mut last_child: i32 = -1;
        for j in 0..num {
            let p: i32 = it.next().unwrap().parse().unwrap();

            if j == 0 {
                tree[id].left = p;
            } else {
                // Link as right sibling of previous child.
                let l = last_child as usize;
                tree[l].right = p;
            }
            // Set/overwrite parent.
            tree[p as usize].parent = id as i32;
            last_child = p;
        }
    }

    // Root identification: last index with parent == -1.
    let mut root: usize = 0;
    for i in 0..n {
        if tree[i].parent == -1 {
            root = i;
        }
    }

    // Depth computation: recursive traversal visiting right sibling first (same depth),
    // then left child (depth + 1).
    fn get_depth(r: usize, d: i32, tree: &[Node], depth: &mut [i32]) {
        depth[r] = d;
        let right = tree[r].right;
        if right != -1 {
            get_depth(right as usize, d, tree, depth);
        }
        let left = tree[r].left;
        if left != -1 {
            get_depth(left as usize, d + 1, tree, depth);
        }
    }
    get_depth(root, 0, &tree, &mut depth);

    // Print fixed-format summary for each node 0..n-1.
    let mut out = String::new();
    for i in 0..n {
        out.push_str(&format!(
            "node {}: parent = {}, depth = {}, ",
            i, tree[i].parent, depth[i]
        ));

        if tree[i].parent == -1 {
            out.push_str("root, [");
        } else if tree[i].left == -1 {
            out.push_str("leaf, [");
        } else {
            out.push_str("internal node, [");
        }

        let mut m = tree[i].left;
        let mut j = 0;
        while m != -1 {
            if j != 0 {
                out.push_str(", ");
            }
            out.push_str(&format!("{}", m));
            m = tree[m as usize].right;
            j += 1;
        }
        out.push_str("]\n");
    }

    print!("{}", out);
}
