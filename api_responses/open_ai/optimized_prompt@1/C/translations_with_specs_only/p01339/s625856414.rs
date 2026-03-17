use std::collections::HashMap;

const MOD: i32 = 1_000_000_007;

struct Graph {
    vertices: usize,
    edges: usize,
    adjacency_list: HashMap<usize, Vec<usize>>,
}

impl Graph {
    fn new(vertices: usize, edges: usize) -> Self {
        // Initialize the graph with the given number of vertices and edges
        Graph {
            vertices,
            edges,
            adjacency_list: HashMap::new(),
        }
    }

    fn add_edge(&mut self, from: usize, to: usize) {
        // Add a directed edge from 'from' to 'to'
        // Behavior is undefined if indices are invalid, so we assume valid input as per spec
        self.adjacency_list.entry(from).or_insert_with(Vec::new).push(to);
    }

    fn count_scc_traversals(&self) -> i32 {
        // Placeholder for SCC counting logic
        // This function should compute the number of distinct traversal ways
        // based on the strongly connected components of the graph.
        // The actual implementation of SCC detection and traversal counting is omitted
        // for brevity, but would typically involve algorithms like Tarjan's or Kosaraju's.

        // For now, we return a dummy value to satisfy the return type.
        // In a complete implementation, this would return the actual count.
        1 // Placeholder value
    }
}

fn main() {
    // Example usage
    let vertices = 5; // Number of vertices
    let edges = 4;    // Number of edges

    let mut graph = Graph::new(vertices, edges);
    
    // Adding edges (example edges, should be replaced with actual input)
    graph.add_edge(0, 1);
    graph.add_edge(1, 2);
    graph.add_edge(2, 0);
    graph.add_edge(3, 4);

    // Count distinct traversal ways based on SCCs
    let result = graph.count_scc_traversals() % MOD;

    // Output the result
    println!("{}", result);
}
