// Define a struct to represent the Union-Find (Disjoint Set Union) data structure
struct UnionFind {
    parent: Vec<usize>, // Parent array to track the root of each element
    size: Vec<usize>,   // Size array to track the size of each set
    removals: Vec<usize>, // Array to track removals for each element
}

impl UnionFind {
    // Initialize the Union-Find structure with a given number of elements
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        let mut removals = Vec::with_capacity(n);
        
        for i in 0..n {
            parent.push(i); // Each element is its own parent initially
            size.push(1);   // Each set starts with size 1
            removals.push(0); // No removals initially
        }
        
        UnionFind { parent, size, removals }
    }

    // Find the root of the element with path compression
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    // Union two sets containing elements x and y
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

    // Record a removal for a specific element
    fn remove(&mut self, x: usize) {
        self.removals[x] += 1; // Increment the removal count for the element
    }

    // Get the size of the set containing element x, adjusted for removals
    fn get_size(&mut self, x: usize) -> isize {
        let root = self.find(x);
        let size_after_removals = self.size[root] as isize - self.removals[x] as isize; // Return size minus removals
        if size_after_removals <= 0 {
            -1 // Return -1 if size is non-positive
        } else {
            size_after_removals
        }
    }
}

// Main function to handle input and output
fn main() {
    // Example input values (these would typically come from user input or a file)
    let num_elements = 5; // Number of elements
    let num_unions = 3;   // Number of union operations
    let num_queries = 2;   // Number of queries

    // Create a new Union-Find structure
    let mut uf = UnionFind::new(num_elements);

    // Example union operations (these would typically be read from input)
    let unions = vec![(0, 1), (1, 2), (3, 4)];
    for (x, y) in unions {
        uf.union(x, y);
    }

    // Example removal operations (these would typically be read from input)
    let removals = vec![0, 1, 0, 0, 1]; // Number of removals for each element
    for (i, &count) in removals.iter().enumerate() {
        for _ in 0..count {
            uf.remove(i);
        }
    }

    // Example query operations (these would typically be read from input)
    let queries = vec![0, 1, 2, 3, 4];
    let mut results = Vec::new();
    for &x in &queries {
        results.push(uf.get_size(x)); // Get adjusted size for each queried element
    }

    // Output the results
    for size in results {
        println!("{}", size); // Print the size of each set after removals
    }
}
