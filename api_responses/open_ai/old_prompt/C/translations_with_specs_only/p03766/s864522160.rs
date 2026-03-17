// Rust implementation of a Binary Indexed Tree (BIT) to compute the nth term of a mathematical sequence.

const MOD: i32 = 1_000_000_007;

struct BIT {
    size: usize,
    tree: Vec<i32>,
}

impl BIT {
    // Create a new BIT of a given size
    fn new(size: usize) -> Self {
        BIT {
            size,
            tree: vec![0; size + 1], // 1-based indexing
        }
    }

    // Update the BIT with a value at a specific index
    fn update(&mut self, index: usize, value: i32) {
        let mut idx = index as usize;
        while idx <= self.size {
            self.tree[idx] = (self.tree[idx] + value) % MOD; // Maintain modulo
            idx += idx & (!idx + 1); // Move to the next index
        }
    }

    // Query the cumulative frequency up to a specific index
    fn query(&self, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as usize;
        while idx > 0 {
            sum = (sum + self.tree[idx]) % MOD; // Maintain modulo
            idx -= idx & (!idx + 1); // Move to the parent index
        }
        sum
    }
}

fn main() {
    let n: usize = read_input(); // Assume a function to read input correctly
    if n < 1 {
        return; // Precondition: n must be >= 1
    }

    let mut bit = BIT::new(n);
    bit.update(1, 1); // Base case: the first term is 1

    for i in 2..=n {
        let value = (bit.query(i - 1) + bit.query(i - 2)) % MOD; // Recurrence relation
        bit.update(i, value); // Update BIT with the new term
    }

    println!("{}", bit.query(n)); // Output the nth term
}

// Function to read input (stub for demonstration)
fn read_input() -> usize {
    // In a real implementation, this would read from standard input
    // For now, we return a fixed value for demonstration purposes
    10 // Example input
}
