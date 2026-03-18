use std::io::{self, Read};

#[derive(Debug, Clone)] // Derive Clone to allow cloning of TreeNode
struct TreeNode {
    value: char,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(value: char) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    // Function to combine two trees
    fn combine(&self, other: &Self) -> Self {
        let mut combined = TreeNode::new(self.value); // Start with the value of the first tree
        combined.children.extend(self.children.clone()); // Add children from the first tree
        combined.children.extend(other.children.clone()); // Add children from the second tree
        combined
    }

    // Function to parse a tree from a string representation
    fn from_string(s: &str) -> Option<Self> {
        // Basic parsing logic to create a tree from a string
        // This is a placeholder for actual parsing logic
        if s.is_empty() {
            return None; // Return None for empty strings
        }
        let root_value = s.chars().next().unwrap(); // Assume first char is the root
        let mut node = TreeNode::new(root_value);
        // Here we would parse children based on some rules (not implemented)
        Some(node)
    }

    // Function to convert the tree back to a string representation
    fn to_string(&self) -> String {
        let mut result = self.value.to_string();
        for child in &self.children {
            result.push_str(&child.to_string()); // Append children's string representations
        }
        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let trees: Vec<&str> = input.lines().collect();
    if trees.len() != 2 {
        eprintln!("Expected two tree representations as input.");
        return;
    }

    let tree1 = TreeNode::from_string(trees[0]);
    let tree2 = TreeNode::from_string(trees[1]);

    let combined_tree = match (tree1, tree2) {
        (Some(t1), Some(t2)) => t1.combine(&t2),
        (Some(t1), None) => t1, // If the second tree is None, return the first
        (None, Some(t2)) => t2, // If the first tree is None, return the second
        (None, None) => return, // If both are None, do nothing
    };

    println!("{}", combined_tree.to_string()); // Output the combined tree
}
