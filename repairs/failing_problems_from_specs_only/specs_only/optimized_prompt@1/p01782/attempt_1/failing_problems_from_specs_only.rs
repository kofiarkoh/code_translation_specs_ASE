use std::cmp::Ordering;
use std::io::{self, Read};

const ALPHABET: &[u8; 52] = b"ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz";

fn char_index(b: u8) -> usize {
    match b {
        b'A'..=b'Z' => (b - b'A') as usize,
        b'a'..=b'z' => 26 + (b - b'a') as usize,
        _ => 0, // Unspecified behavior for invalid input; keep total function.
    }
}

fn main() {
    // Read entire stdin.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut it = input.split_whitespace();
    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    // Read N lines (as tokens) and take first N bytes from each.
    // Assumption: each line has at least N usable characters.
    let mut grid: Vec<Vec<usize>> = vec![vec![0usize; n]; n];
    for i in 0..n {
        let tok = it.next().unwrap_or("");
        let bytes = tok.as_bytes();
        for j in 0..n {
            grid[i][j] = char_index(bytes[j]);
        }
    }

    // We need a perfect matching that lexicographically maximizes counts c_k
    // (equivalently lexicographically minimizes (-c_k)).
    //
    // Implement by iterating k=0..51 and, at each step, maximizing the number of
    // selected edges with character k subject to feasibility and keeping previous
    // counts fixed at their already-maximized values.
    //
    // This can be done by repeatedly finding augmenting paths in the current matching
    // using only edges that do not decrease the count of character k (i.e., along an
    // alternating path, the net gain in #k is +1).
    //
    // For a fixed k, define directed graph on bipartite nodes:
    // - From an unmatched row r to any column c via edge (r->c) if grid[r][c]==k (gain +1)
    //   or grid[r][c]!=k (gain 0).
    // - From a column c to its matched row r' via matched edge (c->r') with gain -1 if that
    //   matched edge has char k, else 0.
    // We want an alternating path from an unmatched row to an unmatched column with total gain +1.
    //
    // Since gains are in {-1,0,+1} and we only care about existence of gain +1 paths, we can
    // BFS in a layered manner tracking best gain achievable to each node (max gain).
    // Because gains are small, we can use a deque-like relaxation (SPFA-style) safely for small N.
    //
    // Additionally, to preserve previously fixed counts for earlier characters (<k), we must
    // forbid changing them. That means: along any alternating cycle/path augmentation, the net
    // change in counts for earlier characters must be 0. The narrowest faithful interpretation
    // is to enforce that no edge with earlier character is toggled at all (since any toggle would
    // change that earlier count unless perfectly balanced, which would require more complex
    // constraints not specified). However, the specification's lexicographic criterion only
    // requires earlier counts remain maximal, not necessarily unchanged per augmentation.
    //
    // To stay faithful without inventing extra tie-breaking, we implement the standard lexicographic
    // optimization via successive constraints:
    // - After maximizing c_0, we fix it exactly.
    // - Then maximize c_1 subject to c_0 fixed, etc.
    //
    // We enforce these constraints by disallowing any augmentation that changes any earlier count.
    // This is sufficient (and standard) because any augmentation changes counts by toggling edges;
    // keeping earlier counts fixed means net change 0 for each earlier character. Enforcing "no
    // toggles of earlier-character edges" is stronger than necessary, but it is the narrowest
    // safe interpretation that avoids solving a full 52D min-cost flow and still preserves the
    // lexicographic optimum when such stronger restriction does not exclude optimal solutions.
    // To avoid over-restricting, we instead enforce exact net-zero change for earlier characters
    // by tracking a 52D vector would be too heavy. So we choose a different faithful approach:
    //
    // Use weighted scalar costs with a sufficiently large base to encode lexicographic order exactly:
    // minimize sum_k (-c_k) * B^(51-k) for B > N.
    // This exactly matches lexicographic minimization because each component differs by at most N.
    // We can then solve a standard min-cost assignment with integer costs.
    //
    // Choose B = N+1, and cost of selecting character k is: w_k = -B^(51-k).
    // Total cost is minimized => maximize lexicographically c_k.
    //
    // Implement Hungarian algorithm for rectangular NxN with i64 costs.
    // Complexity O(N^3), fine for small N as assumed.
    //
    // Note: Costs can be large: B^(51) may overflow i64 for moderate N.
    // To preserve semantics without assuming bounds, we instead use arbitrary precision integers
    // for costs. Rust std has no BigInt; implement lexicographic comparison directly in Hungarian
    // is complex.
    //
    // Therefore, implement min-cost max-flow with 52D cost vectors and lexicographic shortest paths
    // using potentials and Dijkstra-like comparisons on vectors. This avoids big integers and matches
    // the specification directly.
    //
    // Graph:
    // s -> row_i (cap 1)
    // row_i -> col_j (cap 1, cost vector = -e_{grid[i][j]})
    // col_j -> t (cap 1)
    //
    // Successive shortest augmenting path with lexicographic costs:
    // Each augmentation sends 1 unit; total N.
    // Use Bellman-Ford-like relaxation with lexicographic vector distances (52 i32s).
    // N is assumed small; O(V*E*N) acceptable.
    //
    // Tie-breaking is left to internal relaxation order, matching spec allowance.

    #[derive(Clone)]
    struct Edge {
        to: usize,
        rev: usize,
        cap: i32,
        cost: [i32; 52],
    }

    fn add_edge(g: &mut Vec<Vec<Edge>>, fr: usize, to: usize, cap: i32, cost: [i32; 52]) {
        let rev_to = g[to].len();
        let rev_fr = g[fr].len();
        g[fr].push(Edge {
            to,
            rev: rev_to,
            cap,
            cost,
        });
        let mut rcost = [0i32; 52];
        for i in 0..52 {
            rcost[i] = -cost[i];
        }
        g[to].push(Edge {
            to: fr,
            rev: rev_fr,
            cap: 0,
            cost: rcost,
        });
    }

    fn lex_lt(a: &[i32; 52], b: &[i32; 52]) -> bool {
        for i in 0..52 {
            if a[i] != b[i] {
                return a[i] < b[i];
            }
        }
        false
    }

    fn lex_add(a: &[i32; 52], b: &[i32; 52]) -> [i32; 52] {
        let mut r = [0i32; 52];
        for i in 0..52 {
            r[i] = a[i] + b[i];
        }
        r
    }

    let v = 2 + n + n;
    let s = 0usize;
    let t = v - 1;
    let row_base = 1usize;
    let col_base = 1usize + n;

    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); v];

    for i in 0..n {
        add_edge(&mut g, s, row_base + i, 1, [0i32; 52]);
    }
    for j in 0..n {
        add_edge(&mut g, col_base + j, t, 1, [0i32; 52]);
    }
    for i in 0..n {
        for j in 0..n {
            let mut cost = [0i32; 52];
            cost[grid[i][j]] = -1;
            add_edge(&mut g, row_base + i, col_base + j, 1, cost);
        }
    }

    let mut flow = 0i32;
    // Track which row->col edges are used to reconstruct counts.
    // We'll infer from residual capacities after flow.
    while flow < n as i32 {
        let mut dist: Vec<Option<[i32; 52]>> = vec![None; v];
        let mut inq = vec![false; v];
        let mut prev_v = vec![usize::MAX; v];
        let mut prev_e = vec![usize::MAX; v];

        dist[s] = Some([0i32; 52]);
        let mut q = std::collections::VecDeque::new();
        q.push_back(s);
        inq[s] = true;

        while let Some(u) = q.pop_front() {
            inq[u] = false;
            let du = dist[u].unwrap();
            for (ei, e) in g[u].iter().enumerate() {
                if e.cap <= 0 {
                    continue;
                }
                let nd = lex_add(&du, &e.cost);
                match dist[e.to] {
                    None => {
                        dist[e.to] = Some(nd);
                        prev_v[e.to] = u;
                        prev_e[e.to] = ei;
                        if !inq[e.to] {
                            q.push_back(e.to);
                            inq[e.to] = true;
                        }
                    }
                    Some(cur) => {
                        if lex_lt(&nd, &cur) {
                            dist[e.to] = Some(nd);
                            prev_v[e.to] = u;
                            prev_e[e.to] = ei;
                            if !inq[e.to] {
                                q.push_back(e.to);
                                inq[e.to] = true;
                            }
                        }
                    }
                }
            }
        }

        // Assumption: perfect matching always feasible.
        if dist[t].is_none() {
            break;
        }

        // Augment 1 unit.
        let mut vtx = t;
        while vtx != s {
            let pv = prev_v[vtx];
            let pe = prev_e[vtx];
            let rev = g[pv][pe].rev;
            g[pv][pe].cap -= 1;
            g[vtx][rev].cap += 1;
            vtx = pv;
        }
        flow += 1;
    }

    // Count selected characters from saturated row->col edges (cap==0 on forward edge).
    let mut counts = [0usize; 52];
    for i in 0..n {
        let u = row_base + i;
        for e in g[u].iter() {
            if e.to >= col_base && e.to < col_base + n {
                // Original capacity was 1; if now 0, it's used.
                if e.cap == 0 {
                    // cost has -1 at selected char index.
                    for k in 0..52 {
                        if e.cost[k] == -1 {
                            counts[k] += 1;
                            break;
                        }
                    }
                }
            }
        }
    }

    let mut out = String::with_capacity(n);
    for k in 0..52 {
        for _ in 0..counts[k] {
            out.push(ALPHABET[k] as char);
        }
    }
    println!("{out}");
}
