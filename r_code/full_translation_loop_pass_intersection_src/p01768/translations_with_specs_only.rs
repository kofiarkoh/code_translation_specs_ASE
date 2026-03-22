use std::collections::HashMap;
use std::io::{self, Read};

const MAX_N: usize = 5000;
const CAP_MIN: i32 = 100000;

fn dfs(
    u: usize,
    adj: &Vec<Vec<usize>>,
    visited: &mut Vec<bool>,
    costs: &Vec<i32>,
    cur_min: &mut i32,
) {
    visited[u] = true;
    if costs[u] < *cur_min {
        *cur_min = costs[u];
    }
    for &v in &adj[u] {
        if !visited[v] {
            dfs(v, adj, visited, costs, cur_min);
        }
    }
}

fn main() {
    // No explicit error handling per spec; unwraps/panics are acceptable.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    // Spec precondition: n <= 5000. We do not enforce; behavior otherwise is unspecified.
    let _ = MAX_N; // document fixed maximum without enforcing.

    let mut name_to_idx: HashMap<String, usize> = HashMap::with_capacity(n * 2);
    let mut costs: Vec<i32> = vec![0; n];

    for i in 0..n {
        let name = it.next().unwrap().to_string();
        let cost: i32 = it.next().unwrap().parse().unwrap();
        // Spec: name length <= 10; we do not enforce.
        name_to_idx.insert(name, i);
        costs[i] = cost;
    }

    let m: usize = it.next().unwrap().parse().unwrap();
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let a = it.next().unwrap();
        let b = it.next().unwrap();

        // Spec: if unknown name appears, behavior is undefined (may access out-of-bounds).
        // We emulate "no explicit error handling" by unwrapping.
        let ia = *name_to_idx.get(a).unwrap();
        let ib = *name_to_idx.get(b).unwrap();

        adj[ia].push(ib);
        adj[ib].push(ia);
    }

    // Per spec, computation is "performing a full graph traversal for each item".
    // We do exactly that: for each i, run a DFS to find the component minimum, then
    // update all nodes in that component to that minimum (capped at 100000).
    let mut total: i64 = 0;

    for i in 0..n {
        let mut visited = vec![false; n];
        let mut cur_min: i32 = CAP_MIN; // Spec: component minimum is capped at 100000.
        dfs(i, &adj, &mut visited, &costs, &mut cur_min);

        // Update stored costs for all items in the component of i.
        for j in 0..n {
            if visited[j] {
                costs[j] = cur_min;
            }
        }

        total += cur_min as i64;
    }

    println!("{}", total);
}
