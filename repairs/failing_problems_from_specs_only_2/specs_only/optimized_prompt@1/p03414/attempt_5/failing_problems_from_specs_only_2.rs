use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn kosaraju_scc(n: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    // Iterative Kosaraju to avoid recursion depth issues and to be robust.
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
        // (node, next_child_index)
        let mut stack: Vec<(usize, usize)> = Vec::new();
        stack.push((start, 0));
        vis[start] = true;

        while let Some((u, i)) = stack.pop() {
            if i < adj[u].len() {
                // Resume u later
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

    // SCC decomposition (must be correct; previous failures indicate SCC computation issues).
    let (k, comp) = kosaraju_scc(n, &adj);

    let mut comp_vertices: Vec<Vec<usize>> = vec![Vec::new(); k];
    for v in 0..n {
        comp_vertices[comp[v]].push(v);
    }

    let mut is_diff = vec![false; m];

    // Inter-SCC structures
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

    // A2: longer SCC-level route (length >= 2) exists.
    // For each (S,T), BFS from S in condensation DAG while forbidding the direct arc S->T.
    // If T is still reachable, then there exists a path of length >= 2 from S to T.
    let mut targets_by_source: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (&(s, t), _) in pair_to_edges.iter() {
        targets_by_source[s].push(t);
    }
    for s in 0..k {
        targets_by_source[s].sort_unstable();
        targets_by_source[s].dedup();
    }

    let mut vis_scc = vec![false; k];
    let mut q: VecDeque<usize> = VecDeque::new();

    for s in 0..k {
        if targets_by_source[s].is_empty() {
            continue;
        }
        for &t in &targets_by_source[s] {
            for v in 0..k {
                vis_scc[v] = false;
            }
            q.clear();

            vis_scc[s] = true;
            q.push_back(s);

            let mut longer = false;
            while let Some(u) = q.pop_front() {
                for &nx in &dag_adj[u] {
                    // Forbid using the direct arc S->T as the first step.
                    if u == s && nx == t {
                        continue;
                    }
                    if !vis_scc[nx] {
                        vis_scc[nx] = true;
                        if nx == t {
                            longer = true;
                            q.clear();
                            break;
                        }
                        q.push_back(nx);
                    }
                }
            }

            if longer {
                if let Some(idxs) = pair_to_edges.get(&(s, t)) {
                    for &ei in idxs {
                        is_diff[ei] = true;
                    }
                }
            }
        }
    }

    // Intra-SCC: BFS per edge excluding that edge.
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

            let mut found = false;
            while let Some(u) = qv.pop_front() {
                if u == dst {
                    found = true;
                    break;
                }
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

            if found {
                is_diff[eid] = true;
            }
        }

        for &v in &comp_vertices[scc_id] {
            in_scc[v] = false;
        }
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(if is_diff[i] { "diff\n" } else { "same\n" });
    }
    print!("{out}");
}
