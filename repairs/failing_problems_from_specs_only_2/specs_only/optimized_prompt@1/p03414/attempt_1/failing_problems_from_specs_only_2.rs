use std::cmp::Ordering;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn tarjan_scc(n: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let mut index: i32 = 0;
    let mut stack: Vec<usize> = Vec::new();
    let mut on_stack = vec![false; n];
    let mut idx = vec![-1i32; n];
    let mut low = vec![0i32; n];
    let mut comp = vec![usize::MAX; n];
    let mut comp_cnt: usize = 0;

    fn dfs(
        v: usize,
        adj: &[Vec<usize>],
        index: &mut i32,
        stack: &mut Vec<usize>,
        on_stack: &mut [bool],
        idx: &mut [i32],
        low: &mut [i32],
        comp: &mut [usize],
        comp_cnt: &mut usize,
    ) {
        idx[v] = *index;
        low[v] = *index;
        *index += 1;
        stack.push(v);
        on_stack[v] = true;

        for &to in &adj[v] {
            if idx[to] == -1 {
                dfs(to, adj, index, stack, on_stack, idx, low, comp, comp_cnt);
                low[v] = low[v].min(low[to]);
            } else if on_stack[to] {
                low[v] = low[v].min(idx[to]);
            }
        }

        if low[v] == idx[v] {
            loop {
                let w = stack.pop().unwrap();
                on_stack[w] = false;
                comp[w] = *comp_cnt;
                if w == v {
                    break;
                }
            }
            *comp_cnt += 1;
        }
    }

    for v in 0..n {
        if idx[v] == -1 {
            dfs(
                v, adj, &mut index, &mut stack, &mut on_stack, &mut idx, &mut low, &mut comp,
                &mut comp_cnt,
            );
        }
    }

    (comp_cnt, comp)
}

