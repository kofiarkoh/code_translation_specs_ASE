const MD: i32 = 1_000_000_007; // Modulo value as per specification
const N: usize = 8; // Maximum digit length

// Array to hold precomputed values for digit combinations
let mut cnt: [i32; N + 1] = [0; N + 1];

// Function to initialize the cnt array
fn init(cnt: &mut [i32; N + 1]) {
    // Precompute values for digit lengths from 1 to N
    for i in 1..=N {
        cnt[i] = 1; // Base case: one way to form a number with a single digit
        for j in 1..=9 {
            if j <= i {
                cnt[i] = (cnt[i] + cnt[i - j]) % MD; // Count combinations
            }
        }
    }
}

// Function to calculate the number of valid combinations
fn count_combinations(s: i32, cnt: &[i32; N + 1]) -> i32 {
    if s < 1 {
        return 0; // Undefined behavior for s < 1, return 0
    }
    
    let mut ans = 0; // Variable to hold the total count of combinations
    for i in 1..=N {
        if s >= i {
            ans = (ans + cnt[i]) % MD; // Accumulate valid combinations
        }
    }
    ans // Return the total count of valid combinations
}

fn main() {
    let s: i32 = 10; // Example input, should be replaced with actual input handling
    init(&mut cnt); // Initialize the cnt array
    let result = count_combinations(s, &cnt); // Calculate combinations
    println!("{}", result); // Output the result
}
