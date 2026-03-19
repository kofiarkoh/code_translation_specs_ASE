use std::io::{self, BufRead};
use std::str::FromStr;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind { parent, size }
    }

    fn root(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.root(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn unite(&mut self, x: usize, y: usize) -> bool {
        let root_x = self.root(x);
        let root_y = self.root(y);
        if root_x == root_y {
            return false; // Already in the same set
        }
        self.parent[root_x] = root_y; // Union
        self.size[root_y] += self.size[root_x]; // Update size
        true
    }

    fn same(&mut self, x: usize, y: usize) -> bool {
        self.root(x) == self.root(y)
    }

    fn component_size(&mut self, x: usize) -> usize {
        self.size[self.root(x)]
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let m: usize = parts.next().unwrap().parse().unwrap();

    let mut edges = Vec::with_capacity(m);
    for _ in 0..m {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let a: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based index
        let b: usize = parts.next().unwrap().parse::<usize>().unwrap() - 1; // Convert to 0-based index
        edges.push((a, b));
    }

    let mut uf = UnionFind::new(n);
    let mut ans = vec![0; m];
    ans[m - 1] = (n * (n - 1)) / 2; // Initial number of connected pairs

    let (a_last, b_last) = edges[m - 1];
    uf.unite(a_last, b_last); // Unite the last edge

    for i in (0..m - 1).rev() {
        let (a_i, b_i) = edges[i];
        if uf.same(a_i, b_i) {
            ans[i] = ans[i + 1]; // No change in connected pairs
        } else {
            let size_a = uf.component_size(a_i);
            let size_b = uf.component_size(b_i);
            ans[i] = ans[i + 1] - size_a * size_b; // Update connected pairs
            uf.unite(a_i, b_i); // Unite the components
        }
    }

    for result in ans {
        println!("{}", result);
    }
}
