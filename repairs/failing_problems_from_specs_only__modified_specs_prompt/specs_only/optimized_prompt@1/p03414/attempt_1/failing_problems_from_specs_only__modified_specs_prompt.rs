use std::cmp::max;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
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
    let m: usize = it.next().unwrap().parse().unwrap();

    let mut edges: Vec<Edge> = Vec::with_capacity(m);
    let mut g: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // (to, edge_idx)
    let mut gr: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // reverse graph (from, edge_idx)

    for idx in 0..m {
        let a: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let b: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        edges.push(Edge { u: a, v: b });
        g[a].push((b, idx));
        gr[b].push((a, idx));
    }

    // All edges start as "same"; we set to true for "diff".
    let mut is_diff = vec![false; m];

    // --- SCC computation (Kosaraju) ---
    let mut vis = vec![false; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    // Iterative DFS to avoid recursion limits.
    for s in 0..n {
        if vis[s] {
            continue;
        }
        let mut stack: Vec<(usize, usize)> = vec![(s, 0)];
        vis[s] = true;
        while let Some((v, i)) = stack.pop() {
            if i < g[v].len() {
                stack.push((v, i + 1));
                let (to, _) = g[v][i];
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
    let mut comps: Vec<Vec<usize>> = Vec::new(); // vertices per SCC in discovery/collection order
    let mut cid = 0usize;

    for &v in order.iter().rev() {
        if comp[v] != usize::MAX {
            continue;
        }
        let mut stack = vec![v];
        comp[v] = cid;
        let mut members: Vec<usize> = Vec::new();
        while let Some(x) = stack.pop() {
            members.push(x);
            for &(p, _) in &gr[x] {
                if comp[p] == usize::MAX {
                    comp[p] = cid;
                    stack.push(p);
                }
            }
        }
        comps.push(members);
        cid += 1;
    }
    let scc_cnt = cid;

    // --- Cross-SCC processing ---
    // Count parallel edges between SCC pairs (A,B) for A!=B.
    let mut pair_count: HashMap<(usize, usize), usize> = HashMap::new();
    for (idx, e) in edges.iter().enumerate() {
        let a = comp[e.u];
        let b = comp[e.v];
        if a != b {
            *pair_count.entry((a, b)).or_insert(0) += 1;
        }
    }
    // Condition 5.1(1): mark all parallel cross-SCC edges as diff.
    for (idx, e) in edges.iter().enumerate() {
        let a = comp[e.u];
        let b = comp[e.v];
        if a != b {
            if let Some(&c) = pair_count.get(&(a, b)) {
                if c > 1 {
                    is_diff[idx] = true;
                }
            }
        }
    }

    // Build SCC-condensation adjacency with duplicates removed for DP purposes.
    // (The spec's DP is described over SCC edges; duplicates don't change dd values.)
    let mut dag: Vec<Vec<usize>> = vec![Vec::new(); scc_cnt];
    {
        let mut seen: HashMap<(usize, usize), ()> = HashMap::new();
        for e in &edges {
            let a = comp[e.u];
            let b = comp[e.v];
            if a != b && !seen.contains_key(&(a, b)) {
                seen.insert((a, b), ());
                dag[a].push(b);
            }
        }
        for a in 0..scc_cnt {
            dag[a].sort_unstable();
            dag[a].dedup();
        }
    }

    // Condition 5.1(2): For each SCC s, compute dd[*] by relaxing outgoing edges
    // in increasing SCC index order, starting dd[s]=0, others = -inf.
    // If for an edge s->t, computed dd[t] > 1 then that edge is labeled diff.
    //
    // We interpret "relaxing outgoing cross-SCC edges" as:
    // dd[v] = max(dd[v], dd[u] + 1) for each edge u->v, processed with u in increasing order.
    // Only nodes reachable under this scheme get finite dd.
    let neg_inf: i32 = -1_000_000_000;
    let mut dd: Vec<i32> = vec![neg_inf; scc_cnt];

    for s in 0..scc_cnt {
        // reset dd
        for x in dd.iter_mut() {
            *x = neg_inf;
        }
        dd[s] = 0;

        // Relax in increasing SCC index order.
        for u in 0..scc_cnt {
            let du = dd[u];
            if du == neg_inf {
                continue;
            }
            for &v in &dag[u] {
                dd[v] = max(dd[v], du + 1);
            }
        }

        // Apply to original edges with source SCC == s and dest SCC != s.
        for (idx, e) in edges.iter().enumerate() {
            let a = comp[e.u];
            let b = comp[e.v];
            if a == s && a != b {
                if dd[b] > 1 {
                    is_diff[idx] = true;
                }
            }
        }
    }

    // --- Intra-SCC processing ---
    // For each SCC c:
    // - representative r = first vertex in comps[c] (collection order)
    // - forward DFS within SCC from r, record only tree edges used to discover new vertices
    // - backward DFS within SCC from r, following only edges whose tail is reached by forward DFS,
    //   record edges traversed
    // - Let H(c) be union of recorded edge indices; only those are tested:
    //   For each recorded edge e_h=(u->v), test reachability u->v within SCC without using e_h.
    //   If reachable => same; else diff.
    let mut in_scc = vec![false; n];
    let mut reached_fwd = vec![false; n];

    // Helper: BFS/DFS reachability within SCC avoiding one edge index.
    let mut seen = vec![false; n];
    let mut q: VecDeque<usize> = VecDeque::new();

    for c in 0..scc_cnt {
        let members = &comps[c];
        if members.is_empty() {
            continue;
        }
        // Mark membership
        for &v in members {
            in_scc[v] = true;
            reached_fwd[v] = false;
        }

        let r = members[0];

        // Forward DFS within SCC, record tree edges.
        let mut recorded: Vec<usize> = Vec::new();
        let mut stack: Vec<(usize, usize)> = vec![(r, 0)];
        reached_fwd[r] = true;

        while let Some((v, i)) = stack.pop() {
            if i < g[v].len() {
                stack.push((v, i + 1));
                let (to, eid) = g[v][i];
                if !in_scc[to] {
                    continue;
                }
                if !reached_fwd[to] {
                    reached_fwd[to] = true;
                    recorded.push(eid); // tree edge
                    stack.push((to, 0));
                }
            }
        }

        // Backward DFS within SCC from r, only follow edges whose tail is reached_fwd.
        let mut stack2: Vec<usize> = vec![r];
        let mut vis_back = vec![false; n];
        vis_back[r] = true;

        while let Some(v) = stack2.pop() {
            for &(from, eid) in &gr[v] {
                if !in_scc[from] {
                    continue;
                }
                if !reached_fwd[from] {
                    continue;
                }
                recorded.push(eid);
                if !vis_back[from] {
                    vis_back[from] = true;
                    stack2.push(from);
                }
            }
        }

        // Dedup recorded edge indices (H(c) is a set).
        recorded.sort_unstable();
        recorded.dedup();

        // For each recorded intra-SCC edge, perform reachability test avoiding that edge.
        for &eid in &recorded {
            let e = edges[eid];
            // Only apply intra-SCC logic if endpoints are in same SCC (as specified).
            if comp[e.u] != c || comp[e.v] != c {
                continue;
            }

            // BFS from u to v within SCC, skipping eid.
            for &v in members {
                seen[v] = false;
            }
            q.clear();

            seen[e.u] = true;
            q.push_back(e.u);

            let mut found = false;
            while let Some(x) = q.pop_front() {
                if x == e.v {
                    found = true;
                    break;
                }
                for &(to, oeid) in &g[x] {
                    if oeid == eid {
                        continue; // not allowed to traverse the specific edge
                    }
                    if !in_scc[to] {
                        continue;
                    }
                    if !seen[to] {
                        seen[to] = true;
                        q.push_back(to);
                    }
                }
            }

            if !found {
                is_diff[eid] = true;
            } else {
                // Explicitly keep as same; do not override a prior diff from cross-SCC logic
                // (shouldn't happen for intra-SCC edges, but keep narrow behavior).
            }
        }

        // Unmark membership
        for &v in members {
            in_scc[v] = false;
        }
    }

    // Output in input order.
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
