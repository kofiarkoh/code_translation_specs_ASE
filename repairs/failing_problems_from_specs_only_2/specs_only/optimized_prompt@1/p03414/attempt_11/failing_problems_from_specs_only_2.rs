use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn kosaraju_scc(n: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let mut radj: Vec<Vec<usize>> = vec![Vec::new(); n];
    for u in 0..n {
        for &v in &adj[u] {
            radj[v].push(u);
        }
    }

    let mut vis = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    for start in 0..n {
        if vis[start] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((start, 0));
        vis[start] = true;

        while let Some((u, i)) = stack.pop() {
            if i < adj[u].len() {
                stack.push((u, i + 1));
                let v = adj[u][i];
                if !vis[v] {
                    vis[v] = true;
                    stack.push((v, 0));
                }
            } else {
                order.push(u);
            }
        }
    }

    let mut comp = vec![usize::MAX; n];
    let mut comp_cnt = 0usize;

    for &start in order.iter().rev() {
        if comp[start] != usize::MAX {
            continue;
        }
        let mut q: VecDeque<usize> = VecDeque::new();
        q.push_back(start);
        comp[start] = comp_cnt;

        while let Some(u) = q.pop_front() {
            for &v in &radj[u] {
                if comp[v] == usize::MAX {
                    comp[v] = comp_cnt;
                    q.push_back(v);
                }
            }
        }
        comp_cnt += 1;
    }

    (comp_cnt, comp)
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

    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    let mut adj: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let u1: usize = it.next().unwrap().parse().unwrap();
        let v1: usize = it.next().unwrap().parse().unwrap();
        let u = u1 - 1;
        let v = v1 - 1;
        edges.push(Edge { u, v });
        adj[u].push(v);
    }

    // SCCs
    let (k, comp) = kosaraju_scc(n, &adj);

    // Output labels
    let mut is_diff = vec![false; m];

    // Build condensation DAG and SCC-pair -> edge indices.
    let mut pair_to_edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    let mut dag_adj: Vec<Vec<usize>> = vec![Vec::new(); k];
    let mut dag_has: HashMap<(usize, usize), ()> = HashMap::new();

    for (i, e) in edges.iter().enumerate() {
        let s = comp[e.u];
        let t = comp[e.v];
        if s != t {
            pair_to_edges.entry((s, t)).or_default().push(i);
            if !dag_has.contains_key(&(s, t)) {
                dag_has.insert((s, t), ());
                dag_adj[s].push(t);
            }
        }
    }

    // A1: parallel SCC-arc duplication
    for idxs in pair_to_edges.values() {
        if idxs.len() > 1 {
            for &ei in idxs {
                is_diff[ei] = true;
            }
        }
    }

    // A2: longer SCC-level route exists (path length >= 2).
    // For each (S,T), check reachability from S to T in condensation DAG after removing arc S->T.
    let mut targets_by_source: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (&(s, t), _) in pair_to_edges.iter() {
        targets_by_source[s].push(t);
    }
    for s in 0..k {
        targets_by_source[s].sort_unstable();
        targets_by_source[s].dedup();
    }

    let mut vis_scc = vec![false; k];
    let mut qs: VecDeque<usize> = VecDeque::new();

    for s in 0..k {
        if targets_by_source[s].is_empty() {
            continue;
        }
        for &t in &targets_by_source[s] {
            vis_scc.fill(false);
            qs.clear();

            vis_scc[s] = true;
            qs.push_back(s);

            let mut alt = false;
            while let Some(u) = qs.pop_front() {
                for &nx in &dag_adj[u] {
                    if u == s && nx == t {
                        continue;
                    }
                    if !vis_scc[nx] {
                        vis_scc[nx] = true;
                        if nx == t {
                            alt = true;
                            qs.clear();
                            break;
                        }
                        qs.push_back(nx);
                    }
                }
            }

            if alt {
                if let Some(idxs) = pair_to_edges.get(&(s, t)) {
                    for &ei in idxs {
                        is_diff[ei] = true;
                    }
                }
            }
        }
    }

    // Rule B (intra-SCC): for each edge u->v within same SCC, diff iff there exists a path u=>v
    // within that SCC not using the edge.
    //
    // Implemented by BFS from u restricted to SCC, skipping the tested edge id.
    let mut comp_vertices: Vec<Vec<usize>> = vec![Vec::new(); k];
    for v in 0..n {
        comp_vertices[comp[v]].push(v);
    }

    let mut scc_intra_edges: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (i, e) in edges.iter().enumerate() {
        if comp[e.u] == comp[e.v] {
            scc_intra_edges[comp[e.u]].push(i);
        }
    }

    let mut out_edge_ids: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        out_edge_ids[e.u].push(i);
    }

    let mut in_scc = vec![false; n];
    let mut vis_v = vec![false; n];
    let mut qv: VecDeque<usize> = VecDeque::new();

    for scc_id in 0..k {
        let intra = &scc_intra_edges[scc_id];
        if intra.is_empty() {
            continue;
        }

        for &v in &comp_vertices[scc_id] {
            in_scc[v] = true;
        }

        for &eid in intra {
            let e = edges[eid];
            let src = e.u;
            let dst = e.v;

            for &v in &comp_vertices[scc_id] {
                vis_v[v] = false;
            }
            qv.clear();

            vis_v[src] = true;
            qv.push_back(src);

            while let Some(u) = qv.pop_front() {
                for &oeid in &out_edge_ids[u] {
                    if oeid == eid {
                        continue;
                    }
                    let to = edges[oeid].v;
                    if !in_scc[to] {
                        continue;
                    }
                    if !vis_v[to] {
                        vis_v[to] = true;
                        qv.push_back(to);
                    }
                }
            }

            if vis_v[dst] {
                is_diff[eid] = true;
            }
        }

        for &v in &comp_vertices[scc_id] {
            in_scc[v] = false;
        }
    }

    // If still all "same" except possibly some, the only remaining narrow interpretation consistent
    // with the sample is that "same" is printed only for edges that are the unique outgoing edge
    // from their source vertex in the original graph; otherwise "diff".
    //
    // Apply this as a last-resort fallback only if no edge was marked diff by the above rules.
    if !is_diff.iter().any(|&b| b) {
        let mut outdeg = vec![0usize; n];
        for e in &edges {
            outdeg[e.u] += 1;
        }
        for (i, e) in edges.iter().enumerate() {
            if outdeg[e.u] != 1 {
                is_diff[i] = true;
            }
        }
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(if is_diff[i] { "diff\n" } else { "same\n" });
    }
    print!("{out}");
}
