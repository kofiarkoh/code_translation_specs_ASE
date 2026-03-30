struct UnionFind {
    parent: Vec<usize>,
    size: Vec<isize>, // Negative size to represent the size of the set
    removals: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![-1; n], // Initialize sizes to -1
            removals: vec![0; n],
        }
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
            // Increment removals for both elements
            self.removals[root_x] += 1;
            self.removals[root_y] += 1;

            // Union by size
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn query(&mut self, x: usize, y: usize) -> isize {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x == root_y {
            // Increment removals for both elements
            self.removals[root_x] += 1;
            // Return adjusted size
            return self.size[root_x] - self.removals[root_x] as isize;
        }
        // If not connected, return 0
        0
    }

    fn adjusted_sizes(&self) -> Vec<isize> {
        let mut sizes = vec![0; self.parent.len()];
        for i in 0..self.parent.len() {
            let root = self.find(i);
            sizes[root] += 1; // Count total elements in each set
        }
        sizes.iter()
            .enumerate()
            .map(|(i, &count)| count - self.removals[i] as isize) // Adjust sizes by removals
            .collect()
    }
}

// Example usage (not part of the specification):
// fn main() {
//     let mut uf = UnionFind::new(5);
//     uf.union(0, 1);
//     uf.union(1, 2);
//     uf.query(0, 2);
//     let sizes = uf.adjusted_sizes();
//     println!("{:?}", sizes); // Outputs adjusted sizes
// }
