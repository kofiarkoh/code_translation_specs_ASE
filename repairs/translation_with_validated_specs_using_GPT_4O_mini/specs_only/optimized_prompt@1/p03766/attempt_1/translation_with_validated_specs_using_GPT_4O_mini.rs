struct BIT {
    size: usize,
    tree: Vec<i64>,
}

impl BIT {
    fn new(size: usize) -> Self {
        BIT {
            size,
            tree: vec![0; size + 1],
        }
    }

    fn update(&mut self, index: usize, value: i64) {
        let mut idx = index as i64;
        while idx <= self.size as i64 {
            self.tree[idx as usize] = (self.tree[idx as usize] + value) % 1_000_000_007;
            idx += idx & -idx; // Move to the next index
        }
    }

    fn query(&self, index: usize) -> i64 {
        let mut sum = 0;
        let mut idx = index as i64;
        while idx > 0 {
            sum = (sum + self.tree[idx as usize]) % 1_000_000_007;
            idx -= idx & -idx; // Move to the parent index
        }
        sum
    }
}

fn compute_value(n: usize) -> i64 {
    const MOD: i64 = 1_000_000_007;

    // Initialize the BIT
    let mut bit = BIT::new(n);
    
    // Initialize the first two values
    let mut tmp = 0;
    if n >= 1 {
        bit.update(1, 1); // Set BIT[1] = 1
        tmp = 1; // tmp = 1 for n = 1
    }
    if n >= 2 {
        bit.update(2, 1); // Set BIT[2] = 1
        tmp = (tmp + 1) % MOD; // tmp = 2 for n = 2
    }

    // Compute values for indices from 3 to n
    for i in 3..=n {
        let sum = bit.query(i - 3);
        let additional_term1 = (i as i64 - 3) * (n as i64 - 1) % MOD;
        let additional_term2 = (n as i64 - i as i64 + 2) * n as i64 % MOD;
        
        tmp = (sum + additional_term1 + additional_term2) % MOD;

        // Update the BIT with the new computed value
        bit.update(i, tmp);
    }

    // Return the final computed value
    tmp
}

fn main() {
    let n = 5; // Example input
    let result = compute_value(n);
    println!("{}", result); // Output the result
}
