use std::io::{self, Read};

struct Dsu {
    // If parent_or_size[x] < 0, x is a root and -parent_or_size[x] is the set size.
    // Otherwise parent_or_size[x] is the parent index (>= 1).
    parent_or_size: Vec<i32>,
}

impl Dsu {
    fn new(n: usize) -> Self {
        // 1-based indexing; index 0 unused.
        let mut v = vec![0i32; n + 1];
        for i in 1..=n {
            v[i] = -1;
        }
        Self { parent_or_size: v }
    }

    fn find(&mut self, mut x: usize) -> usize {
        // Safe path compression without relying on a fixed stack depth.
        let mut path: Vec<usize> = Vec::new();
        while self.parent_or_size[x] >= 1 {
            path.push(x);
            x = self.parent_or_size[x] as usize;
        }
        for node in path {
            self.parent_or_size[node] = x as i32;
        }
        x
    }

    fn unite(&mut self, x: usize, y: usize) {
        let mut rx = self.find(x);
        let mut ry = self.find(y);
        if rx == ry {
            return;
        }
        // Union by size: more negative means larger.
        if self.parent_or_size[rx] < self.parent_or_size[ry] {
            // rx is larger
            self.parent_or_size[rx] += self.parent_or_size[ry];
            self.parent_or_size[ry] = rx as i32;
        } else {
            self.parent_or_size[ry] += self.parent_or_size[rx];
            self.parent_or_size[rx] = ry as i32;
        }
    }

    fn set_size(&mut self, x: usize) -> i32 {
        let r = self.find(x);
        -self.parent_or_size[r]
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut m: i32 = it.next().unwrap().parse().unwrap();
    let mut k: i32 = it.next().unwrap().parse().unwrap();

    let mut dsu = Dsu::new(n);

    // rmv[i] starts at 1 (exclude itself)
    let mut rmv = vec![0i32; n + 1];
    for i in 1..=n {
        rmv[i] = 1;
    }

    // Process connections: union (if different components) and increment direct-connection counts
    while m > 0 {
        m -= 1;
        let a: usize = it.next().unwrap().parse().unwrap();
        let b: usize = it.next().unwrap().parse().unwrap();

        // C code checks roots before calling unite; unite itself also handles same-root safely.
        if dsu.find(a) != dsu.find(b) {
            dsu.unite(a, b);
        }
        rmv[a] += 1;
        rmv[b] += 1;
    }

    // Process incompatible pairs: only count if within same connected component
    while k > 0 {
        k -= 1;
        let c: usize = it.next().unwrap().parse().unwrap();
        let d: usize = it.next().unwrap().parse().unwrap();

        if dsu.find(c) == dsu.find(d) {
            rmv[c] += 1;
            rmv[d] += 1;
        }
    }

    // Output: |C(i)| - rmv[i]
    let mut out = String::new();
    for i in 1..=n {
        let val = dsu.set_size(i) - rmv[i];
        if i == n {
            out.push_str(&format!("{val}\n"));
        } else {
            out.push_str(&format!("{val} "));
        }
    }
    print!("{out}");
}
