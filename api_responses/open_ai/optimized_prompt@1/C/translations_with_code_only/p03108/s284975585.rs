use std::io::{self, BufRead};

const MAX: usize = 100_000;

struct UnionFind {
    par: [usize; MAX],
    root_cnt: [usize; MAX],
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut par = [0; MAX];
        let mut root_cnt = [0; MAX];
        for i in 0..n {
            par[i] = i;
            root_cnt[i] = 1;
        }
        UnionFind { par, root_cnt }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.par[x] == x {
            return x;
        }
        self.par[x] = self.root(self.par[x]); // Path compression
        self.par[x]
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let rx = self.root(x);
        let ry = self.root(y);
        if rx == ry {
            return false;
        }
        self.par[rx] = ry;
        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace();
    let n: usize = nums.next().unwrap().parse().unwrap();
    let m: usize = nums.next().unwrap().parse().unwrap();

    let mut a = [0; MAX];
    let mut b = [0; MAX];
    for i in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut nums = line.split_whitespace();
        a[i] = nums.next().unwrap().parse::<usize>().unwrap() - 1;
        b[i] = nums.next().unwrap().parse::<usize>().unwrap() - 1;
    }

    let mut uf = UnionFind::new(n);
    let mut ans = [0; MAX];

    ans[m - 1] = (n * (n - 1)) / 2;
    uf.root_cnt[uf.root(b[m - 1])] += uf.root_cnt[uf.root(a[m - 1])];
    uf.root_cnt[uf.root(a[m - 1])] = 0;
    uf.unite(a[m - 1], b[m - 1]); // b's root receives a

    if m >= 2 {
        ans[m - 2] = (n * (n - 1)) / 2 - 1;
    }

    for i in (0..m - 1).rev() {
        if uf.same(a[i], b[i]) {
            ans[i - 1] = ans[i];
        } else {
            ans[i - 1] = ans[i] - (uf.root_cnt[uf.root(b[i])] * uf.root_cnt[uf.root(a[i])]);
            uf.root_cnt[uf.root(b[i])] += uf.root_cnt[uf.root(a[i])];
            uf.root_cnt[uf.root(a[i])] = 0;
            uf.unite(a[i], b[i]); // b's root receives a
        }
    }

    for i in 0..m {
        println!("{}", ans[i]);
    }
}
