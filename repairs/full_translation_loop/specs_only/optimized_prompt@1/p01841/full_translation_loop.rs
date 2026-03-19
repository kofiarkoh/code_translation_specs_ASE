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
        if s.is_empty() {
            return None; // Return None for empty strings
        }

        let mut stack: Vec<TreeNode> = Vec::new();
        let mut current_node: Option<TreeNode> = None;

        for c in s.chars() {
            match c {
                '(' => {
                    // Start a new node
                    let new_node = TreeNode::new('('); // Placeholder value
                    if let Some(ref mut parent) = current_node {
                        parent.children.push(new_node);
                    }
                    stack.push(new_node);
                    current_node = Some(stack.last_mut().unwrap());
                }
                ')' => {
                    // End current node
                    if let Some(node) = stack.pop() {
                        current_node = if stack.is_empty() {
                            None // Finished parsing
                        } else {
                            Some(stack.last_mut().unwrap())
                        };
                    }
                }
                '[' => {
                    // Start a new child node
                    let new_node = TreeNode::new('['); // Placeholder value
                    if let Some(ref mut parent) = current_node {
                        parent.children.push(new_node);
                    }
                    stack.push(new_node);
                    current_node = Some(stack.last_mut().unwrap());
                }
                ']' => {
                    // End current child node
                    if let Some(node) = stack.pop() {
                        current_node = if stack.is_empty() {
                            None // Finished parsing
                        } else {
                            Some(stack.last_mut().unwrap())
                        };
                    }
                }
                '0'..='9' => {
                    // Handle numeric values
                    let new_node = TreeNode::new(c);
                    if let Some(ref mut parent) = current_node {
                        parent.children.push(new_node);
                    }
                }
                _ => {}
            }
        }

        // Return the root node if parsing was successful
        if stack.is_empty() {
            current_node
        } else {
            None
        }
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
