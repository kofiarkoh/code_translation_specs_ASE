use std::cmp::max;
use std::io::{self, Read};

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let m: usize = it.next().unwrap().parse().unwrap();

    // Original edges in input order (0-based vertices)
    let mut ii = vec![0usize; m];
    let mut jj = vec![0usize; m];

    // Adjacency lists over vertices, storing edge indices h
    let mut ae: Vec<Vec<usize>> = vec![Vec::new(); n]; // outgoing edges
    let mut af: Vec<Vec<usize>> = vec![Vec::new(); n]; // incoming edges (reverse graph)

    for h in 0..m {
        let mut i: usize = it.next().unwrap().parse().unwrap();
        let mut j: usize = it.next().unwrap().parse().unwrap();
        i -= 1;
        j -= 1;
        ii[h] = i;
        jj[h] = j;
        ae[i].push(h);
        af[j].push(h);
    }

    // --- SCC computation (Kosaraju), matching C semantics closely ---
    // cc: 0 = unvisited, -1 = visited in first pass, >=0 = SCC id
    let mut cc: Vec<i32> = vec![0; n];
    let mut order: Vec<usize> = Vec::with_capacity(n);

    // Iterative DFS1 to avoid recursion depth issues.
    for start in 0..n {
        if cc[start] != 0 {
            continue;
        }
        // stack of (vertex, next_edge_index)
        let mut stack: Vec<(usize, usize)> = Vec::new();
        cc[start] = -1;
        stack.push((start, 0));
        while let Some((v, idx)) = stack.pop() {
            if idx < ae[v].len() {
                // resume later
                stack.push((v, idx + 1));
                let h = ae[v][idx];
                let to = jj[h];
                if cc[to] == 0 {
                    cc[to] = -1;
                    stack.push((to, 0));
                }
            } else {
                // finished
                order.push(v);
            }
        }
    }

    // DFS2 on reverse graph, assigning SCC ids
    let mut n_scc: usize = 0;
    for &v in order.iter().rev() {
        if cc[v] != -1 {
            continue;
        }
        let cid = n_scc as i32;
        n_scc += 1;

        let mut stack: Vec<usize> = vec![v];
        cc[v] = cid;
        while let Some(x) = stack.pop() {
            for &h in &af[x] {
                let from = ii[h];
                if cc[from] == -1 {
                    cc[from] = cid;
                    stack.push(from);
                }
            }
        }
    }

    // --- Inter-SCC classification on condensation DAG ---
    // ae_scc[s] stores original edge indices h that go from SCC s to a different SCC.
    let mut ae_scc: Vec<Vec<usize>> = vec![Vec::new(); n_scc];
    for h in 0..m {
        let s = cc[ii[h]] as usize;
        let t = cc[jj[h]] as usize;
        if s != t {
            ae_scc[s].push(h);
        }
    }

    let mut diff = vec![false; m];

    // For each SCC s, mark:
    // 1) duplicates to same target SCC as diff
    // 2) edges s->t as diff if there exists a path of length >=2 from s to t in condensation DAG
    //
    // Note: The C code uses a DP-like relaxation in SCC index order (i from s..n_scc-1),
    // which is not a general topological order. However, SCC ids produced by Kosaraju
    // in that implementation are used as-is by the program; the SPEC states reachability
    // is evaluated "using the SCC indices produced by the SCC computation" and corresponds
    // to whether T is reachable from S via at least two condensation arcs.
    //
    // To preserve intended behavior per SPEC (and avoid relying on incidental SCC numbering),
    // we compute reachability in the condensation DAG properly (BFS/DFS) and then check
    // for existence of a length>=2 path.
    //
    // This matches the SPEC even if it differs from the C's index-ordered relaxation on
    // some graphs/SCC numberings.
    //
    // Mismatch note: If one interprets the C code literally, its DP may under-approximate
    // reachability when SCC ids are not in topological order. We follow the SPEC.
    let mut dd_count = vec![0u32; n_scc];
    for s in 0..n_scc {
        // Count parallel arcs (S,T) by target SCC
        dd_count.fill(0);
        for &h in &ae_scc[s] {
            let t = cc[jj[h]] as usize;
            dd_count[t] += 1;
        }
        for &h in &ae_scc[s] {
            let t = cc[jj[h]] as usize;
            if dd_count[t] > 1 {
                diff[h] = true;
            }
        }

        // Build condensation adjacency (unique targets) for reachability from s
        // We can reuse ae_scc but need SCC-level neighbors.
        let mut neigh: Vec<Vec<usize>> = vec![Vec::new(); n_scc];
        // Build once globally would be better, but n<=1000 so OK; still, do it globally:
        // We'll instead build global adjacency outside loop.
        drop(neigh);
    }

    // Global condensation adjacency (unique SCC arcs)
    let mut cond_adj: Vec<Vec<usize>> = vec![Vec::new(); n_scc];
    {
        // Use a temporary boolean marker per source to deduplicate targets
        let mut seen = vec![false; n_scc];
        for s in 0..n_scc {
            seen.fill(false);
            for &h in &ae_scc[s] {
                let t = cc[jj[h]] as usize;
                if !seen[t] {
                    seen[t] = true;
                    cond_adj[s].push(t);
                }
            }
        }
    }

    // For each s, compute shortest path length in condensation DAG (in edges) to all nodes.
    // Then for each original edge s->t, if dist[t] >= 2 mark diff.
    let mut dist = vec![-1i32; n_scc];
    let mut q: std::collections::VecDeque<usize> = std::collections::VecDeque::new();
    for s in 0..n_scc {
        dist.fill(-1);
        q.clear();
        dist[s] = 0;
        q.push_back(s);
        while let Some(x) = q.pop_front() {
            let dx = dist[x];
            for &y in &cond_adj[x] {
                if dist[y] == -1 {
                    dist[y] = dx + 1;
                    q.push_back(y);
                }
            }
        }
        for &h in &ae_scc[s] {
            let t = cc[jj[h]] as usize;
            if dist[t] >= 2 {
                diff[h] = true;
            }
        }
    }

    // --- Intra-SCC classification ---
    // For each SCC c, for each edge u->v within c:
    // diff if there exists an alternative path u=>v within c that does NOT use that edge.
    //
    // The C code restricts candidate edges to those discovered by two DFS traversals
    // (dfs3 forward + dfs4 reverse) from an arbitrary vertex in the SCC, then tests each
    // such edge with a reachability query excluding that edge.
    //
    // Per SPEC, classification must apply to all intra-SCC edges. We'll implement it
    // directly: for each intra-SCC edge, run a DFS/BFS within the SCC excluding that edge.
    // n<=1000, m<=200000; worst-case per-edge search could be heavy, but SCC sizes are
    // bounded by n and this is the spec-driven correct behavior.
    //
    // To keep it efficient, we group edges by SCC and for each SCC build its vertex list.
    let mut verts_in_scc: Vec<Vec<usize>> = vec![Vec::new(); n_scc];
    for v in 0..n {
        verts_in_scc[cc[v] as usize].push(v);
    }

    let mut visited = vec![false; n];

    for c in 0..n_scc {
        // Collect intra-SCC edges (by original index)
        let mut intra_edges: Vec<usize> = Vec::new();
        for &v in &verts_in_scc[c] {
            for &h in &ae[v] {
                if cc[jj[h]] as usize == c {
                    intra_edges.push(h);
                }
            }
        }

        // For each intra edge, test reachability excluding it
        for &h_excl in &intra_edges {
            let src = ii[h_excl];
            let dst = jj[h_excl];

            // DFS from src to dst within SCC c, excluding edge h_excl
            visited.fill(false);
            let mut stack: Vec<usize> = vec![src];
            visited[src] = true;
            let mut found = false;

            while let Some(x) = stack.pop() {
                if x == dst {
                    found = true;
                    break;
                }
                for &h in &ae[x] {
                    if h == h_excl {
                        continue;
                    }
                    let y = jj[h];
                    if cc[y] as usize != c {
                        continue;
                    }
                    if !visited[y] {
                        visited[y] = true;
                        stack.push(y);
                    }
                }
            }

            // SPEC: diff if alternative path exists; same otherwise.
            // (C sets diff[h] = !dfs5(...), i.e., diff when no alternative path exists,
            // but that is opposite of the SPEC. We follow the SPEC.)
            diff[h_excl] = found;
        }
    }

    // Output in input order
    let mut out = String::new();
    for h in 0..m {
        if diff[h] {
            out.push_str("diff\n");
        } else {
            out.push_str("same\n");
        }
    }
    print!("{out}");
}
