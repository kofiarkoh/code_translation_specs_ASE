use std::io::{self, BufRead};

struct UnionFind {
    par: Vec<usize>,
    root_cnt: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let par = (0..n).collect();
        let root_cnt = vec![1; n];
        UnionFind { par, root_cnt, count: n }
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
            if self.root_cnt[root_x] < self.root_cnt[root_y] {
                self.par[root_x] = root_y;
                self.root_cnt[root_y] += self.root_cnt[root_x];
            } else {
                self.par[root_y] = root_x;
                self.root_cnt[root_x] += self.root_cnt[root_y];
            }
            self.count -= 1; // Decrease the number of connected components
        }
    }

    fn connected_pairs(&self) -> usize {
        // Calculate the number of connected pairs based on the sizes of components
        let mut total_pairs = 0;
        for &size in &self.root_cnt {
            total_pairs += size * (size - 1) / 2; // C(size, 2)
        }
        total_pairs
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let parts: Vec<usize> = first_line.split_whitespace()
        .map(|s| s.parse().unwrap()).collect();
    let n = parts[0];
    let m = parts[1];

    let mut uf = UnionFind::new(n);
    let mut edges = Vec::new();

    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let edge: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap()).collect();
        edges.push((edge[0], edge[1]));
    }

    // Process edges in reverse order
    let mut results = Vec::new();
    for (a, b) in edges.iter().rev() {
        uf.union(*a, *b);
        results.push(uf.connected_pairs());
    }

    // Output results in reverse order
    for result in results.iter().rev() {
        println!("{}", result);
    }
}
