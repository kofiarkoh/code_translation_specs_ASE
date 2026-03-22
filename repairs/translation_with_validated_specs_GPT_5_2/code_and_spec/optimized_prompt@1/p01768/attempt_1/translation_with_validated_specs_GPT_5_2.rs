use std::io::{self, Read};

fn main() {
    // Read entire stdin, token-scan like scanf("%s"/"%d").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    // Fixed maximums exist in the C program (5000 items, name length 10),
    // but per spec there is no explicit error handling; we assume inputs satisfy preconditions.
    let mut names: Vec<String> = Vec::with_capacity(n);
    let mut costs: Vec<i32> = Vec::with_capacity(n);

    for _ in 0..n {
        let name = it.next().unwrap().to_string();
        let cost: i32 = it.next().unwrap().parse().unwrap();
        names.push(name);
        costs.push(cost);
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency list; C stores bidirectionally and grows dynamically.
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Linear search by name, matching C's searchIdx semantics (O(n) per lookup).
    let search_idx = |s: &str, names: &Vec<String>| -> usize {
        let mut i = 0usize;
        while i < names.len() {
            if names[i] == s {
                break;
            }
            i += 1;
        }
        i
    };

    for _ in 0..m {
        let s = it.next().unwrap();
        let t = it.next().unwrap();
        let idx_s = search_idx(s, &names);
        let idx_t = search_idx(t, &names);

        // Per spec, endpoints match exactly one provided name; otherwise behavior is undefined.
        adj[idx_s].push(idx_t);
        adj[idx_t].push(idx_s);
    }

    // DFS that computes the minimum cost in the reachable set from `start`,
    // using the *current* costs (which may have been mutated by prior iterations),
    // matching the C program's behavior and the spec's invariants.
    fn dfs(
        v: usize,
        adj: &Vec<Vec<usize>>,
        costs: &Vec<i32>,
        visited: &mut Vec<bool>,
        cur_min: &mut i32,
    ) {
        visited[v] = true;
        if costs[v] < *cur_min {
            *cur_min = costs[v];
        }
        for &to in &adj[v] {
            if !visited[to] {
                dfs(to, adj, costs, visited, cur_min);
            }
        }
    }

    let mut ans: i32 = 0;
    for i in 0..n {
        let mut visited = vec![false; n];
        let mut cur_min: i32 = 100_000; // matches C's initialization
        dfs(i, &adj, &costs, &mut visited, &mut cur_min);

        // Postcondition: each item's stored cost is replaced by its component minimum.
        costs[i] = cur_min;

        ans = ans.wrapping_add(cur_min); // C has undefined overflow; wrapping is a reasonable analogue.
    }

    print!("{ans}\n");
}
