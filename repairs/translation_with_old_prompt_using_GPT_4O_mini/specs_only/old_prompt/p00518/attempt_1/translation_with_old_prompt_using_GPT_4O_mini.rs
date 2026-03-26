fn compute_combinations(n: usize, input: &str) -> i32 {
    // Modulo constant as specified
    const MOD: i32 = 10007;

    // Schedule array to hold counts of valid combinations
    let mut schedule = [0; 64]; // 8 bits for each key state (J, O, I)
    let mut schedule_next = [0; 64];

    // Edge case: if N is 0, return 0 immediately
    if n == 0 {
        return 0;
    }

    // Process each character in the input string
    for c in input.chars().take(n) {
        // Reset the next schedule for the current character
        schedule_next.fill(0);

        // Determine the index based on the character
        let index = match c {
            'J' => 0,
            'O' => 1,
            'I' => 2,
            _ => unreachable!(), // According to the spec, input is always valid
        };

        // Update the schedule based on the current character
        for j in 0..64 {
            // Only update the next schedule if the current state is valid
            if schedule[j] > 0 {
                // Update the next state based on the current key
                schedule_next[j | (1 << index)] = (schedule_next[j | (1 << index)] + schedule[j]) % MOD;
            }
        }

        // Include the new character as a starting point
        schedule_next[1 << index] = (schedule_next[1 << index] + 1) % MOD;

        // Move to the next schedule
        schedule.copy_from_slice(&schedule_next);
    }

    // Calculate the total combinations from the schedule
    let total: i32 = schedule.iter().sum();
    total % MOD
}

fn main() {
    // Example usage
    let n = 5; // Length of the input string
    let input = "JOIJI"; // Input string
    let result = compute_combinations(n, input);
    println!("{}", result); // Output the result
}
