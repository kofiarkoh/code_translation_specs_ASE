struct Graph {
    num_vertices: usize,
    edges: Vec<(usize, usize, usize)>, // (start, end, cost)
}

impl Graph {
    fn new(num_vertices: usize, start_vertices: Vec<usize>, end_vertices: Vec<usize>, costs: Vec<usize>) -> Self {
        let edges = start_vertices.into_iter()
            .zip(end_vertices.into_iter())
            .zip(costs.into_iter())
            .map(|((start, end), cost)| (start, end, cost))
            .collect();
        Graph { num_vertices, edges }
    }

    fn max_path_cost(&self, start_vertex: usize) -> usize {
        // Initialize cost array and visited status
        let mut costs = vec![0; self.num_vertices];
        let mut visited = vec![false; self.num_vertices];

        // Perform DFS to find maximum path cost
        self.dfs(start_vertex, &mut costs, &mut visited);

        // Return the maximum cost found
        *costs.iter().max().unwrap_or(&0)
    }

    fn dfs(&self, vertex: usize, costs: &mut Vec<usize>, visited: &mut Vec<bool>) {
        // Mark the vertex as visited
        visited[vertex] = true;

        // Explore all edges from the current vertex
        for &(start, end, cost) in &self.edges {
            if start == vertex && !visited[end] {
                // Update the cost to reach the end vertex
                costs[end] = costs[end].max(costs[start] + cost);
                // Recursively visit the next vertex
                self.dfs(end, costs, visited);
            }
        }

        // Reset the visited status for the next computation
        visited[vertex] = false;
    }
}

fn main() {
    // Example usage
    let num_vertices = 5;
    let start_vertices = vec![0, 1, 2, 3];
    let end_vertices = vec![1, 2, 3, 4];
    let costs = vec![10, 20, 30, 40];

    let graph = Graph::new(num_vertices, start_vertices, end_vertices, costs);
    let max_cost = graph.max_path_cost(0);
    println!("Maximum path cost: {}", max_cost);
}
