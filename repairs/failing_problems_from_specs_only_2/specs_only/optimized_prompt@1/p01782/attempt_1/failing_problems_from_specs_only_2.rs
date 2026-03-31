use std::cmp::Ordering;
use std::collections::VecDeque;
use std::io::{self, Read};

const K: usize = 52;

fn char_index(b: u8) -> usize {
    match b {
        b'A'..=b'Z' => (b - b'A') as usize,
        b'a'..=b'z' => 26 + (b - b'a') as usize,
        _ => 0, // Unspecified behavior for invalid input; choose a narrow default.
    }
}

fn index_char(i: usize) -> u8 {
    if i < 26 {
        b'A' + i as u8
    } else {
        b'a' + (i as u8 - 26)
    }
}

#[derive(Clone, Copy, Debug)]
struct Cost([i32; K]);

impl Cost {
    fn zero() -> Self {
        Cost([0; K])
    }
    fn add(&self, other: &Cost) -> Cost {
        let mut out = [0i32; K];
        for i in 0..K {
            out[i] = self.0[i] + other.0[i];
        }
        Cost(out)
    }
    fn sub(&self, other: &Cost) -> Cost {
        let mut out = [0i32; K];
        for i in 0..K {
            out[i] = self.0[i] - other.0[i];
        }
        Cost(out)
    }
}

impl PartialEq for Cost {
    fn eq(&self, other: &Self) -> bool {
        self.0 == other.0
    }
}
impl Eq for Cost {}

