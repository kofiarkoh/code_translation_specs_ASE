// Import necessary modules
use std::io::{self, BufRead};

// Union-Find structure to manage family relationships
struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    max: Vec<i32>,
    cnt: Vec<i32>,
}

impl UnionFind {
    // Create a new Union-Find structure
    fn new(n: usize) -> Self {
        UnionFind {
            parent: (0..n).collect(),
            size: vec![1; n],
            max: vec![0; n], // Initialize max values to 0
            cnt: vec![0; n], // Initialize counts to 0
        }
    }

    // Find the root of the set containing `x`
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    // Union the sets containing `x` and `y`
    fn union(&mut self, x: usize, y: usize, f: &[(i32, i32)]) {
        let root_x = self.find(x);
        let root_y = self.find(y);

        if root_x != root_y {
            // Union by size
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
                self.cnt[root_y] += self.cnt[root_x];
                self.max[root_y] = self.max[root_y].max(f[root_x].1); // Update max
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
                self.cnt[root_x] += self.cnt[root_y];
                self.max[root_x] = self.max[root_x].max(f[root_y].1); // Update max
            }
        }
    }

    // Update counts and max values for a member
    fn update(&mut self, index: usize, value: i32) {
        let root = self.find(index);
        self.cnt[root] += 1; // Increment count for the group
        self.max[root] = self.max[root].max(value); // Update max value
    }
}

// Main function to read input and compute results
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read number of family members
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut union_find = UnionFind::new(n);

    // Read family relationships and associated values
    let mut f = vec![(0, 0); n];
    for i in 0..n {
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        f[i] = (values[0], values[1]); // Store associated values
        union_find.update(i, values[1]); // Update counts and max values
    }

    // Process relationships
    while let Some(line) = lines.next() {
        let line = line.unwrap();
        let values: Vec<usize> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        let a = values[0];
        let b = values[1];
        union_find.union(a, b, &f); // Union the two family members
    }

    // Calculate the result
    let mut result = 1;
    for i in 0..n {
        if union_find.find(i) == i { // Only consider root representatives
            result *= union_find.cnt[i] * union_find.max[i]; // Product of counts and max values
        }
    }

    // Print the result
    println!("{}", result);
}
