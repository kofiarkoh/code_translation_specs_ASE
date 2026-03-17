// Union-Find (Disjoint Set) implementation in Rust
use std::io::{self, BufRead};

struct UnionFind {
    par: Vec<usize>,      // Parent array
    root_cnt: Vec<usize>, // Size of each component
    count: usize,         // Number of connected pairs
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let par = (0..n).collect(); // Each node is its own parent
        let root_cnt = vec![1; n];  // Each component starts with size 1
        UnionFind { par, root_cnt, count: 0 }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.par[x] != x {
            self.par[x] = self.find(self.par[x]); // Path compression
        }
        self.par[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by size
            self.count -= self.root_cnt[root_x] * self.root_cnt[root_y]; // Update connected pairs
            if self.root_cnt[root_x] < self.root_cnt[root_y] {
                self.par[root_x] = root_y;
                self.root_cnt[root_y] += self.root_cnt[root_x];
            } else {
                self.par[root_y] = root_x;
                self.root_cnt[root_x] += self.root_cnt[root_y];
            }
            self.count += self.root_cnt[self.find(root_x)] * self.root_cnt[self.find(root_y)]; // Update connected pairs
        }
    }

    fn connected_pairs(&self) -> usize {
        self.count
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read n and m
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut uf = UnionFind::new(n);
    let mut results = Vec::new();

    // Read edges and process unions
    let mut edges = Vec::new();
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse().unwrap();
        let b: usize = parts.next().unwrap().parse().unwrap();
        edges.push((a, b));
    }

    // Process unions in reverse order
    for (a, b) in edges.iter().rev() {
        uf.union(*a, *b);
        results.push(uf.connected_pairs());
    }

    // Output results in reverse order
    for result in results.iter().rev() {
        println!("{}", result);
    }
}
