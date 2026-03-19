use std::io::{self, Write};

struct BinaryIndexedTree {
    size: usize,
    tree: Vec<i32>,
}

impl BinaryIndexedTree {
    fn new(size: usize) -> Self {
        // Initialize the binary indexed tree with zeros
        let tree = vec![0; size + 1];
        BinaryIndexedTree { size, tree }
    }

    fn update(&mut self, index: usize, value: i32) {
        let mut idx = index as usize;
        while idx <= self.size {
            self.tree[idx] = (self.tree[idx] + value) % (1_000_000_007); // Maintain modulus
            idx += idx & (!idx + 1); // Move to the next index
        }
    }

    fn query(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as usize;
        while idx > 0 {
            sum = (sum + self.tree[idx]) % (1_000_000_007); // Maintain modulus
            idx -= idx & (!idx + 1); // Move to the parent index
        }
        sum
    }
}

fn compute_value(n: i32) -> i32 {
    let mut bit = BinaryIndexedTree::new(n as usize);
    for i in 1..=n {
        bit.update(i as usize, i * i); // Update the tree with the square of the current index
    }
    bit.query(n as usize) // Query the cumulative sum for the entire range
}

fn main() {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input and ensure it is a positive integer
    if let Ok(n) = input.trim().parse::<i32>() {
        if n > 0 {
            let result = compute_value(n);
            println!("{}", result); // Output the computed result
        }
    }
    // If input is invalid, behavior is undefined as per the specification
}
