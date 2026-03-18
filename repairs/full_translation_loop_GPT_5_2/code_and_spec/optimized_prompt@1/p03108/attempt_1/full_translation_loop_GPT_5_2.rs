use std::io::{self, Read};

struct UnionFind {
    par: Vec<usize>,
    size: Vec<i64>, // component sizes (only meaningful at roots)
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut par = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            par.push(i);
            size.push(1);
        }
        Self { par, size }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        let r = self.root(self.par[x]);
        self.par[x] = r;
        r
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return false;
        }
        // Match C semantics: attach rx under ry (no union by size/rank).
        self.par[rx] = ry;
        self.size[ry] += self.size[rx];
        self.size[rx] = 0;
        true
    }

    fn comp_size(&mut self, x: usize) -> i64 {
        let r = self.root(x);
        self.size[r]
    }
}

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;
    let m: usize = it.next().unwrap().parse::<i64>().unwrap() as usize;

    let mut a = vec![0usize; m];
    let mut b = vec![0usize; m];
    for i in 0..m {
        let ai: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        let bi: usize = it.next().unwrap().parse::<usize>().unwrap() - 1;
        a[i] = ai;
        b[i] = bi;
    }

    // Per spec: compute number of connected pairs after each edge addition,
    // output in reverse order of input edges.
    //
    // We implement the standard reverse-addition approach:
    // Start with no edges: disconnected pairs = n*(n-1)/2.
    // Add edges from last to first; before adding edge i, record current value.
    // If it connects two different components of sizes s and t, disconnected pairs decreases by s*t.
    let mut uf = UnionFind::new(n);
    let mut ans = vec![0i64; m];

    let mut disconnected: i64 = (n as i64) * ((n as i64) - 1) / 2;

    for idx in (0..m).rev() {
        ans[idx] = disconnected;
        if !uf.same(a[idx], b[idx]) {
            let sa = uf.comp_size(a[idx]);
            let sb = uf.comp_size(b[idx]);
            disconnected -= sa * sb;
            uf.unite(a[idx], b[idx]);
        }
    }

    // Note: The provided C code has out-of-bounds writes (ans[i-1] when i==0)
    // and special-cases that do not match the specification. We follow the spec.

    let mut out = String::new();
    for i in 0..m {
        out.push_str(&format!("{}\n", ans[i]));
    }
    print!("{out}");
}
