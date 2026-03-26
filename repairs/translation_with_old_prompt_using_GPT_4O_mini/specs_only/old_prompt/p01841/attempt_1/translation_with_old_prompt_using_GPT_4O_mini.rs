use std::io::{self, Read};

#[derive(Debug)]
struct MyStr<'a> {
    ptr: &'a str,
    len: usize,
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    
    // Split the input into two tree strings
    let trees: Vec<&str> = input.lines().collect();
    let tree1 = MyStr {
        ptr: trees.get(0).unwrap_or(&""),
        len: trees.get(0).map_or(0, |s| s.len()),
    };
    let tree2 = MyStr {
        ptr: trees.get(1).unwrap_or(&""),
        len: trees.get(1).map_or(0, |s| s.len()),
    };

    // Combine the two trees
    let result = combine_trees(tree1, tree2);
    
    // Print the result
    println!("{}", result);
}

// Function to combine two trees
fn combine_trees(tree1: MyStr, tree2: MyStr) -> String {
    // If either tree is empty, return the other tree
    if tree1.len == 0 {
        return tree2.ptr.to_string();
    }
    if tree2.len == 0 {
        return tree1.ptr.to_string();
    }

    // Parse the values from the trees
    let (value1, left1, right1) = parse_tree(tree1);
    let (value2, left2, right2) = parse_tree(tree2);

    // Combine the values
    let combined_value = value1 + value2;

    // Construct the new tree representation
    format!("({})[{}]({})", left1, combined_value, right2)
}

// Function to parse a tree string into its components
fn parse_tree(tree: MyStr) -> (i32, String, String) {
    // This is a simplified parsing logic assuming well-formed input
    // In a real implementation, you would need to handle parsing more robustly
    let value: i32 = tree.ptr.chars().filter(|c| c.is_digit(10)).collect::<String>().parse().unwrap_or(0);
    let left = String::new(); // Placeholder for left subtree
    let right = String::new(); // Placeholder for right subtree

    (value, left, right)
}
