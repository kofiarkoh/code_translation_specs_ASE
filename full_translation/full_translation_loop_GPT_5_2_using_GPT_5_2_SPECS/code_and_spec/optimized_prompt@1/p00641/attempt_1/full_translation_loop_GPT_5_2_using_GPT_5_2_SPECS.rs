use std::io::{self, Read};

const MOD: i32 = 10007;

struct DSU {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl DSU {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        for i in 0..n {
            parent.push(i);
            size.push(1);
        }
        Self { parent, size }
    }

    fn root(&mut self, mut x: usize) -> usize {
        // Path compression (iterative)
        while self.parent[x] != x {
            let p = self.parent[x];
            self.parent[x] = self.parent[p];
            x = self.parent[x];
        }
        x
    }

    fn unite(&mut self, a: usize, b: usize) {
        let mut ra = self.root(a);
        let mut rb = self.root(b);
        if ra == rb {
            return;
        }
        if self.size[ra] < self.size[rb] {
            std::mem::swap(&mut ra, &mut rb);
        }
        self.parent[rb] = ra;
        self.size[ra] += self.size[rb];
    }
}

fn check(comp: usize, val: i32, maxv: &mut [i32], cnt: &mut [i32]) {
    if cnt[comp] == 0 || val > maxv[comp] {
        cnt[comp] = 1;
        maxv[comp] = val;
    } else if val == maxv[comp] {
        cnt[comp] += 1;
    }
}

fn main() {
    // Read all input; spec says well-formed non-negative integers.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n_opt = it.next();
        if n_opt.is_none() {
            break;
        }
        let n: usize = n_opt.unwrap().parse().unwrap();
        if n == 0 {
            break;
        }

        let mut dsu = DSU::new(n);
        let mut f0 = vec![0i32; n];
        let mut f1 = vec![0i32; n];

        for i in 0..n {
            let a: usize = it.next().unwrap().parse().unwrap();
            let v0: i32 = it.next().unwrap().parse().unwrap();
            let b: usize = it.next().unwrap().parse().unwrap();
            let v1: i32 = it.next().unwrap().parse().unwrap();

            f0[i] = v0;
            f1[i] = v1;

            // Per spec, indices are within range; no explicit error handling.
            dsu.unite(i, a);
            dsu.unite(i, b);
        }

        let mut maxv = vec![0i32; n];
        let mut cnt = vec![0i32; n];

        for i in 0..n {
            let k = dsu.root(i);
            check(k, f0[i], &mut maxv, &mut cnt);
            check(k, f1[i], &mut maxv, &mut cnt);
        }

        let mut ans: i32 = 1;
        for i in 0..n {
            if cnt[i] != 0 {
                ans = (ans * (cnt[i] / 2)) % MOD;
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
