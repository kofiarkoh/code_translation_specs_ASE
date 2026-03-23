use std::io::{self, Read};

const DIM: usize = 52;

#[derive(Clone, Copy)]
struct Vec52 {
    a: [i16; DIM], // signed char in C; i16 is safe for all operations here
}

impl Vec52 {
    fn zero() -> Self {
        Self { a: [0; DIM] }
    }
    fn inf() -> Self {
        // C sets each component to 60.
        let mut v = Self::zero();
        for x in &mut v.a {
            *x = 60;
        }
        v
    }
    fn add(&self, other: &Self) -> Self {
        let mut r = Self::zero();
        for i in 0..DIM {
            r.a[i] = self.a[i] + other.a[i];
        }
        r
    }
    fn sub(&self, other: &Self) -> Self {
        let mut r = Self::zero();
        for i in 0..DIM {
            r.a[i] = self.a[i] - other.a[i];
        }
        r
    }
    fn mul(&self, k: i16) -> Self {
        let mut r = Self::zero();
        for i in 0..DIM {
            r.a[i] = self.a[i] * k;
        }
        r
    }
    fn cmp_lex(&self, other: &Self) -> i16 {
        // Returns first non-zero component difference (self - other), like C.
        for i in 0..DIM {
            let d = self.a[i] - other.a[i];
            if d != 0 {
                return d;
            }
        }
        0
    }
}

#[derive(Clone)]
struct Edge {
    to: usize,
    rev: usize,
    cap: i32,
    cost: Vec52,
    rcost: Vec52,
}

fn add_edge(g: &mut Vec<Vec<Edge>>, from: usize, to: usize, cap: i32, cost: Vec52, rcost: Vec52) {
    let f_rev = g[to].len();
    let t_rev = g[from].len();
    g[from].push(Edge {
        to,
        rev: f_rev,
        cap,
        cost,
        rcost: rcost, // not used on forward edge, but kept for symmetry
    });
    g[to].push(Edge {
        to: from,
        rev: t_rev,
        cap: 0,
        cost: rcost,
        rcost: cost, // not used on reverse edge
    });
}

fn min_cost_flow(g: &mut Vec<Vec<Edge>>, s: usize, t: usize, mut f: i32) -> Vec52 {
    let v = g.len();
    let mut dist = vec![Vec52::inf(); v];
    let mut prevv = vec![0usize; v];
    let mut preve = vec![0usize; v];

    let mut ret = Vec52::zero();

    while f > 0 {
        for d in &mut dist {
            *d = Vec52::inf();
        }
        dist[s] = Vec52::zero();

        // Bellman-Ford style relaxation until no update (as in C).
        loop {
            let mut updated = false;
            for vv in 0..v {
                if dist[vv].cmp_lex(&Vec52::inf()) == 0 {
                    continue;
                }
                // Iterate edges by index to allow storing preve.
                for ei in 0..g[vv].len() {
                    if g[vv][ei].cap <= 0 {
                        continue;
                    }
                    let nv = g[vv][ei].to;
                    let cand = dist[vv].add(&g[vv][ei].cost);
                    if dist[nv].cmp_lex(&cand) > 0 {
                        dist[nv] = cand;
                        prevv[nv] = vv;
                        preve[nv] = ei;
                        updated = true;
                    }
                }
            }
            if !updated {
                break;
            }
        }

        // Augment by minimum residual capacity along path.
        let mut d = f;
        let mut vv = t;
        while vv != s {
            let pv = prevv[vv];
            let ei = preve[vv];
            let cap = g[pv][ei].cap;
            if d > cap {
                d = cap;
            }
            vv = pv;
        }
        f -= d;

        ret = ret.add(&dist[t].mul(d as i16));

        // Apply augmentation.
        let mut vv2 = t;
        while vv2 != s {
            let pv = prevv[vv2];
            let ei = preve[vv2];
            let rev = g[pv][ei].rev;

            g[pv][ei].cap -= d;
            g[vv2][rev].cap += d;

            vv2 = pv;
        }
    }

    ret
}

fn char_index(c: u8) -> usize {
    if c < b'a' {
        (c - b'A') as usize
    } else {
        26 + (c - b'a') as usize
    }
}

fn main() {
    // Read entire stdin; behavior is unspecified on malformed input, per spec.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.lines();

    let n: usize = it.next().unwrap().trim().parse().unwrap();

    let mut grid: Vec<Vec<u8>> = Vec::with_capacity(n);
    for _ in 0..n {
        let line = it.next().unwrap_or("");
        let bytes = line.as_bytes();
        // Spec: each line has at least N usable characters; ignore beyond N.
        let mut row = Vec::with_capacity(n);
        row.extend_from_slice(&bytes[..n]);
        grid.push(row);
    }

    let source = n * 2;
    let sink = source + 1;
    let v = sink + 1;

    let mut g: Vec<Vec<Edge>> = vec![Vec::new(); v];

    for i in 0..n {
        add_edge(&mut g, source, i, 1, Vec52::zero(), Vec52::zero());
        for j in 0..n {
            let c = grid[i][j];
            let idx = char_index(c);
            let mut a = Vec52::zero();
            let mut b = Vec52::zero();
            a.a[idx] = -1;
            b.a[idx] = 1;
            add_edge(&mut g, i, n + j, 1, a, b);
        }
        add_edge(&mut g, n + i, sink, 1, Vec52::zero(), Vec52::zero());
    }

    let cost = min_cost_flow(&mut g, source, sink, n as i32);

    let mut out = String::with_capacity(n + 1);
    for i in 0..26 {
        let mut cnt = -cost.a[i] as i32;
        while cnt > 0 {
            out.push((b'A' + i as u8) as char);
            cnt -= 1;
        }
    }
    for i in 0..26 {
        let mut cnt = -cost.a[i + 26] as i32;
        while cnt > 0 {
            out.push((b'a' + i as u8) as char);
            cnt -= 1;
        }
    }
    out.push('\n');
    print!("{out}");
}
