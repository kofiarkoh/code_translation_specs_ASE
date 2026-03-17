use std::collections::HashMap;

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

struct BinaryTree {
    nodes: HashMap<usize, Node>,
    root_id: usize,
}

impl BinaryTree {
    fn new() -> Self {
        BinaryTree {
            nodes: HashMap::new(),
            root_id: 0, // Assuming the root node has id 0
        }
    }

    fn add_node(&mut self, id: usize, parent: isize, children: Vec<usize>) {
        let depth = if parent == -1 { 0 } else { self.nodes[&parent as &usize].depth + 1 };
        let node_type = if children.is_empty() {
            NodeType::Leaf
        } else {
            NodeType::Internal
        };

        let node = Node {
            id,
            parent,
            depth,
            node_type,
            children,
        };

        self.nodes.insert(id, node);
    }

    fn output_properties(&self) {
        for node in self.nodes.values() {
            let children_str = if node.children.is_empty() {
                String::from("None")
            } else {
                node.children.iter().map(|&id| id.to_string()).collect::<Vec<_>>().join(", ")
            };

            println!(
                "Node ID: {}, Parent: {}, Depth: {}, Type: {:?}, Children: [{}]",
                node.id,
                node.parent,
                node.depth,
                node.node_type,
                children_str
            );
        }
    }
}

fn main() {
    let mut tree = BinaryTree::new();
    
    // Example input: 3 nodes
    let input_data = vec![
        (0, -1, vec![1, 2]), // Node 0 (root) with children 1 and 2
        (1, 0, vec![]),     // Node 1 (leaf) with no children
        (2, 0, vec![]),     // Node 2 (leaf) with no children
    ];

    for (id, parent, children) in input_data {
        tree.add_node(id, parent, children);
    }

    tree.output_properties();
}
