struct Graph {
    adjacency_list: Vec<Vec<usize>>,
}

impl Graph {
    // Create a new graph with a specified number of vertices
    fn new(num_vertices: usize) -> Self {
        // Initialize the adjacency list with empty vectors for each vertex
        Graph {
            adjacency_list: vec![Vec::new(); num_vertices],
        }
    }

    // Add an edge from source to target
    fn add_edge(&mut self, source: usize, target: usize) {
        // Assuming the caller ensures that source and target are valid indices
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
                    return true; // Cycle detected
                }
            } else if rec_stack[neighbor] {
                return true; // Cycle detected
            }
        }

        rec_stack[vertex] = false; // Backtrack
        false
    }
}

// Example usage
fn main() {
    let num_vertices = 5; // Example number of vertices
    let num_edges = 4; // Example number of edges

    let mut graph = Graph::new(num_vertices);
    
    // Example edges (source, target)
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0); // This creates a cycle
    graph.add_edge(3, 4);

    let cycle_exists = graph.has_cycle();
    println!("{}", cycle_exists); // Outputs 1 if a cycle exists, otherwise 0
}
