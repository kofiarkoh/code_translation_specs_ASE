use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };
    let m: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut gr: Vec<Vec<usize>> = vec![Vec::new(); n];

    // Input indexing scheme is not specified; samples may be 1-based.
    // Narrowest robust interpretation: accept both 0-based and 1-based by detecting max endpoint.
    let mut raw: Vec<(usize, usize)> = Vec::with_capacity(m);
    let mut max_ep: usize = 0;
    for _ in 0..m {
        let u: usize = it.next().unwrap().parse().unwrap();
        let v: usize = it.next().unwrap().parse().unwrap();
        max_ep = max_ep.max(u).max(v);
        raw.push((u, v));
    }
    let one_based = max_ep == n; // if endpoints reach exactly n, likely 1..=n
    for (u0, v0) in raw {
        let u = if one_based { u0 - 1 } else { u0 };
        let v = if one_based { v0 - 1 } else { v0 };
        edges.push(Edge { u, v });
        g[u].push(v);
        gr[v].push(u);
    }

    // 1) SCC decomposition (Kosaraju, iterative)
    let mut vis = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for i in 0..n {
        if vis[i] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = vec![(i, 0)];
        vis[i] = true;
        while let Some((v, idx)) = stack.pop() {
            if idx < g[v].len() {
                stack.push((v, idx + 1));
                let to = g[v][idx];
                if !vis[to] {
                    vis[to] = true;
                    stack.push((to, 0));
                }
            } else {
                order.push(v);
            }
        }
    }

    let mut comp = vec![usize::MAX; n];
    let mut comp_cnt = 0usize;

    for &v in order.iter().rev() {
        if comp[v] != usize::MAX {
            continue;
        }
        let mut dq = VecDeque::new();
        dq.push_back(v);
        comp[v] = comp_cnt;
        while let Some(x) = dq.pop_front() {
            for &p in &gr[x] {
                if comp[p] == usize::MAX {
                    comp[p] = comp_cnt;
                    dq.push_back(p);
                }
            }
        }
        comp_cnt += 1;
    }

    let mut comp_vertices: Vec<Vec<usize>> = vec![Vec::new(); comp_cnt];
    for v in 0..n {
        comp_vertices[comp[v]].push(v);
    }

    // 2) Component graph multiplicity and adjacency
    let mut pair_count: Vec<HashMap<usize, usize>> = vec![HashMap::new(); comp_cnt];
    for e in &edges {
        let cu = comp[e.u];
        let cv = comp[e.v];
        if cu == cv {
            continue;
        }
        *pair_count[cu].entry(cv).or_insert(0) += 1;
    }

    let mut comp_adj: Vec<Vec<usize>> = vec![Vec::new(); comp_cnt];
    for cu in 0..comp_cnt {
        comp_adj[cu] = pair_count[cu].keys().copied().collect();
    }

    // 3) Bypassability in component graph: path length >= 2
    let mut dist: Vec<Vec<i32>> = vec![vec![-1; comp_cnt]; comp_cnt];
    for scc in 0..comp_cnt {
        let mut dq = VecDeque::new();
        dist[scc][scc] = 0;
        dq.push_back(scc);
        while let Some(x) = dq.pop_front() {
            let dx = dist[scc][x];
            for &to in &comp_adj[x] {
                if dist[scc][to] == -1 {
                    dist[scc][to] = dx + 1;
                    dq.push_back(to);
                }
            }
        }
    }

    // 4) Intra-SCC alternative path avoiding the specific edge instance
    let mut intra_alt: Vec<bool> = vec![false; m];
    let mut out_edges: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // (to, edge_index)
    for (idx, e) in edges.iter().enumerate() {
        out_edges[e.u].push((e.v, idx));
    }

    for c in 0..comp_cnt {
        let verts = &comp_vertices[c];
        for &u in verts {
            for &(v, ei) in &out_edges[u] {
                if comp[v] != c {
                    continue;
                }
                let mut q = VecDeque::new();
                let mut seen = vec![false; n];
                seen[u] = true;
                q.push_back(u);

                while let Some(x) = q.pop_front() {
                    for &(to, eidx) in &out_edges[x] {
                        if comp[to] != c {
                            continue;
                        }
                        if eidx == ei {
                            continue;
                        }
                        if !seen[to] {
                            seen[to] = true;
                            q.push_back(to);
                        }
                    }
                }
                intra_alt[ei] = seen[v];
            }
        }
    }

    // 5) Classify edges
    let mut out = String::new();
    for (i, e) in edges.iter().enumerate() {
        let cu = comp[e.u];
        let cv = comp[e.v];

        let mut diff = false;

        if cu != cv {
            if let Some(&cnt) = pair_count[cu].get(&cv) {
                if cnt >= 2 {
                    diff = true; // (a)
                }
            }
            let d = dist[cu][cv];
            if d >= 2 {
                diff = true; // (b)
            }
        } else {
            if intra_alt[i] {
                diff = true; // (c)
            }
        }

        if diff {
            out.push_str("diff\n");
        } else {
            out.push_str("same\n");
        }
    }

    print!("{out}");
}
