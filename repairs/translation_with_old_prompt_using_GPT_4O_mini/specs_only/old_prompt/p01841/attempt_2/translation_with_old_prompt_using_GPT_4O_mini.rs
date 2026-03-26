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
    let mut value = 0;
    let mut left = String::new();
    let mut right = String::new();
    let mut state = 0; // 0: reading left, 1: reading value, 2: reading right

    let mut chars = tree.ptr.chars().peekable();
    while let Some(c) = chars.next() {
        match c {
            '(' => {
                if state == 0 {
                    // Start of left subtree
                    left.push(c);
                    state = 0; // still in left
                } else {
                    // Start of right subtree
                    right.push(c);
                    state = 2; // now in right
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
                state = 1; // reading value
            }
            ']' => {
                state = 0; // finished reading value
            }
            _ if state == 1 => {
                // Accumulate value
                value = value * 10 + c.to_digit(10).unwrap_or(0) as i32;
            }
            _ => {}
        }
    }

    (value, left, right)
}
