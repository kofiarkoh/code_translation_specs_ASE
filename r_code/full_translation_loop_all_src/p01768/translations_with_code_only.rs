use std::io::{self, Read};

const INIT_CAP: usize = 10;

#[derive(Clone)]
struct AdjList {
    to: Vec<usize>,
}

#[derive(Clone)]
struct Rec {
    name: String,
    cost: i32,
}

fn search_idx(ing: &[Rec], n: usize, s: &str) -> usize {
    // Mirrors C: linear search, returns n if not found (though inputs likely ensure found).
    for i in 0..n {
        if ing[i].name == s {
            return i;
        }
    }
    n
}

fn dfs(i: usize, adj: &[AdjList], ing: &[Rec], visited: &mut [bool], cur_min: &mut i32) {
    visited[i] = true;
    *cur_min = (*cur_min).min(ing[i].cost);
    for &to in &adj[i].to {
        if !visited[to] {
            dfs(to, adj, ing, visited, cur_min);
        }
    }
}

fn search_min_cost(i: usize, adj: &[AdjList], ing: &mut [Rec]) -> i32 {
    let mut cur_min: i32 = 100000;
    let mut visited = vec![false; ing.len()];
    dfs(i, adj, ing, &mut visited, &mut cur_min);
    ing[i].cost = cur_min; // matches C side-effect: ing[i].cost = min
    cur_min
}

fn main() {
    // Token-based input to emulate scanf("%d"/"%s") whitespace behavior.
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

    // init(n): create adjacency lists with initial capacity INIT_CAP
    let mut adj: Vec<AdjList> = (0..n)
        .map(|_| AdjList {
            to: Vec::with_capacity(INIT_CAP),
        })
        .collect();

    for _ in 0..m {
        let s = it.next().unwrap();
        let t = it.next().unwrap();

        let idx_s = search_idx(&ing, n, s);
        let idx_t = search_idx(&ing, n, t);

        // add undirected edges; assumes indices are valid as in C.
        if idx_s < n && idx_t < n {
            adj[idx_s].to.push(idx_t);
            adj[idx_t].to.push(idx_s);
        } else {
            // If a name is missing, C would index out of bounds (UB).
            // We avoid panicking by doing nothing; such inputs are not expected.
        }
    }

    // Sum of per-node component minimums, with the same mutation behavior as C.
    let mut ans: i32 = 0;
    for i in 0..n {
        ans = ans.wrapping_add(search_min_cost(i, &adj, &mut ing));
    }

    print!("{ans}\n");
}
