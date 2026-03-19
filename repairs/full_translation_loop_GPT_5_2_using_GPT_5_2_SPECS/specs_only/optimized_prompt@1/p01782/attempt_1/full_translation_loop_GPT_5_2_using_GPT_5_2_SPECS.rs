use std::cmp::min;
use std::io::{self, Read};

const INF: i32 = 1_000_000_000;

fn char_rank(c: u8) -> usize {
    // Order: 'A'..'Z' then 'a'..'z'
    if (b'A'..=b'Z').contains(&c) {
        (c - b'A') as usize
    } else {
        26 + (c - b'a') as usize
    }
}

fn min_cost_max_flow(cost: &Vec<Vec<i32>>, n: usize) -> (i32, Vec<usize>) {
    // Assignment via min-cost max-flow on bipartite graph:
    // source -> rows -> cols -> sink, capacity 1, row->col cost = cost[i][j].
    // Returns (min_cost, matching) where matching[i] = chosen column for row i.
    let v = 2 * n + 2;
    let s = 2 * n;
    let t = 2 * n + 1;

    #[derive(Clone)]
    struct Edge {
        to: usize,
        rev: usize,
        cap: i32,
        cost: i32,
    }

    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); v];

    let mut add_edge = |u: usize, w: usize, cap: i32, cost: i32, g: &mut Vec<Vec<Edge>>| {
        let a = Edge {
            to: w,
            rev: g[w].len(),
            cap,
            cost,
        };
        let b = Edge {
            to: u,
            rev: g[u].len(),
            cap: 0,
            cost: -cost,
        };
        g[u].push(a);
        g[w].push(b);
    };

    for i in 0..n {
        add_edge(s, i, 1, 0, &mut g);
    }
    for j in 0..n {
        add_edge(n + j, t, 1, 0, &mut g);
    }
    for i in 0..n {
        for j in 0..n {
            add_edge(i, n + j, 1, cost[i][j], &mut g);
        }
    }

    let mut flow = 0;
    let mut total_cost: i32 = 0;

    let mut dist = vec![0i32; v];
    let mut inq = vec![false; v];
    let mut pv_v = vec![0usize; v];
    let mut pv_e = vec![0usize; v];

    while flow < n as i32 {
        for d in dist.iter_mut() {
            *d = INF;
        }
        dist[s] = 0;
        inq.fill(false);

        // SPFA (sufficient for these small bounds; costs are nonnegative on forward edges)
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        inq[s] = true;

        while let Some(u) = q.pop_front() {
            inq[u] = false;
            let du = dist[u];
            if du == INF {
                continue;
            }
            for (ei, e) in g[u].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                let nd = du.saturating_add(e.cost);
                if nd < dist[e.to] {
                    dist[e.to] = nd;
                    pv_v[e.to] = u;
                    pv_e[e.to] = ei;
                    if !inq[e.to] {
                        inq[e.to] = true;
                        q.push_back(e.to);
                    }
                }
            }
        }

        // Per spec, malformed/unsatisfiable cases are undefined; assume always reachable.
        if dist[t] == INF {
            break;
        }

        // Augment 1 unit (all capacities are 1 on s->row and col->t, so bottleneck is 1)
        let mut addf = 1i32;
        let mut vtx = t;
        while vtx != s {
            let u = pv_v[vtx];
            let ei = pv_e[vtx];
            addf = min(addf, g[u][ei].cap);
            vtx = u;
        }

        vtx = t;
        while vtx != s {
            let u = pv_v[vtx];
            let ei = pv_e[vtx];
            let rev = g[u][ei].rev;
            g[u][ei].cap -= addf;
            g[vtx][rev].cap += addf;
            vtx = u;
        }

        flow += addf;
        total_cost = total_cost.saturating_add(dist[t].saturating_mul(addf));
    }

    // Extract matching from residual graph: row i -> col j edge used iff its forward cap is 0.
    let mut matching = vec![0usize; n];
    for i in 0..n {
        let mut chosen = None;
        for e in &g[i] {
            if e.to >= n && e.to < 2 * n {
                // Original capacity was 1; if now 0, it carried flow.
                if e.cap == 0 {
                    chosen = Some(e.to - n);
                    break;
                }
            }
        }
        // Undefined behavior if not found; keep 0 as fallback.
        matching[i] = chosen.unwrap_or(0);
    }

    (total_cost, matching)
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut grid: Vec<Vec<u8>> = vec![vec![b'A'; n]; n];
    for i in 0..n {
        let line = it.next().unwrap_or("");
        let bytes = line.as_bytes();
        for j in 0..n {
            grid[i][j] = bytes[j];
        }
    }

    // Build cost matrix: lexicographic minimization of sorted multiset under rank order
    // is equivalent to minimizing the count vector in lexicographic order, which can be
    // encoded as a large-base positional cost.
    //
    // Choose base = N+1 so that each digit (count) is < base, preventing carry.
    // Use i32; N<=52, base<=53, and 53^52 is enormous, so we cannot represent exact
    // positional weights. Instead, we use a safe narrow interpretation:
    // perform iterative restriction by letter rank: for each rank r, minimize count of r
    // subject to feasibility, fixing previous ranks' counts to their minima.
    //
    // This avoids huge integers and preserves the specified lexicographic objective.
    let ranks = 52usize;

    // Precompute rank matrix
    let mut rmat = vec![vec![0usize; n]; n];
    for i in 0..n {
        for j in 0..n {
            rmat[i][j] = char_rank(grid[i][j]);
        }
    }

    // Allowed edges mask, initially all true
    let mut allowed = vec![vec![true; n]; n];

    // Final chosen matching (row -> col)
    let mut final_matching = vec![0usize; n];

    // For each rank, minimize how many times it appears, while keeping earlier ranks fixed.
    // We do this by:
    // - Assign cost 1 to edges of current rank, 0 otherwise, but disallow edges that would
    //   violate already-fixed earlier ranks by restricting allowed edges accordingly.
    // - After finding min count for this rank, further restrict allowed edges to those
    //   that can achieve that min count (by checking each edge via re-solving with it forced).
    //
    // This is polynomial but fine for N<=52.
    for cur in 0..ranks {
        // Build cost matrix for current rank with current allowed set.
        let mut cost = vec![vec![0i32; n]; n];
        for i in 0..n {
            for j in 0..n {
                if !allowed[i][j] {
                    cost[i][j] = INF / 4;
                } else {
                    cost[i][j] = if rmat[i][j] == cur { 1 } else { 0 };
                }
            }
        }

        let (minc, _) = min_cost_max_flow(&cost, n);

        // Restrict allowed edges to those that can still achieve minc for this rank.
        // For each edge, test feasibility with it forced (if it is currently allowed).
        // Keep edge if there exists an optimal solution using it.
        let mut new_allowed = vec![vec![false; n]; n];

        for fi in 0..n {
            for fj in 0..n {
                if !allowed[fi][fj] {
                    continue;
                }

                // Force (fi,fj) by:
                // - disallow other columns in row fi
                // - disallow other rows in column fj
                // This is the narrowest forcing consistent with bijection.
                let mut cost_forced = vec![vec![0i32; n]; n];
                for i in 0..n {
                    for j in 0..n {
                        let mut ok = allowed[i][j];
                        if i == fi && j != fj {
                            ok = false;
                        }
                        if j == fj && i != fi {
                            ok = false;
                        }
                        if !ok {
                            cost_forced[i][j] = INF / 4;
                        } else {
                            cost_forced[i][j] = if rmat[i][j] == cur { 1 } else { 0 };
                        }
                    }
                }

                let (cst, _) = min_cost_max_flow(&cost_forced, n);
                if cst == minc {
                    new_allowed[fi][fj] = true;
                }
            }
        }

        allowed = new_allowed;
    }

    // With all ranks processed, allowed edges should correspond to exactly the set of edges
    // that can appear in some globally optimal solution; pick any perfect matching within it.
    // Use min-cost max-flow with zero costs to find a matching.
    let mut zero_cost = vec![vec![0i32; n]; n];
    for i in 0..n {
        for j in 0..n {
            if !allowed[i][j] {
                zero_cost[i][j] = INF / 4;
            }
        }
    }
    let (_, matching) = min_cost_max_flow(&zero_cost, n);
    final_matching.copy_from_slice(&matching);

    // Count letters in chosen positions and output sorted with uppercase first.
    let mut counts = [0usize; 52];
    for i in 0..n {
        let j = final_matching[i];
        counts[rmat[i][j]] += 1;
    }

    let mut out = String::new();
    for r in 0..52 {
        let ch = if r < 26 {
            (b'A' + r as u8) as char
        } else {
            (b'a' + (r as u8 - 26)) as char
        };
        for _ in 0..counts[r] {
            out.push(ch);
        }
    }
    out.push('\n');
    print!("{out}");
}
