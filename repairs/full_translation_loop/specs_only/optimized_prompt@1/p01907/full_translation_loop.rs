use std::collections::HashMap;

#[derive(Debug)]
struct TreeNode {
    cost: f64,
    children: Vec<(usize, f64)>, // Store child index and cost together
}

impl TreeNode {
    fn new() -> Self {
        TreeNode {
            cost: 0.0,
            children: Vec::new(),
        }
    }
}

struct Tree {
    nodes: HashMap<usize, TreeNode>,
}

impl Tree {
    fn new() -> Self {
        Tree {
            nodes: HashMap::new(),
        }
    }

    fn add_edge(&mut self, parent: usize, child: usize, cost: f64) {
        self.nodes.entry(parent).or_insert(TreeNode::new());
        self.nodes.entry(child).or_insert(TreeNode::new());
        self.nodes.get_mut(&parent).unwrap().children.push((child, cost));
    }

    fn compute_weighted_sum(&mut self, probability: f64) -> f64 {
        let mut total_cost = 0.0;
        self.dfs(1, 0, probability, &mut total_cost); // Start DFS from node 1 (root)
        total_cost
    }

    fn dfs(&mut self, node_index: usize, depth: usize, probability: f64, total_cost: &mut f64) {
        if let Some(node) = self.nodes.get_mut(&node_index) {
            *total_cost += node.cost * (depth as f64) * probability; // Weighted cost calculation
            for &(child, cost) in &node.children {
                self.nodes.get_mut(&child).unwrap().cost = cost; // Set cost for child node
                self.dfs(child, depth + 1, probability, total_cost);
            }
        }
    }
}

fn main() {
    let probability: f64 = 0.5; // Example probability factor

    let mut tree = Tree::new();
    // Example edges (parent, child, cost)
    tree.add_edge(1, 2, 3.0);
    tree.add_edge(1, 3, 4.0);
    tree.add_edge(2, 4, 2.0);
    tree.add_edge(2, 5, 1.0);

    let result = tree.compute_weighted_sum(probability);
    println!("{:.10}", result); // Output formatted to ten decimal places
}
