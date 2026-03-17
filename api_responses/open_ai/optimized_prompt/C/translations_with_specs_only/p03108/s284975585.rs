struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    count: usize,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        UnionFind { parent, size, count: n }
    }

    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        
        if root_x != root_y {
            // Union by size
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
            self.count -= 1; // Decrease the number of connected components
        }
    }

    fn connected_pairs(&self) -> usize {
        // The number of connected pairs is given by the formula n * (n - 1) / 2
        // minus the number of pairs in each component
        let mut total_pairs = 0;
        for &s in &self.size {
            if s > 1 {
                total_pairs += s * (s - 1) / 2; // Pairs in this component
            }
        }
        (self.count * (self.count - 1)) / 2 - total_pairs
    }
}

fn main() {
    let n: usize = 5; // Example number of nodes
    let edges: Vec<(usize, usize)> = vec![(0, 1), (1, 2), (3, 4)]; // Example edges

    let mut uf = UnionFind::new(n);
    let mut results = Vec::new();

    for (u, v) in edges {
        uf.union(u, v);
        results.push(uf.connected_pairs());
    }

    // Print results in reverse order
    for result in results.iter().rev() {
        println!("{}", result);
    }
}
