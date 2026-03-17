fn main() {
    // Example input values for testing
    let n: usize = 5; // Number of characters to process
    let input_string: &str = "JOIJI"; // Input string of length N

    // Ensure the input string length matches N
    assert_eq!(input_string.len(), n);

    // Initialize the schedule array to maintain counts of valid combinations
    let mut schedule = [0; 8]; // 2^3 = 8 possible states for 'J', 'O', 'I'
    schedule[0] = 1; // Start with the initial state (no keys pressed)

    // Process each character in the input string
    for ch in input_string.chars() {
        let mut new_schedule = [0; 8]; // Temporary array for the new state

        // Update the schedule based on the current character
        match ch {
            'J' => {
                for state in 0..8 {
                    if schedule[state] > 0 {
                        new_schedule[state | 0b001] = (new_schedule[state | 0b001] + schedule[state]) % 10007; // Set J
                    }
                }
            }
            'O' => {
                for state in 0..8 {
                    if schedule[state] > 0 {
                        new_schedule[state | 0b010] = (new_schedule[state | 0b010] + schedule[state]) % 10007; // Set O
                    }
                }
            }
            'I' => {
                for state in 0..8 {
                    if schedule[state] > 0 {
                        new_schedule[state | 0b100] = (new_schedule[state | 0b100] + schedule[state]) % 10007; // Set I
                    }
                }
            }
            _ => {
                // Undefined behavior for invalid characters; no action taken
            }
        }

        // Update the schedule to the new state
        schedule = new_schedule;
    }

    // Calculate the cumulative result from the schedule
    let result: i32 = schedule.iter().sum::<i32>() % 10007;

    // Print the result to standard output
    println!("{}", result);
}
