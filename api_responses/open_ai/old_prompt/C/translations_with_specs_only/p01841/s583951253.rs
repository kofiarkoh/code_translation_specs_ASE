use std::io::{self, Read};

#[derive(Debug)]
struct MyStr {
    ptr: *const char,
    len: usize,
}

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split the input into two tree strings
    let trees: Vec<&str> = input.lines().collect();
    if trees.len() != 2 {
        panic!("Expected exactly two tree strings");
    }

    let tree1 = trees[0];
    let tree2 = trees[1];

    // Process the trees and combine them
    let result = combine_trees(tree1, tree2);

    // Print the resulting tree
    println!("{}", result);
}

// Function to combine two trees represented as strings
fn combine_trees(tree1: &str, tree2: &str) -> String {
    // Parse the trees and sum their values
    let (left1, value1, right1) = parse_tree(tree1);
    let (left2, value2, right2) = parse_tree(tree2);

    // Combine the values
    let combined_value = value1 + value2;

    // Construct the new tree representation
    format!("({})[{}]({})", left1, combined_value, right2)
}

// Function to parse a tree string into its components
fn parse_tree(tree: &str) -> (String, i32, String) {
    // This is a simplified parser; it assumes the input is well-formed
    // and extracts left subtree, value, and right subtree.
    let mut left = String::new();
    let mut value = 0;
    let mut right = String::new();
    let mut state = 0; // 0: left, 1: value, 2: right

    let mut current_number = String::new();

    for c in tree.chars() {
        match c {
            '(' => {
                if state == 0 {
                    // Start of left subtree
                    left.push(c);
                } else {
                    // Start of right subtree
                    right.push(c);
                }
            }
            ')' => {
                if state == 0 {
                    left.push(c);
                } else {
                    right.push(c);
                }
            }
            '[' => {
                state = 1; // Move to value state
            }
            ']' => {
                state = 2; // Move to right state
                if !current_number.is_empty() {
                    value = current_number.parse().unwrap();
                    current_number.clear();
                }
            }
            _ => {
                if state == 1 {
                    current_number.push(c);
                } else {
                    if state == 0 {
                        left.push(c);
                    } else {
                        right.push(c);
                    }
                }
            }
        }
    }

    (left, value, right)
}
