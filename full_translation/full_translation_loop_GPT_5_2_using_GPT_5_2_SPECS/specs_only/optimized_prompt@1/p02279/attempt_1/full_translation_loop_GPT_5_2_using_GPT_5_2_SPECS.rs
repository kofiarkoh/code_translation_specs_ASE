use std::io::{self, Read};

fn set_depth(
    u: usize,
    d: i32,
    children: &Vec<Vec<usize>>,
    depth: &mut Vec<i32>,
) {
    depth[u] = d;
    for &v in &children[u] {
        set_depth(v, d + 1, children, depth);
    }
}

fn main() {
    // Read entire stdin as whitespace-separated tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(t) => t.parse().unwrap(),
        None => return,
    };

    // Sentinel for "no parent" is -1 (matches typical C implementations and spec's sentinel notion).
    let mut parent: Vec<i32> = vec![-1; n];
    let mut children: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Incorporate input: establish ordered child lists and last parent assignment.
    for _ in 0..n {
        let id: usize = it.next().unwrap().parse().unwrap();
        let k: usize = it.next().unwrap().parse().unwrap();
        let mut ch: Vec<usize> = Vec::with_capacity(k);
        for _ in 0..k {
            let c: usize = it.next().unwrap().parse().unwrap();
            ch.push(c);
            parent[c] = id as i32; // last assignment wins
        }
        children[id] = ch; // last assignment wins for the node's own children list
    }

    // Select root: among nodes with no parent, choose the one with greatest identifier.
    // If none exist (precondition violation), root remains 0 (narrowest reasonable fallback).
    let mut root: usize = 0;
    for i in 0..n {
        if parent[i] == -1 {
            root = i;
        }
    }

    // Depth values: only guaranteed meaningful for nodes reachable from chosen root.
    // Initialize to 0 to keep code safe; spec allows undefined values for unreachable nodes.
    let mut depth: Vec<i32> = vec![0; n];
    if n > 0 {
        set_depth(root, 0, &children, &mut depth);
    }

    // Emit per-node summaries.
    for i in 0..n {
        let kind = if parent[i] == -1 {
            "root"
        } else if children[i].is_empty() {
            "leaf"
        } else {
            "internal node"
        };

        print!("node {}: parent = {}, depth = {}, {}, [", i, parent[i], depth[i], kind);
        for (j, &c) in children[i].iter().enumerate() {
            if j > 0 {
                print!(", ");
            }
            print!("{}", c);
        }
        println!("]");
    }
}
