// Define a struct to represent the disjoint set
struct DisjointSet {
    parent: Vec<usize>, // Parent array for union-find
    size: Vec<usize>,   // Size array to track the size of each set
    removed: Vec<bool>, // Track removed elements
}

impl DisjointSet {
    // Initialize the disjoint set with a given number of elements
    fn new(n: usize) -> Self {
        let mut parent = Vec::with_capacity(n);
        let mut size = Vec::with_capacity(n);
        let mut removed = Vec::with_capacity(n);
        
        for i in 0..n {
            parent.push(i);
            size.push(1); // Each element is its own set initially
            removed.push(false); // No elements are removed initially
        }
        
        DisjointSet { parent, size, removed }
    }

    // Find the root of the set containing element x
    fn find(&mut self, x: usize) -> usize {
        if self.parent[x] != x {
            self.parent[x] = self.find(self.parent[x]); // Path compression
        }
        self.parent[x]
    }

    // Union the sets containing elements x and y
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

    // Mark an element as removed
    fn remove(&mut self, x: usize) {
        self.removed[x] = true;
    }

    // Get the size of the set containing element x, adjusted for removals
    fn adjusted_size(&mut self, x: usize) -> usize {
        let root = self.find(x);
        let mut count = 0;
        
        // Count the number of elements in the set that are not removed
        for i in 0..self.parent.len() {
            if self.find(i) == root && !self.removed[i] {
                count += 1;
            }
        }
        
        count
    }
}

// Main function to handle input and output
fn main() {
    // Example input values (these would typically come from user input)
    let num_elements = 5; // Number of elements
    let num_unions = 2;   // Number of union operations
    let num_queries = 3;  // Number of query operations

    // Create a new disjoint set
    let mut ds = DisjointSet::new(num_elements);

    // Example union operations (these would typically come from user input)
    ds.union(0, 1);
    ds.union(1, 2);

    // Example remove operations (these would typically come from user input)
    ds.remove(1);

    // Example query operations (these would typically come from user input)
    let mut results = Vec::new();
    for i in 0..num_elements {
        results.push(ds.adjusted_size(i));
    }

    // Print the adjusted sizes of each set
    println!("{:?}", results);
}
