// Import necessary modules
use std::io::{self, BufRead};
use std::collections::HashMap;

const MODULO: usize = 10007;

// Structure to represent a Union-Find (Disjoint Set Union) for managing family groups
struct UnionFind {
    parent: Vec<usize>,
    max_value: Vec<usize>,
}

impl UnionFind {
    // Create a new Union-Find structure
    fn new(size: usize) -> Self {
        UnionFind {
            parent: (0..size).collect(),
            max_value: vec![0; size],
        }
    }

    // Find the root of the set containing `x` with path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]);
        }
        self.parent[x]
    }

    // Union the sets containing `x` and `y`
    fn union(&mut self, x: usize, y: usize) {
        let root_x = self.find(x);
        let root_y = self.find(y);
        if root_x != root_y {
            self.parent[root_y] = root_x; // Union by linking root_y to root_x
        }
    }

    // Update the maximum value for the group containing `x`
    fn update_max_value(&mut self, x: usize, value: usize) {
        let root = self.find(x);
        self.max_value[root] = self.max_value[root].max(value);
    }

    // Calculate the total maximum values for all groups
    fn calculate_result(&mut self) -> usize {
        let mut unique_roots = HashMap::new();
        for i in 0..self.parent.len() {
            let root = self.find(i);
            unique_roots.insert(root, self.max_value[root]);
        }
        unique_roots.values().sum::<usize>() % MODULO
    }
}

// Main function to read input and process relationships
fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    while let Some(Ok(line)) = lines.next() {
        let mut values: Vec<usize> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Read the number of relationships
        let n = values.remove(0);
        let mut uf = UnionFind::new(100_002); // Initialize Union-Find for up to 100,002 members

        // Process each pair of relationships
        for _ in 0..n {
            if let Some(member1) = values.pop() {
                if let Some(member2) = values.pop() {
                    // Read the associated value for the relationship
                    let value = values.pop().unwrap_or(0); // Default to 0 if not enough values
                    uf.union(member1, member2);
                    uf.update_max_value(member1, value);
                    uf.update_max_value(member2, value);
                }
            }
        }

        // Calculate and output the result
        let result = uf.calculate_result();
        println!("{}", result);
    }
}
