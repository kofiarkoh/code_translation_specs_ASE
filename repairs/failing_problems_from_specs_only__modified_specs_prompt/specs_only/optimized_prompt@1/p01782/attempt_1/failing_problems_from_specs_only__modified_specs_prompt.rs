use std::cmp::Ordering;
use std::io::{self, Read};

const K: usize = 52;
const INF_COMP: i8 = 60;

#[derive(Clone, Copy)]
struct Vec52 {
    v: [i8; K],
}

impl Vec52 {
    fn zero() -> Self {
        Self { v: [0i8; K] }
    }
    fn inf() -> Self {
        Self { v: [INF_COMP; K] }
    }
    fn add(self, other: Self) -> Self {
        let mut out = self;
        for i in 0..K {
            // Specification: signed 8-bit arithmetic; behavior unspecified on overflow.
            // Rust i8 addition in release wraps; in debug it panics. Use wrapping to emulate 8-bit storage.
            out.v[i] = out.v[i].wrapping_add(other.v[i]);
        }
        out
    }
    fn sub(self, other: Self) -> Self {
        let mut out = self;
        for i in 0..K {
            out.v[i] = out.v[i].wrapping_sub(other.v[i]);
        }
        out
    }
    fn cmp_lex(&self, other: &Self) -> Ordering {
        for i in 0..K {
            if self.v[i] < other.v[i] {
                return Ordering::Less;
            } else if self.v[i] > other.v[i] {
                return Ordering::Greater;
            }
        }
        Ordering::Equal
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: Vec52,
}

struct MCMF {
    g: Vec<Vec<Edge>>,
}

impl MCMF {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, fr: usize, to: usize, cap: i32, cost: Vec52) {
        let rev_to = self.g[to].len();
        let rev_fr = self.g[fr].len();
        self.g[fr].push(Edge {
            to,
            rev: rev_to,
            cap,
            cost,
        });
        self.g[to].push(Edge {
            to: fr,
            rev: rev_fr,
            cap: 0,
            cost: Vec52::zero().sub(cost), // reverse edge cost = -cost
        });
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> Vec52 {
        let n = self.g.len();
        let mut total = Vec52::zero();

        while f > 0 {
            let mut dist = vec![Vec52::inf(); n];
            let mut inq = vec![false; n];
            let mut prev_v = vec![usize::MAX; n];
            let mut prev_e = vec![usize::MAX; n];

            dist[s] = Vec52::zero();
            let mut q = std::collections::VecDeque::new();
            q.push_back(s);
            inq[s] = true;

            while let Some(v) = q.pop_front() {
                inq[v] = false;
                let dv = dist[v];
                for (ei, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = dv.add(e.cost);
                    if nd.cmp_lex(&dist[e.to]) == Ordering::Less {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = ei;
                        if !inq[e.to] {
                            inq[e.to] = true;
                            q.push_back(e.to);
                        }
                    }
                }
            }

            // If unreachable, behavior is unspecified by the spec; we stop.
            if prev_v[t] == usize::MAX {
                break;
            }

            let mut addf = f;
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let cap = self.g[pv][pe].cap;
                if cap < addf {
                    addf = cap;
                }
                v = pv;
            }

            // Apply augmentation
            v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let rev = self.g[pv][pe].rev;

                // Update costs: total += dist[t] * addf, but addf is always 1 here (unit capacities),
                // still implement multiplication by small integer componentwise via repeated add.
                // Specification mentions multiplication by small integer; keep narrow and safe.
                self.g[pv][pe].cap -= addf;
                self.g[v][rev].cap += addf;

                v = pv;
            }

            // add dist[t] * addf to total (componentwise i8 arithmetic)
            for _ in 0..addf {
                total = total.add(dist[t]);
            }

            f -= addf;
        }

        total
    }
}

fn letter_index(b: u8) -> Option<usize> {
    match b {
        b'A'..=b'Z' => Some((b - b'A') as usize),
        b'a'..=b'z' => Some(26 + (b - b'a') as usize),
        _ => None,
    }
}

fn index_letter(i: usize) -> u8 {
    if i < 26 {
        b'A' + (i as u8)
    } else {
        b'a' + ((i - 26) as u8)
    }
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let n: usize = match it.next() {
        Some(s) => s.trim().parse().unwrap(),
        None => return,
    };

    // Build grid using first N characters of each line (bytes)
    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = it.next().unwrap_or("");
        let bytes = line.as_bytes();
        let mut row = Vec::with_capacity(n);
        for j in 0..n {
            row.push(*bytes.get(j).unwrap_or(&0u8));
        }
        grid.push(row);
    }

    // Flow network:
    // source -> rows (cap 1, cost 0)
    // rows -> cols (cap 1, cost vector with -1 at letter)
    // cols -> sink (cap 1, cost 0)
    let vcnt = 2 * n + 2;
    let s = 2 * n;
    let t = 2 * n + 1;

    let mut mcmf = MCMF::new(vcnt);

    for i in 0..n {
        mcmf.add_edge(s, i, 1, Vec52::zero());
    }
    for j in 0..n {
        mcmf.add_edge(n + j, t, 1, Vec52::zero());
    }

    for i in 0..n {
        for j in 0..n {
            let ch = grid[i][j];
            let mut cost = Vec52::zero();
            if let Some(idx) = letter_index(ch) {
                cost.v[idx] = cost.v[idx].wrapping_sub(1); // add -1 to that component
            } else {
                // Spec: other characters not handled meaningfully; keep cost as zero.
            }
            mcmf.add_edge(i, n + j, 1, cost);
        }
    }

    let total_cost = mcmf.min_cost_flow(s, t, n as i32);

    // Output: for each letter in fixed order, print it -c times where c is component.
    // Under intended constraints, this prints exactly N letters.
    let mut out = Vec::new();
    for i in 0..K {
        let c = total_cost.v[i];
        // c is i8; -c in i8 may overflow for -128, but spec says behavior unspecified if out of range.
        // Use wrapping_neg to emulate 8-bit storage.
        let times_i8 = c.wrapping_neg();
        let times = times_i8 as i16; // interpret as signed count
        if times > 0 {
            let ch = index_letter(i);
            for _ in 0..times {
                out.push(ch);
            }
        }
    }
    out.push(b'\n');
    print!("{}", String::from_utf8_lossy(&out));
}
