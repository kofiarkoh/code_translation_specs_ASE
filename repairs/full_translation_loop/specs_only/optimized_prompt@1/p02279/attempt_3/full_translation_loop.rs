use std::fmt;

#[derive(Debug)]
struct Node {
    id: usize,
    parent: isize,
    depth: usize,
    node_type: NodeType,
    children: Vec<usize>,
}

#[derive(Debug)]
enum NodeType {
    Root,
    Leaf,
    Internal,
}

impl fmt::Display for Node {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let children_str = self.children
            .iter()
            .map(|child| child.to_string())
            .collect::<Vec<String>>()
            .join(", ");
        let type_str = match self.node_type {
            NodeType::Root => "root",
            NodeType::Leaf => "leaf",
            NodeType::Internal => "internal node",
        };
        write!(
            f,
            "node {}: parent = {}, depth = {}, {}, [{}]",
            self.id, self.parent, self.depth, type_str, children_str
        )
    }
}

struct BinaryTree {
    nodes: Vec<Node>,
}

impl BinaryTree {
    fn new(num_nodes: usize) -> Self {
        let mut nodes = Vec::with_capacity(num_nodes);
        for id in 0..num_nodes {
            nodes.push(Node {
                id,
                parent: -1,
                depth: 0,
                node_type: NodeType::Leaf, // Default type
                children: Vec::new(),
            });
        }
        BinaryTree { nodes }
    }

    fn add_node(&mut self, id: usize, parent: isize, children: Vec<usize>) {
        if parent >= 0 {
            self.nodes[id].parent = parent;
            self.nodes[parent as usize].children.push(id);
            self.nodes[parent as usize].node_type = NodeType::Internal; // Parent becomes internal
        }
        self.nodes[id].children = children; // Direct assignment is fine
        self.nodes[id].depth = self.calculate_depth(id);
        if self.nodes[id].children.is_empty() {
            self.nodes[id].node_type = NodeType::Leaf; // No children means it's a leaf
        }
    }

    fn calculate_depth(&self, id: usize) -> usize {
        let mut depth = 0;
        let mut current = self.nodes[id].parent;
        while current >= 0 {
            depth += 1;
            current = self.nodes[current as usize].parent;
        }
        depth
    }

    fn display(&self) {
        for node in &self.nodes {
            println!("{}", node);
        }
    }
}

fn main() {
    let num_nodes = 5; // Example input
    let mut tree = BinaryTree::new(num_nodes);

    // Example input data: (id, parent, children)
    let input_data = vec![
        (0, -1, vec![1, 2, 3]), // Root node with three children
        (1, 0, vec![]),          // Leaf node
        (2, 0, vec![]),          // Leaf node
        (3, 0, vec![4]),         // Internal node with one child
        (4, 3, vec![]),          // Leaf node
    ];

    for (id, parent, children) in input_data {
        tree.add_node(id, parent, children);
    }

    tree.display();
}
