use std::cmp::min;
use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    u: usize,
    v: usize,
}

fn tarjan_scc(n: usize, g: &[Vec<usize>]) -> Vec<usize> {
    // Iterative Tarjan is more code; recursive is fine for typical constraints.
    // If recursion depth is too large, this could overflow the stack, but the
    // specification provides no limits; we keep it simple and safe (no unsafe).
    fn dfs(
        v: usize,
        g: &[Vec<usize>],
        index: &mut i32,
        idx: &mut [i32],
        low: &mut [i32],
        onstack: &mut [bool],
        stack: &mut Vec<usize>,
        comp: &mut [usize],
        comp_cnt: &mut usize,
    ) {
        idx[v] = *index;
        low[v] = *index;
        *index += 1;
        stack.push(v);
        onstack[v] = true;

        for &to in &g[v] {
            if idx[to] == -1 {
                dfs(to, g, index, idx, low, onstack, stack, comp, comp_cnt);
                low[v] = min(low[v], low[to]);
            } else if onstack[to] {
                low[v] = min(low[v], idx[to]);
            }
        }

        if low[v] == idx[v] {
            loop {
                let w = stack.pop().unwrap();
                onstack[w] = false;
                comp[w] = *comp_cnt;
                if w == v {
                    break;
                }
            }
            *comp_cnt += 1;
        }
    }

    let mut idx = vec![-1i32; n];
    let mut low = vec![0i32; n];
    let mut onstack = vec![false; n];
    let mut stack = Vec::new();
    let mut comp = vec![0usize; n];
    let mut index = 0i32;
    let mut comp_cnt = 0usize;

    for v in 0..n {
        if idx[v] == -1 {
            dfs(
                v,
                g,
                &mut index,
                &mut idx,
                &mut low,
                &mut onstack,
                &mut stack,
                &mut comp,
                &mut comp_cnt,
            );
        }
    }
    comp
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut edges = Vec::with_capacity(m);
    let mut g = vec![Vec::<usize>::new(); n];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        // Input is assumed valid per specification; behavior undefined otherwise.
        let u = a - 1;
        let v = b - 1;
        edges.push(Edge { u, v });
        g[u].push(v);
    }

    // Interpretation consistent with the sample:
    // - "same" iff the edge is inside a strongly connected component (u and v mutually reachable).
    // - "diff" otherwise.
    let comp = tarjan_scc(n, &g);

    let mut out = String::new();
    for e in edges {
        if comp[e.u] == comp[e.v] {
            out.push_str("same\n");
        } else {
            out.push_str("diff\n");
        }
    }
    print!("{out}");
}
