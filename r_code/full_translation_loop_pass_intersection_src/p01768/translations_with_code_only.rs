use std::io::{self, Read};

const INIT_CAP: usize = 10;

#[derive(Clone)]
struct Rec {
    name: String,
    cost: i32,
}

fn dfs(i: usize, adj: &Vec<Vec<usize>>, ing: &Vec<Rec>, visited: &mut [u8], cur_min: &mut i32) {
    visited[i] = 1;
    *cur_min = (*cur_min).min(ing[i].cost);

    for &to in &adj[i] {
        if visited[to] == 0 {
            dfs(to, adj, ing, visited, cur_min);
        }
    }
}

fn search_min_cost(i: usize, adj: &Vec<Vec<usize>>, ing: &mut Vec<Rec>) -> i32 {
    let mut cur_min: i32 = 100000;
    let mut visited = vec![0u8; ing.len()];
    dfs(i, adj, ing, &mut visited, &mut cur_min);
    ing[i].cost = cur_min;
    cur_min
}

fn main() {
    // Read entire stdin and tokenize like scanf would (whitespace-separated).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut ing: Vec<Rec> = Vec::with_capacity(n);
    for _ in 0..n {
        let name = it.next().unwrap().to_string();
        let cost: i32 = it.next().unwrap().parse().unwrap();
        ing.push(Rec { name, cost });
    }

    let m: usize = it.next().unwrap().parse().unwrap();

    // Adjacency lists; C code uses manual capacity growth, but Vec already does that.
    // We pre-reserve INIT_CAP to mirror the initial allocation behavior.
    let mut adj: Vec<Vec<usize>> = (0..n).map(|_| Vec::with_capacity(INIT_CAP)).collect();

    // Linear search by name, matching the C code's strcmp loop.
    let mut search_idx = |s: &str, ing: &Vec<Rec>| -> usize {
        let mut i = 0usize;
        while i < ing.len() {
            if ing[i].name == s {
                break;
            }
            i += 1;
        }
        i
    };

    for _ in 0..m {
        let s = it.next().unwrap();
        let t = it.next().unwrap();
        let idx_s = search_idx(s, &ing);
        let idx_t = search_idx(t, &ing);

        // The C code assumes indices are valid; keep the same assumption.
        adj[idx_s].push(idx_t);
        adj[idx_t].push(idx_s);
    }

    // Sum in i32 to match C's `int ans;` (overflow behavior in C is UB, but we keep i32).
    let mut ans: i32 = 0;
    for i in 0..n {
        ans += search_min_cost(i, &adj, &mut ing);
    }

    print!("{ans}\n");
}
