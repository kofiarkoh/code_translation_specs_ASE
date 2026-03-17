// Constants
const MD: i32 = 1_000_000_007; // Modulo value for the result
const N: usize = 8; // Maximum digit length

// Function to initialize the count array
fn init() -> [i32; N + 1] {
    let mut cnt = [0; N + 1];
    // Precompute valid combinations for digit lengths
    for i in 1..=N {
        cnt[i] = 1; // Each digit length has at least one valid combination (itself)
    }
    cnt
}

// Function to calculate the number of valid combinations that sum to `s`
fn count_combinations(s: i32) -> i32 {
    if s < 1 {
        return 0; // Undefined behavior for s < 1, return 0 as a safe fallback
    }

    let cnt = init(); // Initialize the count array
    let mut ans = 0; // Variable to hold the total count of valid combinations

    // Nested loops to calculate combinations
    for length in 1..=N {
        for sum in length..=s {
            ans = (ans + cnt[length]) % MD; // Accumulate valid combinations
        }
    }

    ans // Return the total count of valid combinations
}

// Main function to read input and output the result
fn main() {
    let s: i32 = 10; // Example input, replace with actual input handling as needed
    let result = count_combinations(s);
    println!("{}", result); // Output the result
}
