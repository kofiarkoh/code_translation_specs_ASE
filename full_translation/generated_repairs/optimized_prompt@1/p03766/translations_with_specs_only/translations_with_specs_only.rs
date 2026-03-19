fn main() {
    // Read input from standard input
    let n: usize = std::env::args()
        .nth(1)
        .expect("Please provide a positive integer as input.")
        .parse()
        .expect("Input must be a valid positive integer.");

    // Preconditions: n must be a positive integer
    if n == 0 {
        panic!("Please provide a positive integer as input.");
    }

    // Initialize the binary indexed tree (BIT) with size n + 1
    let mut bit = vec![0; n + 1];
    
    // Function to update the BIT
    fn update(bit: &mut Vec<i32>, index: usize, value: i32) {
        let modulus = 1_000_000_007;
        let mut idx = index as i32;
        while idx < bit.len() as i32 {
            bit[idx as usize] = (bit[idx as usize] + value) % modulus;
            idx += idx & -idx; // Move to the next index
        }
    }

    // Function to query the BIT for cumulative frequency
    fn query(bit: &Vec<i32>, index: usize) -> i32 {
        let mut sum = 0;
        let mut idx = index as i32;
        while idx > 0 {
            sum = (sum + bit[idx as usize]) % 1_000_000_007;
            idx -= idx & -idx; // Move to the parent index
        }
        sum
    }

    // Populate the BIT with values from 1 to n
    for i in 1..=n {
        update(&mut bit, i, 1); // Update BIT with value 1 at index i
    }

    // Compute the final result using the BIT
    let result = query(&bit, n); // Query the cumulative sum up to n

    // Postconditions: Output must be non-negative and less than 10^9 + 7
    println!("{}", result);
}