impl PartialOrd for Cost {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl Ord for Cost {
    fn cmp(&self, other: &Self) -> Ordering {
        // Lexicographic compare by component order 0..51.
        for i in 0..K {
            if self.0[i] < other.0[i] {
                return Ordering::Less;
            } else if self.0[i] > other.0[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
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
    // Specification: each line has at least N usable characters; extra ignored.
    let mut grid_idx = vec![vec![0usize; n]; n];
    for i in 0..n {
        let tok = it.next().unwrap_or("");
        let bytes = tok.as_bytes();
        for j in 0..n {
            grid_idx[i][j] = char_index(bytes[j]);
        }
    }

    // Build per-edge cost vectors: selecting character k contributes -1 at k.
    let mut edge_cost = vec![vec![Cost::zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut v = [0i32; K];
            v[grid_idx[i][j]] = -1;
            edge_cost[i][j] = Cost(v);
        }
    }

    // Min-cost perfect matching in complete bipartite graph (rows -> cols),
    // with lexicographic 52D integer costs.
    //
    // Use successive shortest augmenting path on residual graph with potentials,
    // where "distance" is a Cost vector and comparisons are lexicographic.
    //
    // Nodes:
    //   0..n-1: rows
    //   n..2n-1: cols
    // Source S = 2n, Sink T = 2n+1
    let s = 2 * n;
    let t = 2 * n + 1;
    let vcnt = 2 * n + 2;

    // Matching representation: row_match[i] = col, col_match[j] = row.
    let mut row_match = vec![None; n];
    let mut col_match = vec![None; n];

    // Potentials for reduced costs (Johnson-style), as Cost vectors.
    let mut pot = vec![Cost::zero(); vcnt];

    // Helper to get reduced cost of an arc in residual graph.
    // Residual arcs:
    //   S -> row i (cap 1, cost 0)
    //   row i -> col j (cap 1 if unmatched edge, cost edge_cost[i][j])
    //   col j -> row i (cap 1 if matched edge, cost -edge_cost[i][j])
    //   col j -> T (cap 1, cost 0)
    //
    // Reduced cost: c + pot[u] - pot[v]
    let zero = Cost::zero();

    for _aug in 0..n {
        // Dijkstra-like over lexicographic costs; since reduced costs are intended non-negative
        // under potentials, but we keep it generic and still use a simple O(V^2) selection.
        let mut dist = vec![None::<Cost>; vcnt];
        let mut prev = vec![None::<(usize, usize)>; vcnt];
        // prev[v] = (u, kind) where kind encodes which residual arc used:
        // kind:
        // 0: S->row i (u=S, v=row)
        // 1: row i -> col j (u=row, v=col)
        // 2: col j -> row i (u=col, v=row) (matched back edge)
        // 3: col j -> T (u=col, v=T)
        let mut used = vec![false; vcnt];

        dist[s] = Some(Cost::zero());

        for _ in 0..vcnt {
            // Pick unused vertex with smallest dist.
            let mut best_v: Option<usize> = None;
            let mut best_d: Option<Cost> = None;
            for v in 0..vcnt {
                if used[v] {
                    continue;
                }
                if let Some(dv) = dist[v] {
                    if best_v.is_none() || dv < best_d.unwrap() {
                        best_v = Some(v);
                        best_d = Some(dv);
                    }
                }
            }
            let u = match best_v {
                Some(v) => v,
                None => break,
            };
            used[u] = true;
            if u == t {
                break;
            }
            let du = dist[u].unwrap();

            // Relax outgoing residual arcs from u.
            if u == s {
                // S -> row i if row i is unmatched from S (always cap 1, but if row already matched,
                // it still has residual cap from S? In standard flow, S->row has cap 1 and flow 1 once matched,
                // so residual cap is 0. We emulate that: only unmatched rows have residual from S.)
                for i in 0..n {
                    if row_match[i].is_some() {
                        continue;
                    }
                    let v = i;
                    let rc = zero.add(&pot[u]).sub(&pot[v]);
                    let nd = du.add(&rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 0));
                    }
                }
            } else if u < n {
                // row i
                let i = u;
                // row -> col edges: if row i is unmatched, it can go to any col with residual cap 1.
                // If row i is matched, in residual graph it still has forward edges with cap 1? In assignment flow,
                // row->col edges have cap 1 and flow 1 only on matched edge; so forward residual exists for all
                // non-matched edges, and for matched edge forward residual is 0.
                for j in 0..n {
                    if row_match[i] == Some(j) {
                        continue;
                    }
                    let v = n + j;
                    let c = edge_cost[i][j];
                    let rc = c.add(&pot[u]).sub(&pot[v]);
                    let nd = du.add(&rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 1));
                    }
                }
                // Also, if row i is matched to some col, there is no direct residual to S in our encoding.
            } else if u < 2 * n {
                // col j
                let j = u - n;
                // col -> T if col is unmatched (cap 1, flow 0), else residual 0.
                if col_match[j].is_none() {
                    let v = t;
                    let rc = zero.add(&pot[u]).sub(&pot[v]);
                    let nd = du.add(&rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 3));
                    }
                }
                // col -> row back edge if col is matched to some row i (residual cap 1, cost -edge_cost[i][j]).
                if let Some(i) = col_match[j] {
                    let v = i;
                    let c = Cost::zero().sub(&edge_cost[i][j]);
                    let rc = c.add(&pot[u]).sub(&pot[v]);
                    let nd = du.add(&rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 2));
                    }
                }
            } else {
                // t has no outgoing arcs.
            }
        }

        // Must be able to reach t (perfect matching always feasible per spec).
        let dt = dist[t].unwrap();

        // Update potentials: pot[v] += dist[v] for all reachable v.
        for v in 0..vcnt {
            if let Some(dv) = dist[v] {
                pot[v] = pot[v].add(&dv);
            }
        }

        // Augment along path from t to s using prev.
        // We need to flip matched/unmatched edges along alternating path.
        let mut cur = t;
        // Collect path nodes in reverse to process edges.
        let mut path: Vec<(usize, usize)> = Vec::new();
        while cur != s {
            let (p, kind) = prev[cur].unwrap();
            path.push((cur, kind));
            cur = p;
        }
        // Now traverse path in reverse (from s to t) and apply residual edge augmentations.
        // We'll interpret edges based on (to_node, kind) and current predecessor.
        cur = s;
        for &(to, kind) in path.iter().rev() {
            match kind {
                0 => {
                    // S -> row i : nothing to store
                    cur = to;
                }
                1 => {
                    // row i -> col j : set tentative along path by recording in a queue
                    cur = to;
                }
                2 => {
                    // col j -> row i (matched back edge): nothing immediate
                    cur = to;
                }
                3 => {
                    // col j -> T : nothing
                    cur = to;
                }
                _ => {}
            }
        }

        // Apply augmentation by walking again and flipping matches.
        // Standard approach: follow alternating path and update matches when traversing row->col forward edges.
        // We'll reconstruct explicit sequence of row/col nodes from prev.
        let mut seq: Vec<usize> = Vec::new();
        let mut v = t;
        seq.push(v);
        while v != s {
            let (p, _) = prev[v].unwrap();
            v = p;
            seq.push(v);
        }
        seq.reverse(); // s ... t

        // seq alternates: s, row, col, row, col, ..., t
        // We flip along each row->col step (row->col forward) and each col->row step (back edge).
        // Implement by tracking current "next assignment" for rows encountered.
        let mut i = 1;
        while i + 1 < seq.len() {
            let a = seq[i];
            let b = seq[i + 1];
            if a < n && (n..2 * n).contains(&b) {
                let row = a;
                let col = b - n;
                // This edge becomes matched.
                // If row was previously matched, it must have been freed earlier in the alternating path.
                row_match[row] = Some(col);
                col_match[col] = Some(row);
            } else if (n..2 * n).contains(&a) && b < n {
                // This is a matched edge being removed (col->row back edge).
                let col = a - n;
                let row = b;
                // Remove match between row and col.
                // It should currently be matched.
                if row_match[row] == Some(col) {
                    row_match[row] = None;
                }
                if col_match[col] == Some(row) {
                    col_match[col] = None;
                }
            }
            i += 1;
        }

        // dt is unused directly; potentials drive lexicographic optimality.
        let _ = dt;
    }

    // Count selected characters from final matching.
    let mut counts = [0usize; K];
    for i in 0..n {
        let j = row_match[i].unwrap();
        counts[grid_idx[i][j]] += 1;
    }

    // Output sorted multiset in fixed order.
    let mut out: Vec<u8> = Vec::with_capacity(n + 1);
    for k in 0..K {
        for _ in 0..counts[k] {
            out.push(index_char(k));
        }
    }
    out.push(b'\n');
    print!("{}", String::from_utf8_lossy(&out));
}
