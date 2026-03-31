use std::cmp::Ordering;
use std::io::{self, Read};

const K: usize = 52;

fn char_index(b: u8) -> usize {
    match b {
        b'A'..=b'Z' => (b - b'A') as usize,
        b'a'..=b'z' => 26 + (b - b'a') as usize,
        _ => 0, // Unspecified for invalid input; narrow default.
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
    fn add(self, other: Cost) -> Cost {
        let mut out = [0i32; K];
        for i in 0..K {
            out[i] = self.0[i] + other.0[i];
        }
        Cost(out)
    }
    fn sub(self, other: Cost) -> Cost {
        let mut out = [0i32; K];
        for i in 0..K {
            out[i] = self.0[i] - other.0[i];
        }
        Cost(out)
    }
    fn neg(self) -> Cost {
        let mut out = [0i32; K];
        for i in 0..K {
            out[i] = -self.0[i];
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
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(s) => s.parse().unwrap(),
        None => return,
    };

    let mut grid_idx = vec![vec![0usize; n]; n];
    for i in 0..n {
        let tok = it.next().unwrap_or("");
        let bytes = tok.as_bytes();
        for j in 0..n {
            grid_idx[i][j] = char_index(bytes[j]);
        }
    }

    let mut edge_cost: Vec<Vec<Cost>> = vec![vec![Cost::zero(); n]; n];
    for i in 0..n {
        for j in 0..n {
            let mut v = [0i32; K];
            v[grid_idx[i][j]] = -1;
            edge_cost[i][j] = Cost(v);
        }
    }

    // Successive shortest augmenting path on assignment residual graph with lexicographic costs.
    let s = 2 * n;
    let t = 2 * n + 1;
    let vcnt = 2 * n + 2;

    let mut row_match: Vec<Option<usize>> = vec![None; n];
    let mut col_match: Vec<Option<usize>> = vec![None; n];

    let mut pot: Vec<Cost> = vec![Cost::zero(); vcnt];
    let zero = Cost::zero();

    for _ in 0..n {
        let mut dist: Vec<Option<Cost>> = vec![None; vcnt];
        let mut prev: Vec<Option<(usize, u8)>> = vec![None; vcnt];
        let mut used: Vec<bool> = vec![false; vcnt];

        dist[s] = Some(Cost::zero());

        for _ in 0..vcnt {
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

            if u == s {
                // S -> row i for unmatched rows.
                for i in 0..n {
                    if row_match[i].is_some() {
                        continue;
                    }
                    let v = i;
                    let rc = zero.add(pot[u]).sub(pot[v]);
                    let nd = du.add(rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 0));
                    }
                }
            } else if u < n {
                // row i -> col j for all j except currently matched edge (if any).
                let i = u;
                for j in 0..n {
                    if row_match[i] == Some(j) {
                        continue;
                    }
                    let v = n + j;
                    let c = edge_cost[i][j];
                    let rc = c.add(pot[u]).sub(pot[v]);
                    let nd = du.add(rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 1));
                    }
                }
            } else if u < 2 * n {
                // col j
                let j = u - n;

                // col j -> T if unmatched.
                if col_match[j].is_none() {
                    let v = t;
                    let rc = zero.add(pot[u]).sub(pot[v]);
                    let nd = du.add(rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 3));
                    }
                }

                // col j -> row i back edge if matched.
                if let Some(i) = col_match[j] {
                    let v = i;
                    let c = edge_cost[i][j].neg();
                    let rc = c.add(pot[u]).sub(pot[v]);
                    let nd = du.add(rc);
                    if dist[v].is_none() || nd < dist[v].unwrap() {
                        dist[v] = Some(nd);
                        prev[v] = Some((u, 2));
                    }
                }
            }
        }

        // Perfect matching is feasible per spec.
        let _dt: Cost = dist[t].unwrap();

        // Update potentials for all reachable vertices.
        for v in 0..vcnt {
            if let Some(dv) = dist[v] {
                pot[v] = pot[v].add(dv);
            }
        }

        // Reconstruct node sequence s..t.
        let mut seq: Vec<usize> = Vec::new();
        let mut v = t;
        seq.push(v);
        while v != s {
            let (p, _) = prev[v].unwrap();
            v = p;
            seq.push(v);
        }
        seq.reverse();

        // Flip along alternating path.
        // seq alternates: s, row, col, row, col, ..., t
        let mut idx = 1;
        while idx + 1 < seq.len() {
            let a = seq[idx];
            let b = seq[idx + 1];
            if a < n && (n..2 * n).contains(&b) {
                // row -> col becomes matched
                let row = a;
                let col = b - n;
                row_match[row] = Some(col);
                col_match[col] = Some(row);
            } else if (n..2 * n).contains(&a) && b < n {
                // col -> row back edge removes existing match
                let col = a - n;
                let row = b;
                if row_match[row] == Some(col) {
                    row_match[row] = None;
                }
                if col_match[col] == Some(row) {
                    col_match[col] = None;
                }
            }
            idx += 1;
        }
    }

    let mut counts = [0usize; K];
    for i in 0..n {
        let j = row_match[i].unwrap();
        counts[grid_idx[i][j]] += 1;
    }

    let mut out: Vec<u8> = Vec::with_capacity(n + 1);
    for k in 0..K {
        for _ in 0..counts[k] {
            out.push(index_char(k));
        }
    }
    out.push(b'\n');
    print!("{}", String::from_utf8_lossy(&out));
}
