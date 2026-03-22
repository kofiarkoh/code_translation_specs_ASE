use std::io::{self, Read};

fn char_index(b: u8) -> usize {
    match b {
        b'A'..=b'Z' => (b - b'A') as usize,
        b'a'..=b'z' => 26 + (b - b'a') as usize,
        _ => 0, // Unspecified for invalid input; keep total.
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: i64,
}

struct Mcmf {
    g: Vec<Vec<Edge>>,
}

impl Mcmf {
    fn new(n: usize) -> Self {
        Self { g: vec![Vec::new(); n] }
    }

    fn add_edge(&mut self, fr: usize, to: usize, cap: i32, cost: i64) {
        let rev_to = self.g[to].len();
        let rev_fr = self.g[fr].len();
        self.g[fr].push(Edge { to, rev: rev_to, cap, cost });
        self.g[to].push(Edge { to: fr, rev: rev_fr, cap: 0, cost: -cost });
    }

    fn min_cost_flow(&mut self, s: usize, t: usize, mut f: i32) -> i64 {
        // Successive shortest augmenting path with potentials.
        // Important: initial potentials must be feasible even with negative costs.
        // We compute them via Bellman-Ford/SPFA from s.
        let n = self.g.len();
        let mut h = vec![0i64; n];

        // SPFA for initial potentials (shortest distances from s in original costs).
        {
            let inf = i64::MAX / 4;
            let mut dist = vec![inf; n];
            let mut inq = vec![false; n];
            let mut q = std::collections::VecDeque::new();
            dist[s] = 0;
            q.push_back(s);
            inq[s] = true;
            while let Some(v) = q.pop_front() {
                inq[v] = false;
                let dv = dist[v];
                if dv >= inf / 2 {
                    continue;
                }
                for e in &self.g[v] {
                    if e.cap <= 0 {
                        continue;
                    }
                    let nd = dv + e.cost;
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        if !inq[e.to] {
                            inq[e.to] = true;
                            q.push_back(e.to);
                        }
                    }
                }
            }
            for i in 0..n {
                if dist[i] < inf / 2 {
                    h[i] = dist[i];
                }
            }
        }

        let mut res = 0i64;
        let inf = i64::MAX / 4;
        let mut dist = vec![0i64; n];
        let mut prev_v = vec![0usize; n];
        let mut prev_e = vec![0usize; n];

        while f > 0 {
            dist.fill(inf);
            dist[s] = 0;

            // Dijkstra on reduced costs (non-negative if potentials are feasible).
            let mut used = vec![false; n];
            for _ in 0..n {
                let mut v = None;
                let mut best = inf;
                for i in 0..n {
                    if !used[i] && dist[i] < best {
                        best = dist[i];
                        v = Some(i);
                    }
                }
                let v = match v {
                    Some(v) => v,
                    None => break,
                };
                used[v] = true;
                for (ei, e) in self.g[v].iter().enumerate() {
                    if e.cap <= 0 {
                        continue;
                    }
                    let rcost = e.cost + h[v] - h[e.to];
                    let nd = dist[v] + rcost;
                    if nd < dist[e.to] {
                        dist[e.to] = nd;
                        prev_v[e.to] = v;
                        prev_e[e.to] = ei;
                    }
                }
            }

            if dist[t] >= inf / 2 {
                break; // Unspecified if infeasible.
            }

            for v in 0..n {
                if dist[v] < inf / 2 {
                    h[v] += dist[v];
                }
            }

            let mut d = f;
            let mut v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                d = d.min(self.g[pv][pe].cap);
                v = pv;
            }

            f -= d;
            res += h[t] * (d as i64);

            v = t;
            while v != s {
                let pv = prev_v[v];
                let pe = prev_e[v];
                let rev = self.g[pv][pe].rev;
                self.g[pv][pe].cap -= d;
                self.g[v][rev].cap += d;
                v = pv;
            }
        }

        res
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut grid = vec![vec![0usize; n]; n];
    for i in 0..n {
        let s = it.next().unwrap();
        let b = s.as_bytes();
        for j in 0..n {
            grid[i][j] = char_index(b[j]);
        }
    }

    // Lexicographic minimization of (-c0..-c51) == lexicographic maximization of (c0..c51).
    // Encode as scalar min-cost with base B=n+1:
    // cost(k) = -B^(51-k). Then total cost is a base-B number whose digits are (-c0..-c51),
    // so minimizing it matches the required lexicographic order.
    let base: i64 = (n as i64) + 1;
    let mut pow = vec![1i64; 52];
    for i in 1..52 {
        pow[i] = pow[i - 1] * base;
    }
    let mut weight = [0i64; 52];
    for k in 0..52 {
        weight[k] = -pow[51 - k];
    }

    let s = 2 * n;
    let t = 2 * n + 1;
    let mut mcmf = Mcmf::new(2 * n + 2);

    for i in 0..n {
        mcmf.add_edge(s, i, 1, 0);
    }
    for j in 0..n {
        mcmf.add_edge(n + j, t, 1, 0);
    }
    for i in 0..n {
        for j in 0..n {
            let k = grid[i][j];
            mcmf.add_edge(i, n + j, 1, weight[k]);
        }
    }

    let _ = mcmf.min_cost_flow(s, t, n as i32);

    // Extract chosen edges: row->col edges with cap==0 (used).
    let mut counts = [0usize; 52];
    for i in 0..n {
        for e in &mcmf.g[i] {
            if e.to >= n && e.to < 2 * n && e.cap == 0 {
                let j = e.to - n;
                let k = grid[i][j];
                counts[k] += 1;
                break;
            }
        }
    }

    let mut out = String::with_capacity(n + 1);
    for idx in 0..52 {
        let ch = if idx < 26 {
            (b'A' + idx as u8) as char
        } else {
            (b'a' + (idx as u8 - 26)) as char
        };
        for _ in 0..counts[idx] {
            out.push(ch);
        }
    }
    out.push('\n');
    print!("{out}");
}
