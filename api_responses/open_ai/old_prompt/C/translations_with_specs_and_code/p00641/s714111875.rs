// Huge Family problem solution in Rust

use std::io::{self, BufRead};

const MAX: usize = 100002;

struct UnionFind {
    id: [usize; MAX],
    size: [usize; MAX],
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut id = [0; MAX];
        let mut size = [1; MAX];
        for i in 0..n {
            id[i] = i;
        }
        UnionFind { id, size }
    }

    fn root(&mut self, i: usize) -> usize {
        if i != self.id[i] {
            self.id[i] = self.root(self.id[i]); // Path compression
        }
        self.id[i]
    }

    fn connected(&mut self, p: usize, q: usize) -> bool {
        self.root(p) == self.root(q)
    }

    fn unite(&mut self, p: usize, q: usize) {
        let i = self.root(p);
        let j = self.root(q);
        if i == j {
            return;
        }
        if self.size[i] < self.size[j] {
            self.id[i] = j;
            self.size[j] += self.size[i];
        } else {
            self.id[j] = i;
            self.size[i] += self.size[j];
        }
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let n: usize = line.trim().parse().expect("Invalid input");
        if n == 0 {
            break; // Exit if n is zero
        }

        let mut uf = UnionFind::new(n);
        let mut f = [[0; 2]; MAX];
        let mut max = [0; MAX];
        let mut cnt = [0; MAX];

        for i in 0..n {
            let line = lines.next().unwrap().unwrap();
            let nums: Vec<usize> = line.split_whitespace()
                .map(|s| s.parse().expect("Invalid input"))
                .collect();
            let a = nums[0];
            f[i][0] = nums[1];
            let b = nums[2];
            f[i][1] = nums[3];
            uf.unite(i, a);
            uf.unite(i, b);
        }

        for i in 0..n {
            let k = uf.root(i);
            check(k, f[i][0], &mut cnt, &mut max);
            check(k, f[i][1], &mut cnt, &mut max);
        }

        let mut ans = 1;
        for i in 0..n {
            if cnt[i] > 0 {
                ans = ans * (cnt[i] >> 1) % 10007;
            }
        }
        println!("{}", ans);
    }
}

fn check(k: usize, f: usize, cnt: &mut [usize; MAX], max: &mut [usize; MAX]) {
    if cnt[k] == 0 || f > max[k] {
        cnt[k] = 1;
        max[k] = f;
    } else if f == max[k] {
        cnt[k] += 1;
    }
}
