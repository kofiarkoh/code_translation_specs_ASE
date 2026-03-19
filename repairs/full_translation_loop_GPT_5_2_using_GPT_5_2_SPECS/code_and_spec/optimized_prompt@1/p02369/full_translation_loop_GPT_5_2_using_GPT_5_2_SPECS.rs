use std::io::{self, Read};

fn is_cyclic(adj: &[Vec<usize>]) -> i32 {
    let v = adj.len();
    let mut indeg = vec![0i32; v];

    // Compute indegree of each vertex.
    for edges in adj.iter() {
        for &tgt in edges.iter() {
            indeg[tgt] += 1;
        }
    }

    // This mirrors the C code's behavior: repeatedly find a vertex with indegree 0,
    // "remove" it (mark as -1), and decrement indegrees of its outgoing neighbors.
    // If at some iteration no indegree-0 vertex exists, a cycle is present.
    for _ in 0..v {
        let mut flag = true;
        for i in 0..v {
            if indeg[i] == 0 {
                indeg[i] = -1;
                flag = false;
                for &tgt in adj[i].iter() {
                    indeg[tgt] -= 1;
                }
                break;
            }
        }
        if flag {
            return 1;
        }
    }
    0
}

fn main() {
    // Read all input (no explicit error handling, per spec).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let v: usize = it.next().unwrap().parse().unwrap();
    let e: usize = it.next().unwrap().parse().unwrap();

    // Build adjacency list; edge "cost" is irrelevant and omitted.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); v];
    for _ in 0..e {
        let s: usize = it.next().unwrap().parse().unwrap();
        let t: usize = it.next().unwrap().parse().unwrap();
        adj[s].push(t);
    }

    let res = is_cyclic(&adj);
    print!("{res}\n");
}
