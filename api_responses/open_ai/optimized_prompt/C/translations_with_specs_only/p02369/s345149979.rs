// Directed graph implementation using an adjacency list representation
struct Graph {
    adjacency_list: Vec<Vec<usize>>, // Adjacency list to store edges
}

impl Graph {
    // Create a new graph with a specified number of vertices
    fn new(num_vertices: usize) -> Self {
        // Preconditions: num_vertices must be non-negative
        assert!(num_vertices >= 0, "Number of vertices must be non-negative");
        
        // Initialize the adjacency list with empty vectors for each vertex
        Graph {
            adjacency_list: vec![Vec::new(); num_vertices],
        }
    }

    // Add an edge from source to target
    fn add_edge(&mut self, source: usize, target: usize) {
        // Preconditions: source and target must be valid indices
        assert!(source < self.adjacency_list.len(), "Source index out of bounds");
        assert!(target < self.adjacency_list.len(), "Target index out of bounds");
        
        // Add the edge to the adjacency list
        self.adjacency_list[source].push(target);
    }

    // Check for cycles in the graph using Depth-First Search (DFS)
    fn has_cycle(&self) -> i32 {
        let mut visited = vec![false; self.adjacency_list.len()];
        let mut rec_stack = vec![false; self.adjacency_list.len()];

        for vertex in 0..self.adjacency_list.len() {
            if !visited[vertex] {
                if self.dfs(vertex, &mut visited, &mut rec_stack) {
                    return 1; // Cycle detected
                }
            }
        }
        0 // No cycle detected
    }

    // Helper function for DFS
    fn dfs(&self, vertex: usize, visited: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
        visited[vertex] = true;
        rec_stack[vertex] = true;

        for &neighbor in &self.adjacency_list[vertex] {
            if !visited[neighbor] {
                if self.dfs(neighbor, visited, rec_stack) {
                    return true; // Cycle detected in recursion
                }
            } else if rec_stack[neighbor] {
                return true; // Cycle detected via back edge
            }
        }

        rec_stack[vertex] = false; // Remove vertex from recursion stack
        false // No cycle found from this vertex
    }
}

// Example usage
fn main() {
    let num_vertices = 5; // Example number of vertices
    let mut graph = Graph::new(num_vertices);
    
    // Example edges
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0); // This creates a cycle
    graph.add_edge(3, 4);

    // Check for cycles
    let cycle_exists = graph.has_cycle();
    println!("Cycle exists: {}", cycle_exists); // Output: Cycle exists: 1
}