fn topo_order_dag(k: usize, dag_adj: &[Vec<usize>]) -> Vec<usize> {
    let mut indeg = vec![0usize; k];
    for u in 0..k {
        for &v in &dag_adj[u] {
            indeg[v] += 1;
        }
    }
    let mut q = VecDeque::new();
    for i in 0..k {
        if indeg[i] == 0 {
            q.push_back(i);
        }
    }
    let mut order = Vec::with_capacity(k);
    while let Some(u) = q.pop_front() {
        order.push(u);
        for &v in &dag_adj[u] {
            indeg[v] -= 1;
            if indeg[v] == 0 {
                q.push_back(v);
            }
        }
    }
    // Condensation graph is a DAG; under spec constraints this should include all nodes.
    order
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

    // SCC decomposition
    let (k, comp) = tarjan_scc(n, &adj);

    // Group vertices by SCC
    let mut comp_vertices: Vec<Vec<usize>> = vec![Vec::new(); k];
    for v in 0..n {
        comp_vertices[comp[v]].push(v);
    }

    // Prepare output labels; default "same"
    let mut is_diff = vec![false; m];

    // Inter-SCC: count parallel SCC arcs and build condensation DAG (unique arcs)
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

    // Condition A1: parallel SCC-arc duplication
    for (_pair, idxs) in pair_to_edges.iter() {
        if idxs.len() > 1 {
            for &ei in idxs {
                is_diff[ei] = true;
            }
        }
    }

    // Condition A2: existence of a longer SCC-level route (path length >= 2)
    // Compute reachability in condensation DAG via bitsets, then for each arc S->T
    // check if there exists an intermediate X with S->X and X=>T.
    let words = (k + 63) / 64;
    let mut reach: Vec<Vec<u64>> = vec![vec![0u64; words]; k];

    for u in 0..k {
        for &v in &dag_adj[u] {
            reach[u][v / 64] |= 1u64 << (v % 64);
        }
    }

    let order = topo_order_dag(k, &dag_adj);
    // Process in reverse topological order: reach[u] |= reach[v] for each u->v
    for &u in order.iter().rev() {
        let outs = dag_adj[u].clone();
        for v in outs {
            // OR reach[v] into reach[u]
            for w in 0..words {
                reach[u][w] |= reach[v][w];
            }
        }
    }

    // For each unique arc (s,t), check if there exists intermediate x != t such that s->x and x reaches t.
    // This corresponds to a path of length >= 2 from s to t.
    for (&(s, t), idxs) in pair_to_edges.iter() {
        // If already diff due to duplication, still diff; but we can skip work if desired.
        // Keep semantics identical: marking diff again is harmless.
        let mut longer = false;
        for &x in &dag_adj[s] {
            if x == t {
                continue;
            }
            let bit = (reach[x][t / 64] >> (t % 64)) & 1;
            if bit != 0 {
                longer = true;
                break;
            }
        }
        if longer {
            for &ei in idxs {
                is_diff[ei] = true;
            }
        }
    }

    // Intra-SCC: for each SCC, classify edges inside it by whether there exists an alternative path u->v
    // within the SCC excluding the edge itself.
    //
    // Narrow interpretation faithful to spec:
    // - For each SCC, build its induced subgraph.
    // - For each source u in SCC, compute reachability to all nodes in SCC.
    // - For each edge u->v inside SCC:
    //     diff iff (reachable(u,v) without using that edge).
    //   We implement this by:
    //     - If there exists a path from u to v not using the direct edge, then diff.
    //     - Equivalent check:
    //         reachable(u,v) AND (either there is some path not starting with that edge).
    //       We compute:
    //         alt(u,v) = (exists w in out(u) within SCC, w != v, with w => v) OR
    //                    (edge u->v exists and v => v via a non-empty cycle) OR
    //                    (there are multiple parallel edges u->v within SCC).
    //       But to avoid inventing graph-theoretic shortcuts, we do the direct BFS-per-edge exclusion
    //       in a way that stays within constraints (n<=1000).
    //
    // Implementation approach:
    // - For each SCC:
    //     - Map its vertices to local indices [0..sz)
    //     - Build local adjacency list with original vertex ids
    //     - Collect intra edges indices for this SCC
    // - For each intra edge (u->v), run BFS from u within SCC skipping exactly that edge index.
    // This is O(sum intra_edges * (V_scc + E_scc)) worst-case; with n<=1000 this is acceptable
    // for typical inputs, and is the narrowest faithful interpretation of "does not use the edge".
    let mut scc_intra_edges: Vec<Vec<usize>> = vec![Vec::new(); k];
    for (i, e) in edges.iter().enumerate() {
        let s = comp[e.u];
        let t = comp[e.v];
        if s == t {
            scc_intra_edges[s].push(i);
        }
    }

    // Build per-vertex list of outgoing edge indices (original indices) to support skipping by edge id.
    let mut out_edge_ids: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (i, e) in edges.iter().enumerate() {
        out_edge_ids[e.u].push(i);
    }

    for scc_id in 0..k {
        let verts = &comp_vertices[scc_id];
        if verts.len() <= 1 {
            // Single-vertex SCC: only self-loops are intra; alternative path exists iff there is
            // another self-loop or a cycle using other edges, but with one vertex only, any path
            // u->u within SCC uses only self-loops. BFS exclusion handles this; keep processing.
        }
        let intra = &scc_intra_edges[scc_id];
        if intra.is_empty() {
            continue;
        }

        // Membership marker for fast "stays within SCC"
        let mut in_scc = vec![false; n];
        for &v in verts {
            in_scc[v] = true;
        }

        // For each intra edge, BFS excluding that edge id.
        let mut q: VecDeque<usize> = VecDeque::new();
        let mut vis = vec![false; n];

        for &eid in intra {
            let e = edges[eid];
            let src = e.u;
            let dst = e.v;

            // BFS from src within SCC, skipping exactly edge eid.
            for &v in verts {
                vis[v] = false;
            }
            q.clear();

            vis[src] = true;
            q.push_back(src);

            let mut found = false;
            while let Some(u) = q.pop_front() {
                // Early exit if we reached dst via some path (possibly length 0 if src==dst).
                // But path must not use the excluded edge; BFS respects that.
                if u == dst {
                    // If src==dst, this corresponds to empty path; spec requires a directed path
                    // from u to v that does not use the edge. Empty path uses no edges, so it counts.
                    // Therefore, any self-loop edge in an SCC is "diff" if u==v (always),
                    // because empty path exists without using the edge.
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
                    if !vis[to] {
                        vis[to] = true;
                        q.push_back(to);
                    }
                }
            }

            if found {
                is_diff[eid] = true;
            }
        }
    }

    // Output in input order
    let mut out = String::new();
    for i in 0..m {
        if is_diff[i] {
            out.push_str("diff\n");
        } else {
            out.push_str("same\n");
        }
    }
    print!("{out}");
}
