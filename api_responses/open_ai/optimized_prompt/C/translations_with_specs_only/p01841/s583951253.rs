use std::io::{self, Read};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read input");

    // Split the input into two tree strings
    let trees: Vec<&str> = input.lines().collect();
    if trees.len() != 2 {
        // If we don't have exactly two trees, we can't proceed
        return;
    }

    let tree1 = trees[0];
    let tree2 = trees[1];

    // Ensure both trees have a length of at least 2
    if tree1.len() < 2 || tree2.len() < 2 {
        // If either tree is too short, represent the corresponding subtree as empty
        println!("()");
        return;
    }

    // Combine the two trees into a new tree representation
    let combined_tree = combine_trees(tree1, tree2);

    // Output the combined tree structure
    println!("{}", combined_tree);
}

// Function to combine two tree strings into a new tree representation
fn combine_trees(tree1: &str, tree2: &str) -> String {
    // This is a simple example of combining two trees.
    // The actual logic for combining trees can vary based on the specific tree structure.
    // Here we assume a simple concatenation for demonstration purposes.
    
    // For a valid rooted tree, we can assume the first character is the root
    // and the rest represents its children.
    let root1 = &tree1[0..1]; // Root of the first tree
    let root2 = &tree2[0..1]; // Root of the second tree

    // Combine roots and their children
    format!("({}{})", root1, root2)
}
