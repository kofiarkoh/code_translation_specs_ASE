#[derive(Debug, Clone)]
struct TreeNode {
    edges: Vec<(usize, f64)>, // (connected_node_index, cost)
}

struct Tree {
    nodes: Vec<TreeNode>,
}

impl Tree {
    fn new(num_nodes: usize) -> Self {
        Tree {
            nodes: vec![TreeNode { edges: Vec::new() }; num_nodes],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: f64) {
        self.nodes[from].edges.push((to, cost));
        self.nodes[to].edges.push((from, cost)); // Assuming undirected edges
    }

    fn depth_first_search(&self, node_index: usize, depth: usize, visited: &mut Vec<bool>, depths: &mut Vec<usize>) {
        visited[node_index] = true;
        depths[node_index] = depth;

        for &(neighbor_index, _) in &self.nodes[node_index].edges {
            if !visited[neighbor_index] {
                self.depth_first_search(neighbor_index, depth + 1, visited, depths);
            }
        }
    }

    fn compute_weighted_sum(&self, probability_factor: f64) -> f64 {
        let num_nodes = self.nodes.len();
        let mut visited = vec![false; num_nodes];
        let mut depths = vec![0; num_nodes];

        // Start DFS from the first node (index 0)
        self.depth_first_search(0, 0, &mut visited, &mut depths);

        // Calculate the weighted sum
        let mut weighted_sum = 0.0;
        for (i, &depth) in depths.iter().enumerate() {
            let cost: f64 = self.nodes[i].edges.iter().map(|&(_, edge_cost)| edge_cost).sum();
            weighted_sum += cost * probability_factor.powi(depth as i32);
        }

        weighted_sum
    }
}

fn main() {
    // Read input values
    let probability_factor: f64 = 0.5; // Example value, should be read from input
    let num_nodes: usize = 5; // Example value, should be read from input

    let mut tree = Tree::new(num_nodes);
    // Example edges, should be read from input
    tree.add_edge(0, 1, 3.0);
    tree.add_edge(0, 2, 4.0);
    tree.add_edge(1, 3, 2.0);
    tree.add_edge(1, 4, 1.0);

    let result = tree.compute_weighted_sum(probability_factor);
    println!("{:.10}", result); // Output formatted to ten decimal places
}
