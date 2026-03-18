use std::collections::HashMap;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    removals: HashMap<usize, usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let parent = (0..n).collect();
        let size = vec![1; n];
        let removals = HashMap::new();
        UnionFind { parent, size, removals }
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
            if self.size[root_x] < self.size[root_y] {
                self.parent[root_x] = root_y;
                self.size[root_y] += self.size[root_x];
            } else {
                self.parent[root_y] = root_x;
                self.size[root_x] += self.size[root_y];
            }
        }
    }

    fn add_removal(&mut self, element: usize) {
        let root = self.find(element); // Find the root first to avoid multiple mutable borrows
        *self.removals.entry(root).or_insert(0) += 1;
    }

    fn get_size(&mut self, element: usize) -> isize {
        let root = self.find(element);
        let size = self.size[root];
        let removal_count = self.removals.get(&root).copied().unwrap_or(0);
        (size as isize).saturating_sub(removal_count as isize) // Ensure no negative sizes
    }
}

fn main() {
    let input = "5 3 2\n1 2\n2 3\n4 5\n1 3\n4 5"; // Example input
    let lines: Vec<&str> = input.lines().collect();
    
    let first_line: Vec<usize> = lines[0].split_whitespace().map(|s| s.parse().unwrap()).collect();
    let n = first_line[0]; // Number of elements
    let union_count = first_line[1]; // Number of union operations
    let query_count = first_line[2]; // Number of queries

    let mut uf = UnionFind::new(n);

    // Process union operations
    for i in 1..=union_count {
        let pair: Vec<usize> = lines[i].split_whitespace().map(|s| s.parse().unwrap()).collect();
        uf.union(pair[0] - 1, pair[1] - 1); // Convert to 0-based index
    }

    // Process removals
    let removals: Vec<usize> = lines[(union_count + 1)..(union_count + 1 + query_count)]
        .iter()
        .flat_map(|line| line.split_whitespace().map(|s| s.parse::<usize>().unwrap()))
        .collect();

    for element in removals {
        uf.add_removal(element - 1); // Convert to 0-based index
    }

    // Output sizes after removals
    for i in 0..n {
        let size = uf.get_size(i);
        println!("{}", size); // Output the sizes after removals
    }
}
