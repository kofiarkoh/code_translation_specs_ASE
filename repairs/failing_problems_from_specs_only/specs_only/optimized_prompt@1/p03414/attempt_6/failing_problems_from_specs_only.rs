use std::cmp::min;
use std::collections::{HashMap, VecDeque};
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn tarjan_scc(n: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>) {
    let mut index: i32 = 0;
    let mut indices = vec![-1i32; n];
    let mut lowlink = vec![0i32; n];
    let mut onstack = vec![false; n];
    let mut stack: Vec<usize> = Vec::new();

    let mut comp_id = vec![usize::MAX; n];
    let mut comp_cnt: usize = 0;

    fn strongconnect(
        v: usize,
        index: &mut i32,
        indices: &mut [i32],
        lowlink: &mut [i32],
        stack: &mut Vec<usize>,
        onstack: &mut [bool],
        comp_id: &mut [usize],
        comp_cnt: &mut usize,
        adj: &[Vec<usize>],
    ) {
        indices[v] = *index;
        lowlink[v] = *index;
        *index += 1;
        stack.push(v);
        onstack[v] = true;

        for &w in &adj[v] {
            if indices[w] == -1 {
                strongconnect(
                    w, index, indices, lowlink, stack, onstack, comp_id, comp_cnt, adj,
                );
                lowlink[v] = min(lowlink[v], lowlink[w]);
            } else if onstack[w] {
                lowlink[v] = min(lowlink[v], indices[w]);
            }
        }

        if lowlink[v] == indices[v] {
            loop {
                let w = stack.pop().unwrap();
                onstack[w] = false;
                comp_id[w] = *comp_cnt;
                if w == v {
                    break;
                }
            }
            *comp_cnt += 1;
        }
    }

    for v in 0..n {
        if indices[v] == -1 {
            strongconnect(
                v,
                &mut index,
                &mut indices,
                &mut lowlink,
                &mut stack,
                &mut onstack,
                &mut comp_id,
                &mut comp_cnt,
                adj,
            );
        }
    }

    (comp_cnt, comp_id)
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

    let (scc_cnt, scc_of) = tarjan_scc(n, &adj);

    let mut is_diff = vec![false; m];

    // Intra-SCC adjacency with edge indices (to exclude the tested edge).
    let mut intra_adj: Vec<Vec<Vec<(usize, usize)>>> = vec![vec![Vec::new(); n]; scc_cnt];
    for (i, e) in edges.iter().enumerate() {
        let su = scc_of[e.u];
        let sv = scc_of[e.v];
        if su == sv {
            intra_adj[su][e.u].push((e.v, i));
        }
    }

    // Intra-SCC classification.
    for (i, e) in edges.iter().enumerate() {
        let s = scc_of[e.u];
        if s != scc_of[e.v] {
            continue;
        }

        let mut q = VecDeque::new();
        let mut vis = vec![false; n];
        vis[e.u] = true;
        q.push_back(e.u);

        let mut found = false;
        while let Some(x) = q.pop_front() {
            if x == e.v {
                found = true;
                break;
            }
            for &(to, eid) in &intra_adj[s][x] {
                if eid == i {
                    continue;
                }
                if !vis[to] {
                    vis[to] = true;
                    q.push_back(to);
                }
            }
        }

        if found {
            is_diff[i] = true;
        }
    }

    // Inter-SCC: group edges by ordered SCC pair.
    let mut pair_to_edges: HashMap<(usize, usize), Vec<usize>> = HashMap::new();
    for (i, e) in edges.iter().enumerate() {
        let su = scc_of[e.u];
        let sv = scc_of[e.v];
        if su != sv {
            pair_to_edges.entry((su, sv)).or_default().push(i);
        }
    }

    // Condition (1): parallel SCC-arc duplication.
    for idxs in pair_to_edges.values() {
        if idxs.len() > 1 {
            for &eid in idxs {
                is_diff[eid] = true;
            }
        }
    }

    // Build condensation adjacency (unique arcs) and indegrees.
    let mut cond_adj: Vec<Vec<usize>> = vec![Vec::new(); scc_cnt];
    let mut indeg = vec![0usize; scc_cnt];
    {
        let mut seen: HashMap<(usize, usize), ()> = HashMap::new();
        for &(s, t) in pair_to_edges.keys() {
            if seen.insert((s, t), ()).is_none() {
                cond_adj[s].push(t);
                indeg[t] += 1;
            }
        }
    }

    // Topological order.
    let mut dq = VecDeque::new();
    for i in 0..scc_cnt {
        if indeg[i] == 0 {
            dq.push_back(i);
        }
    }
    let mut topo: Vec<usize> = Vec::with_capacity(scc_cnt);
    while let Some(x) = dq.pop_front() {
        topo.push(x);
        for &y in &cond_adj[x] {
            indeg[y] -= 1;
            if indeg[y] == 0 {
                dq.push_back(y);
            }
        }
    }

    // Reachability bitsets on condensation DAG:
    // reach[s] contains all nodes reachable from s via >=1 arcs.
    let words = (scc_cnt + 63) / 64;
    let mut reach: Vec<Vec<u64>> = vec![vec![0u64; words]; scc_cnt];
    for &s in topo.iter().rev() {
        for &t in &cond_adj[s] {
            reach[s][t / 64] |= 1u64 << (t % 64);
            for w in 0..words {
                reach[s][w] |= reach[t][w];
            }
        }
    }

    // Condition (2): existence of a longer SCC-level route.
    //
    // The sample behavior indicates the check is simply whether T is reachable from S
    // in the condensation graph by *any* path other than the direct arc S->T, i.e.
    // whether there exists a path of length >= 2 from S to T.
    //
    // We implement this by temporarily removing the direct arc (S->T) and checking
    // reachability from S to T in the remaining condensation graph.
    //
    // Since SCC count <= n <= 1000, doing a BFS per distinct SCC-arc is acceptable.
    for (&(s, t), idxs) in pair_to_edges.iter() {
        // If already marked diff by duplication, we still must preserve semantics; condition (2)
        // can only add more diffs, never revert.
        let mut q = VecDeque::new();
        let mut vis = vec![false; scc_cnt];
        vis[s] = true;
        q.push_back(s);

        let mut found = false;
        while let Some(x) = q.pop_front() {
            for &y in &cond_adj[x] {
                // Skip the direct arc S->T only when traversing from S to T in one step.
                if x == s && y == t {
                    continue;
                }
                if !vis[y] {
                    if y == t {
                        found = true;
                        break;
                    }
                    vis[y] = true;
                    q.push_back(y);
                }
            }
            if found {
                break;
            }
        }

        if found {
            for &eid in idxs {
                is_diff[eid] = true;
            }
        }
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(if is_diff[i] { "diff\n" } else { "same\n" });
    }
    print!("{out}");
}
