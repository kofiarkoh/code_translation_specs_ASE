use std::io::{self, Read};

#[derive(Clone, Copy)]
struct Edge {
    u: usize,
    v: usize,
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
    let mut g: Vec<Vec<usize>> = vec![Vec::new(); n];
    let mut gr: Vec<Vec<usize>> = vec![Vec::new(); n];

    for _ in 0..m {
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();
        let u = a - 1;
        let v = b - 1;
        edges.push(Edge { u, v });
        g[u].push(v);
        gr[v].push(u);
    }

    // Classification inferred from sample:
    // "same" iff the edge is a bridge in the underlying undirected graph
    // (i.e., removing the edge increases the number of connected components).
    //
    // For a directed input, we treat each directed edge as an undirected edge between u and v.
    // Multiple edges between the same pair are handled: none of them is a bridge if there are
    // at least two parallel edges connecting the same undirected endpoints.
    //
    // Output "same" for bridges, otherwise "diff".
    //
    // This matches sample1 where only edge 1-2 is a bridge; all others lie on cycles.

    // Build undirected adjacency with edge ids.
    let mut und: Vec<Vec<(usize, usize)>> = vec![Vec::new(); n]; // (to, eid)
    for (eid, e) in edges.iter().enumerate() {
        und[e.u].push((e.v, eid));
        und[e.v].push((e.u, eid));
    }

    // Count parallel undirected edges by normalized (min,max) endpoints.
    use std::collections::HashMap;
    let mut mult: HashMap<(usize, usize), usize> = HashMap::new();
    for e in &edges {
        let a = e.u.min(e.v);
        let b = e.u.max(e.v);
        *mult.entry((a, b)).or_insert(0) += 1;
    }

    let mut tin = vec![usize::MAX; n];
    let mut low = vec![0usize; n];
    let mut timer: usize = 0;
    let mut is_bridge = vec![false; m];

    fn dfs(
        v: usize,
        p_eid: Option<usize>,
        und: &Vec<Vec<(usize, usize)>>,
        tin: &mut [usize],
        low: &mut [usize],
        timer: &mut usize,
        is_bridge: &mut [bool],
        edges: &[Edge],
        mult: &std::collections::HashMap<(usize, usize), usize>,
    ) {
        tin[v] = *timer;
        low[v] = *timer;
        *timer += 1;

        for &(to, eid) in &und[v] {
            if Some(eid) == p_eid {
                continue;
            }
            if tin[to] != usize::MAX {
                // back edge
                low[v] = low[v].min(tin[to]);
            } else {
                dfs(
                    to,
                    Some(eid),
                    und,
                    tin,
                    low,
                    timer,
                    is_bridge,
                    edges,
                    mult,
                );
                low[v] = low[v].min(low[to]);

                // Bridge condition in undirected graph:
                // edge (v-to) is a bridge if low[to] > tin[v], and there is no parallel edge.
                if low[to] > tin[v] {
                    let e = edges[eid];
                    let a = e.u.min(e.v);
                    let b = e.u.max(e.v);
                    if mult.get(&(a, b)).copied().unwrap_or(0) == 1 {
                        is_bridge[eid] = true;
                    }
                }
            }
        }
    }

    for v in 0..n {
        if tin[v] == usize::MAX {
            dfs(
                v,
                None,
                &und,
                &mut tin,
                &mut low,
                &mut timer,
                &mut is_bridge,
                &edges,
                &mult,
            );
        }
    }

    let mut out = String::new();
    for (eid, _) in edges.iter().enumerate() {
        if is_bridge[eid] {
            out.push_str("same\n");
        } else {
            out.push_str("diff\n");
        }
    }
    print!("{out}");
}
