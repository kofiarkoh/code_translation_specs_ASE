use std::io::{self, BufRead};

const MAX: usize = 100002;

struct UnionFind {
    id: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let id = (0..n).collect();
        let size = vec![1; n];
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

fn read_input() -> (usize, Vec<((usize, usize), (usize, usize))>) {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let n: usize = lines.next().unwrap().trim().parse().unwrap();
    let mut relationships = Vec::new();

    for _ in 0..n {
        let line = lines.next().unwrap();
        let nums: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        relationships.push(((nums[0], nums[1]), (nums[2], nums[3])));
    }

    (n, relationships)
}

fn check(k: usize, f: usize, cnt: &mut Vec<usize>, max: &mut Vec<usize>) {
    if cnt[k] == 0 || f > max[k] {
        cnt[k] = 1;
        max[k] = f;
    } else if f == max[k] {
        cnt[k] += 1;
    }
}

fn main() {
    let (n, relationships) = read_input();
    let mut uf = UnionFind::new(n);
    let mut f = vec![(0, 0); n];
    let mut cnt = vec![0; n];
    let mut max = vec![0; n];

    for (i, &((a, f0), (b, f1))) in relationships.iter().enumerate() {
        f[i] = (f0, f1);
        uf.unite(i, a);
        uf.unite(i, b);
    }

    for i in 0..n {
        let k = uf.root(i);
        check(k, f[i].0, &mut cnt, &mut max);
        check(k, f[i].1, &mut cnt, &mut max);
    }

    let mut ans = 1;
    const MOD: usize = 10007;
    for i in 0..n {
        if cnt[i] > 0 {
            ans = ans * (cnt[i] / 2) % MOD;
        }
    }
    println!("{}", ans);
}
