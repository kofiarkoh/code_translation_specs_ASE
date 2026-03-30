use std::io::{self, Read};

#[derive(Debug, Clone)] // Implement Clone to allow cloning of TreeNode
struct TreeNode {
    value: i32,
    children: Vec<TreeNode>,
}

impl TreeNode {
    fn new(value: i32) -> Self {
        TreeNode {
            value,
            children: Vec::new(),
        }
    }

    fn from_string(s: &str) -> Option<(Self, usize)> {
        let mut chars = s.chars();
        let mut value_str = String::new();

        // Read the value until we hit a non-digit character
        while let Some(c) = chars.next() {
            if c.is_digit(10) || c == '-' {
                value_str.push(c);
            } else {
                break;
            }
        }

        let value = value_str.parse::<i32>().ok()?;
        let mut node = TreeNode::new(value);
        let mut consumed = value_str.len();

        // Read children
        while let Some(c) = chars.next() {
            consumed += 1; // Count the character we just read
            if c == '(' {
                // Parse a child subtree
                if let Some((child, child_consumed)) = TreeNode::from_string(chars.as_str()) {
                    node.children.push(child);
                    consumed += child_consumed + 1; // +1 for the closing ')'
                    chars = chars.by_ref().skip(child_consumed); // Use by_ref() to keep the original iterator
                }
            } else if c == ')' {
                break; // End of current node
            }
        }

        Some((node, consumed))
    }

    fn combine(left: &Self, right: &Self) -> Self {
        let combined_value = left.value + right.value;
        let mut combined_node = TreeNode::new(combined_value);

        // Combine children
        let max_children = left.children.len().max(right.children.len());
        for i in 0..max_children {
            if i < left.children.len() {
                combined_node.children.push(left.children[i].clone());
            }
            if i < right.children.len() {
                combined_node.children.push(right.children[i].clone());
            }
        }

        combined_node
    }

    fn to_string(&self) -> String {
        let mut result = format!("{}", self.value);
        if !self.children.is_empty() {
            result.push('(');
            for (i, child) in self.children.iter().enumerate() {
                if i > 0 {
                    result.push(',');
                }
                result.push_str(&child.to_string());
            }
            result.push(')');
        }
        result
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    let trees: Vec<&str> = input.lines().collect();
    if trees.len() != 2 || trees[0].len() < 2 || trees[1].len() < 2 {
        // Handle empty tree case
        println!("()");
        return;
    }

    let (tree1, _) = TreeNode::from_string(trees[0]).unwrap();
    let (tree2, _) = TreeNode::from_string(trees[1]).unwrap();

    let combined_tree = TreeNode::combine(&tree1, &tree2);
    let output = combined_tree.to_string();
    println!("{}", output);
}
