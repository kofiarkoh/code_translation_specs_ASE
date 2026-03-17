use std::collections::VecDeque;

#[derive(Debug)]
struct Edge {
    target: usize,
    cost: i32,
}

#[derive(Debug)]
struct AdjacencyList {
    edges: Vec<Vec<Edge>>,
    num_edges: Vec<usize>,
}

impl AdjacencyList {
    // Creates a new adjacency list for a directed graph with the given number of vertices.
    fn new(num_vertex: usize) -> Self {
        // Preconditions: num_vertex must be a positive integer.
        assert!(num_vertex > 0, "Number of vertices must be positive.");
        
        AdjacencyList {
            edges: vec![Vec::new(); num_vertex],
            num_edges: vec![0; num_vertex],
        }
    }

    // Adds an edge from the source vertex to the target vertex with the given cost.
    fn push_edge(&mut self, source: usize, edge: Edge) {
        // Preconditions: source must be a valid index within the range of vertices.
        assert!(source < self.edges.len(), "Source vertex index out of bounds.");
        
        self.edges[source].push(edge);
        self.num_edges[source] += 1; // Update the number of edges for the source vertex.
    }

    // Checks if the graph contains a cycle.
    fn is_cyclic(&self) -> i32 {
        let mut visited = vec![false; self.edges.len()];
        let mut rec_stack = vec![false; self.edges.len()];

        for node in 0..self.edges.len() {
            if !visited[node] {
                if self.dfs(node, &mut visited, &mut rec_stack) {
                    return 1; // Cycle detected
                }
            }
        }
        0 // No cycle detected
    }

    // Depth-first search helper function to detect cycles.
    fn dfs(&self, node: usize, visited: &mut Vec<bool>, rec_stack: &mut Vec<bool>) -> bool {
        visited[node] = true;
        rec_stack[node] = true;

        for edge in &self.edges[node] {
            if !visited[edge.target] {
                if self.dfs(edge.target, visited, rec_stack) {
                    return true; // Cycle detected
                }
            } else if rec_stack[edge.target] {
                return true; // Cycle detected
            }
        }

        rec_stack[node] = false; // Remove the node from recursion stack
        false
    }
}

fn main() {
    // Example usage:
    let mut graph = AdjacencyList::new(5); // Create a graph with 5 vertices
    graph.push_edge(0, Edge { target: 1, cost: 10 });
    graph.push_edge(1, Edge { target: 2, cost: 20 });
    graph.push_edge(2, Edge { target: 0, cost: 30 }); // This creates a cycle

    let cycle_detected = graph.is_cyclic();
    println!("{}", cycle_detected); // Output: 1 (cycle detected)
}
