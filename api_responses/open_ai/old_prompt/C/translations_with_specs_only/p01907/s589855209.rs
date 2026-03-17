use std::collections::HashMap;
use std::fmt;

#[derive(Debug)]
struct Edge {
    to: usize,
    cost: usize,
}

struct Tree {
    edges: HashMap<usize, Vec<Edge>>,
    depth: Vec<usize>,
    costs: Vec<usize>,
}

impl Tree {
    fn new(n: usize) -> Self {
        Tree {
            edges: HashMap::new(),
            depth: vec![0; n],
            costs: vec![0; n],
        }
    }

    fn add_edge(&mut self, from: usize, to: usize, cost: usize) {
        self.edges.entry(from).or_insert_with(Vec::new).push(Edge { to, cost });
        self.costs[from] = cost; // Assuming cost is associated with the 'from' node
    }

    fn dfs(&mut self, node: usize, current_depth: usize) {
        self.depth[node] = current_depth;
        if let Some(neighbors) = self.edges.get(&node) {
            for edge in neighbors {
                self.dfs(edge.to, current_depth + 1);
            }
        }
    }

    fn calculate_weighted_sum(&self, p: f64) -> f64 {
        let mut weighted_sum = 0.0;
        for (i, &cost) in self.costs.iter().enumerate() {
            weighted_sum += cost as f64 * (p.powi(self.depth[i] as i32));
        }
        weighted_sum
    }
}

fn main() {
    let p: f64 = 0.5; // Example probability factor
    let n: usize = 3; // Example number of nodes

    let mut tree = Tree::new(n);
    tree.add_edge(0, 1, 10);
    tree.add_edge(0, 2, 20);

    // Perform DFS starting from the root node (0)
    tree.dfs(0, 0);

    // Calculate the weighted sum based on the depth and costs
    let result = tree.calculate_weighted_sum(p);

    // Print the result formatted to 10 decimal places
    println!("{:.10}", result);
}
