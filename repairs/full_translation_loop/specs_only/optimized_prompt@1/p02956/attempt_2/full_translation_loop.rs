const MOD: i64 = 998244353;

#[derive(Debug, Clone)] // Derive Clone to allow cloning of Pair
struct Pair {
    key: i64,
    identifier: i64,
}

fn process_pairs(pairs: Vec<Pair>) -> i64 {
    // Sort pairs by key, maintaining the relationship between key and identifier
    let mut sorted_pairs = pairs.clone();
    sorted_pairs.sort_by(|a, b| a.key.cmp(&b.key));

    // Initialize a binary indexed tree (BIT) for efficient calculations
    let max_identifier = sorted_pairs.iter().map(|p| p.identifier).max().unwrap_or(0);
    let mut bit = vec![0; (max_identifier + 1) as usize];

    // Function to update the BIT
    fn update(bit: &mut Vec<i64>, index: i64, value: i64) {
        let mut idx = index;
        while idx < bit.len() as i64 {
            bit[idx as usize] = (bit[idx as usize] + value) % MOD;
            idx += idx & -idx; // Move to the next index
        }
    }

    // Function to query the BIT
    fn query(bit: &Vec<i64>, index: i64) -> i64 {
        let mut sum = 0;
        let mut idx = index;
        while idx > 0 {
            sum = (sum + bit[idx as usize]) % MOD;
            idx -= idx & -idx; // Move to the parent index
        }
        sum
    }

    let mut result = 0;

    // Process each pair
    for pair in sorted_pairs {
        // Query the BIT for the current identifier
        let current_sum = query(&bit, pair.identifier);
        result = (result + current_sum) % MOD;

        // Update the BIT with the current identifier
        update(&mut bit, pair.identifier, 1);
    }

    result
}

fn main() {
    // Example input
    let pairs = vec![
        Pair { key: 1, identifier: 2 },
        Pair { key: 3, identifier: 1 },
        Pair { key: 2, identifier: 3 },
        Pair { key: 5, identifier: 4 },
        Pair { key: 4, identifier: 5 },
    ];

    // Process the pairs and print the result
    let result = process_pairs(pairs);
    println!("{}", result);
}
