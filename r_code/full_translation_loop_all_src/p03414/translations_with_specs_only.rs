use std::collections::VecDeque;
use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
}

fn tarjan_scc(n: usize, adj: &[Vec<usize>]) -> (usize, Vec<usize>, Vec<Vec<usize>>) {
    let mut index: i32 = 0;
    let mut stack: Vec<usize> = Vec::new();
    let mut on_stack = vec![false; n];
    let mut idx = vec![-1i32; n];
    let mut low = vec![0i32; n];

    let mut comp_id = vec![usize::MAX; n];
    let mut comps: Vec<Vec<usize>> = Vec::new();

    fn strongconnect(
        v: usize,
        index: &mut i32,
        stack: &mut Vec<usize>,
        on_stack: &mut [bool],
        idx: &mut [i32],
        low: &mut [i32],
        adj: &[Vec<usize>],
        comp_id: &mut [usize],
        comps: &mut Vec<Vec<usize>>,
    ) {
        idx[v] = *index;
        low[v] = *index;
        *index += 1;

        stack.push(v);
        on_stack[v] = true;

        for &w in &adj[v] {
            if idx[w] == -1 {
                strongconnect(w, index, stack, on_stack, idx, low, adj, comp_id, comps);
                low[v] = low[v].min(low[w]);
            } else if on_stack[w] {
                low[v] = low[v].min(idx[w]);
            }
        }

        if low[v] == idx[v] {
            let mut comp: Vec<usize> = Vec::new();
            loop {
                let w = stack.pop().unwrap();
                on_stack[w] = false;
                comp_id[w] = comps.len();
                comp.push(w);
                if w == v {
                    break;
                }
            }
            comps.push(comp);
        }
    }

    for v in 0..n {
        if idx[v] == -1 {
            strongconnect(
                v,
                &mut index,
                &mut stack,
                &mut on_stack,
                &mut idx,
                &mut low,
                adj,
                &mut comp_id,
                &mut comps,
            );
        }
    }

    (comps.len(), comp_id, comps)
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

    let (k, comp, comps) = tarjan_scc(n, &adj);

    let mut is_diff = vec![false; m];

    // Build condensation graph with multiplicities.
    // Also keep, for each ordered SCC pair (S,T), the list of original edge indices inducing it.
    let mut pair_edges: Vec<Vec<Vec<usize>>> = vec![vec![Vec::new(); k]; k];
    let mut cond_adj: Vec<Vec<usize>> = vec![Vec::new(); k];
    let mut cond_has: Vec<Vec<bool>> = vec![vec![false; k]; k];

    // Intra-SCC edge indices per SCC
    let mut intra_by_scc: Vec<Vec<usize>> = vec![Vec::new(); k];

    for (i, e) in edges.iter().enumerate() {
        let s = comp[e.u];
        let t = comp[e.v];
        if s == t {
            intra_by_scc[s].push(i);
        } else {
            pair_edges[s][t].push(i);
            if !cond_has[s][t] {
                cond_has[s][t] = true;
                cond_adj[s].push(t);
            }
        }
    }

    // Rule A1: duplicates between same ordered SCC pair => all diff
    for s in 0..k {
        for t in 0..k {
            if pair_edges[s][t].len() > 1 {
                for &ei in &pair_edges[s][t] {
                    is_diff[ei] = true;
                }
            }
        }
    }

    // Rule A2: longer SCC-level route (path length >= 2) in condensation DAG.
    // For each direct arc S->T, check if T is reachable from S without using that direct arc.
    // If yes, then there exists a path of length >= 2 (since SCC condensation is a DAG, no cycles).
    let mut seen = vec![false; k];
    for s in 0..k {
        for &t in &cond_adj[s] {
            // BFS from s, skipping the single arc s->t.
            seen.fill(false);
            let mut q: VecDeque<usize> = VecDeque::new();
            seen[s] = true;
            q.push_back(s);

            let mut found = false;
            while let Some(x) = q.pop_front() {
                for &y in &cond_adj[x] {
                    if x == s && y == t {
                        continue; // exclude the direct arc under test
                    }
                    if !seen[y] {
                        seen[y] = true;
                        if y == t {
                            found = true;
                            break;
                        }
                        q.push_back(y);
                    }
                }
                if found {
                    break;
                }
            }

            if found {
                for &ei in &pair_edges[s][t] {
                    is_diff[ei] = true;
                }
            }
        }
    }

    // Intra-SCC rule: edge u->v is diff if there exists an alternative path u=>v within SCC
    // that does not use the edge itself.
    //
    // Implement per SCC: build local adjacency with edge indices, then per edge BFS skipping it.
    for scc in 0..k {
        // Map global vertex id -> local [0..sz)
        let mut map_local = vec![usize::MAX; n];
        for (li, &v) in comps[scc].iter().enumerate() {
            map_local[v] = li;
        }
        let sz = comps[scc].len();
        let mut ladj: Vec<Vec<(usize, usize)>> = vec![Vec::new(); sz]; // (to_local, edge_index)

        for &ei in &intra_by_scc[scc] {
            let e = edges[ei];
            let lu = map_local[e.u];
            let lv = map_local[e.v];
            ladj[lu].push((lv, ei));
        }

        let mut lseen = vec![false; sz];
        for &ei in &intra_by_scc[scc] {
            let e = edges[ei];
            let src = map_local[e.u];
            let dst = map_local[e.v];

            lseen.fill(false);
            let mut q: VecDeque<usize> = VecDeque::new();
            lseen[src] = true;
            q.push_back(src);

            let mut found = false;
            while let Some(x) = q.pop_front() {
                if x == dst {
                    found = true;
                    break;
                }
                for &(y, edge_idx) in &ladj[x] {
                    if edge_idx == ei {
                        continue;
                    }
                    if !lseen[y] {
                        lseen[y] = true;
                        q.push_back(y);
                    }
                }
            }

            if found {
                is_diff[ei] = true;
            }
        }
    }

    let mut out = String::new();
    for i in 0..m {
        out.push_str(if is_diff[i] { "diff\n" } else { "same\n" });
    }
    print!("{out}");
}
