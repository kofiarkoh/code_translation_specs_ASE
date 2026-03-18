use std::io::{self, Read};

const INIT_CAP: usize = 10;
const MAX_N: usize = 5000;

#[derive(Clone)]
struct Rec {
    name: String,
    cost: i32,
}

fn search_idx(ing: &[Rec], name: &str) -> Option<usize> {
    ing.iter().position(|r| r.name == name)
}

fn dfs(i: usize, adj: &[Vec<usize>], ing: &[Rec], visited: &mut [bool], cur_min: &mut i32) {
    visited[i] = true;
    *cur_min = (*cur_min).min(ing[i].cost);

    for &to in &adj[i] {
        if !visited[to] {
            dfs(to, adj, ing, visited, cur_min);
        }
    }
}

fn search_min_cost(i: usize, adj: &[Vec<usize>], ing: &mut [Rec], visited: &mut [bool]) -> i32 {
    let mut cur_min: i32 = 100000;
    visited.fill(false);
    dfs(i, adj, ing, visited, &mut cur_min);
    ing[i].cost = cur_min;
    cur_min
}

fn main() {
    // Read all input (scanf-like whitespace tokenization).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    // Spec says max size 5000; C code uses fixed arrays of 5000.
    // We keep vectors but still respect the implied bound.
    assert!(n <= MAX_N);

    let mut ing: Vec<Rec> = Vec::with_capacity(n);
    for _ in 0..n {
        let name = it.next().unwrap().to_string();
        let cost: i32 = it.next().unwrap().parse().unwrap();
        ing.push(Rec { name, cost });
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency lists; C code pre-allocates INIT_CAP and grows via realloc.
    // Vec handles growth automatically; we reserve INIT_CAP to mirror intent.
    let mut adj: Vec<Vec<usize>> = (0..n).map(|_| Vec::with_capacity(INIT_CAP)).collect();

    for _ in 0..m {
        let s = it.next().unwrap();
        let t = it.next().unwrap();

        // SPEC: If an ingredient name is not found during connection establishment,
        // do not create a connection for that ingredient.
        // Note: This differs from the C code, which would add using index==n (out of bounds).
        if let (Some(idx_s), Some(idx_t)) = (search_idx(&ing, s), search_idx(&ing, t)) {
            adj[idx_s].push(idx_t);
            adj[idx_t].push(idx_s);
        }
    }

    let mut visited = vec![false; n];
    let mut ans: i64 = 0;
    for i in 0..n {
        ans += search_min_cost(i, &adj, &mut ing, &mut visited) as i64;
    }

    print!("{ans}\n");
}
