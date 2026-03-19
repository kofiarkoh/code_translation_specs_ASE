use std::io::{self, Read};

fn max_i32(a: i32, b: i32) -> i32 {
    if a < b { b } else { a }
}

struct Graph {
    // adjacency list as linked lists via arrays (to match C semantics closely)
    ta: Vec<i32>, // head index per vertex, -1 means empty
    nt: Vec<i32>, // next edge index, -1 means end
    fr: Vec<usize>,
    to: Vec<usize>,
    co: Vec<i32>,
    cdist: Vec<i32>, // in C used as accumulated distance in dfs; spec says "minimum", but code uses it as distance
    vis: Vec<bool>,
}

impl Graph {
    fn new(v: usize, e2: usize) -> Self {
        Self {
            ta: vec![-1; v],
            nt: vec![-1; e2],
            fr: vec![0; e2],
            to: vec![0; e2],
            co: vec![0; e2],
            cdist: vec![1_000_000_000; v],
            vis: vec![false; v],
        }
    }

    fn dfs(&mut self, s: usize, d: i32) -> i32 {
        let mut maxv = d;
        let mut i = self.ta[s];
        while i != -1 {
            let ei = i as usize;
            let t = self.to[ei];
            if !self.vis[t] {
                self.vis[t] = true;
                self.cdist[t] = self.cdist[s] + self.co[ei];
                maxv = max_i32(maxv, self.dfs(t, d + self.co[ei]));
            }
            i = self.nt[ei];
        }
        maxv
    }
}

/// Computes the maximum cost of a path from the starting vertex to any reachable vertex.
/// Note: The provided C code actually computes the diameter of an undirected weighted tree
/// (it adds edges in both directions and runs two DFS passes). This is a mismatch with the
/// SPEC's "directed graph" wording; we follow the C code's behavior as the only concrete
/// executable definition.
fn tyokkei(v: usize, e: usize, a: &[usize], b: &[usize], c: &[i32]) -> i32 {
    let mut gph = Graph::new(v, 2 * e);

    // build undirected edges (two directed edges per input edge)
    for i in 0..e {
        let ai = a[i];
        let bi = b[i];
        let ci = c[i];

        // edge i: ai -> bi
        gph.nt[i] = gph.ta[ai];
        gph.fr[i] = ai;
        gph.to[i] = bi;
        gph.co[i] = ci;
        gph.ta[ai] = i as i32;

        // edge i+e: bi -> ai
        let j = i + e;
        gph.nt[j] = gph.ta[bi];
        gph.fr[j] = bi;
        gph.to[j] = ai;
        gph.co[j] = ci;
        gph.ta[bi] = j as i32;
    }

    // first DFS from 0 to find farthest distance
    gph.cdist.fill(1_000_000_000);
    gph.vis.fill(false);
    gph.cdist[0] = 0;
    gph.vis[0] = true;
    let max1 = gph.dfs(0, 0);

    // pick a vertex g with distance == max1 (last such vertex in index order, as in C)
    let mut g = 0usize;
    for i in 0..v {
        if gph.cdist[i] == max1 {
            g = i;
        }
    }

    // second DFS from g to get diameter length
    gph.cdist.fill(1_000_000_000);
    gph.vis.fill(false);
    gph.cdist[g] = 0;
    gph.vis[g] = true;
    gph.dfs(g, 0)
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<usize> = Vec::with_capacity(n.saturating_sub(1));
    let mut b: Vec<usize> = Vec::with_capacity(n.saturating_sub(1));
    let mut c: Vec<i32> = Vec::with_capacity(n.saturating_sub(1));

    for _ in 0..n.saturating_sub(1) {
        let ai: usize = it.next().unwrap().parse().unwrap();
        let bi: usize = it.next().unwrap().parse().unwrap();
        let ci: i32 = it.next().unwrap().parse().unwrap();
        a.push(ai);
        b.push(bi);
        c.push(ci);
    }

    let ans = tyokkei(n, n.saturating_sub(1), &a, &b, &c);
    print!("{ans}\n");
}
