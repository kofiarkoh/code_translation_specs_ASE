// Union-Find (Disjoint Set) implementation in Rust
// This implementation tracks connected components and the number of removals for each element.

const N_MAX: usize = 100_000;

struct UnionFind {
    parent: Vec<usize>,
    size: Vec<usize>,
    removals: Vec<usize>,
}

impl UnionFind {
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n + 1);
        let mut size = Vec::with_capacity(n + 1);
        let mut removals = Vec::with_capacity(n + 1);
        
        for i in 0..=n {
            parent.push(i);
            size.push(1);
            removals.push(0);
        }
        
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

    fn remove(&mut self, x: usize) {
        self.removals[x] += 1; // Increment removal count for the element
    }

    fn component_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        self.size[root] - self.removals[x] // Adjust size for removals
    }

    fn print_sizes(&mut self, n: usize) {
        let mut sizes = Vec::new();
        for i in 1..=n {
            sizes.push(self.component_size(i));
        }
        println!("{}", sizes.iter().map(|s| s.to_string()).collect::<Vec<String>>().join(" "));
    }
}

fn main() {
    let n: usize = 5; // Example input for number of elements
    let m: usize = 3; // Example input for number of union operations
    let k: usize = 2; // Example input for number of queries

    let mut uf = UnionFind::new(n);

    // Example union operations
    uf.union(1, 2);
    uf.union(2, 3);
    uf.union(4, 5);

    // Example removal operations
    uf.remove(2);
    uf.remove(3);

    // Print sizes after accounting for removals
    uf.print_sizes(n);
}
