use std::io::{self, Read};

#[derive(Clone)]
struct Edge {
    a: usize,
    b: usize,
    d: i32,
}

fn main() {
    // Fast parse
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            None => break,
            Some(x) => x.parse::<usize>().unwrap(),
        };
        let m = it.next().unwrap().parse::<usize>().unwrap();
        if n == 0 {
            break;
        }

        let mut edges: Vec<Edge> = Vec::with_capacity(m);
        // adjacency list: for each vertex, list of incident edge indices
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n + 1];

        for idx in 0..m {
            let a = it.next().unwrap().parse::<usize>().unwrap();
            let b = it.next().unwrap().parse::<usize>().unwrap();
            let d = it.next().unwrap().parse::<i32>().unwrap();
            edges.push(Edge { a, b, d });
            if a <= n && b <= n {
                adj[a].push(idx);
                adj[b].push(idx);
            } else {
                // Spec says labels are intended to be in-range; no explicit error handling.
                // Still, keep structure consistent to avoid panics.
            }
        }

        // Best answer tracking (mirrors C's fixed storage assumptions)
        let mut best_len: i32 = 0;
        let mut best_num: usize = 0; // number of vertices in best trail
        let mut best_ans = [0usize; 12];

        let mut tmp = [0usize; 12];
        let mut used = vec![false; m];

        // DFS exploring trails without reusing edges.
        // k = current number of vertices in tmp (next write index)
        fn dfs(
            k: usize,
            cur: usize,
            w: i32,
            edges: &[Edge],
            adj: &[Vec<usize>],
            used: &mut [bool],
            tmp: &mut [usize; 12],
            best_len: &mut i32,
            best_num: &mut usize,
            best_ans: &mut [usize; 12],
        ) {
            // Update best at every node (as in C: update after exploring all extensions,
            // but equivalent for max since w is fixed for this state).
            if w > *best_len {
                *best_len = w;
                *best_num = k;
                best_ans[..k].copy_from_slice(&tmp[..k]);
            }

            // Respect the spec's fixed storage: at most 12 vertices.
            if k >= 12 {
                return;
            }

            for &ei in &adj[cur] {
                if used[ei] {
                    continue;
                }
                let e = &edges[ei];
                let next = if e.a == cur { e.b } else { e.a };
                used[ei] = true;
                tmp[k] = next;
                dfs(
                    k + 1,
                    next,
                    w + e.d,
                    edges,
                    adj,
                    used,
                    tmp,
                    best_len,
                    best_num,
                    best_ans,
                );
                used[ei] = false;
            }
        }

        // Start from each directed use of each edge, matching the C code's enumeration.
        for a in 1..=n {
            for &ei in &adj[a] {
                if used[ei] {
                    continue;
                }
                let e = &edges[ei];
                let b = if e.a == a { e.b } else { e.a };

                used[ei] = true;
                tmp[0] = a;
                tmp[1] = b;
                dfs(
                    2,
                    b,
                    e.d,
                    &edges,
                    &adj,
                    &mut used,
                    &mut tmp,
                    &mut best_len,
                    &mut best_num,
                    &mut best_ans,
                );
                used[ei] = false;
            }
        }

        out.push_str(&format!("{best_len}\n"));

        // The original C prints ans[0] unconditionally; under intended inputs there is at least one edge.
        // To avoid panics on degenerate cases, fall back to printing "1" if no trail was found.
        if best_num == 0 {
            out.push_str("1\n");
        } else {
            out.push_str(&format!("{}", best_ans[0]));
            for i in 1..best_num {
                out.push_str(&format!(" {}", best_ans[i]));
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
