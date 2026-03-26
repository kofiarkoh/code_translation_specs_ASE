use std::io::{self, Write};

fn f(n: i64, x: i64) -> i64 {
    if n == 0 {
        return if x <= 0 { 0 } else { 1 };
    }
    let p = (1i64 << n) - 1; // Calculate the number of nodes in a complete binary tree of depth n
    if x <= 2 * p {
        f(n - 1, x - 1) // Recursive call for the left subtree
    } else {
        p + 1 + f(n - 1, x - 1 - 2 * p) // Recursive call for the right subtree
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    let n: i64 = iter.next().unwrap().parse().expect("Invalid input for n");
    let k: i64 = iter.next().unwrap().parse().expect("Invalid input for k");
    
    // Output the result of the function f
    println!("{}", f(n, k));
}
