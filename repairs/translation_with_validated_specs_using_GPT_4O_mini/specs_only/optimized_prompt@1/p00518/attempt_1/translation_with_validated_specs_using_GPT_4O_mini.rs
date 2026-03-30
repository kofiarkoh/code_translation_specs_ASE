fn process_keys(n: usize, input: &str) -> i32 {
    const MODULO: i32 = 10007;
    // The schedule array tracks combinations of keys
    let mut schedule = [0; 8]; // 3 keys -> 2^3 = 8 combinations

    // Map characters to their corresponding binary values
    let key_map = |c: char| match c {
        'J' => 1, // 001
        'O' => 2, // 010
        'I' => 4, // 100
        _ => 0,   // Invalid, but we assume valid input as per the spec
    };

    // Initialize the schedule with the first character
    if n > 0 {
        let first_key = key_map(input.chars().nth(0).unwrap());
        schedule[first_key] += 1; // Start with the first key
    }

    // Process each character in the input string
    for i in 1..n {
        let current_key = key_map(input.chars().nth(i).unwrap());
        let mut schedule_next = [0; 8];

        // Update the schedule based on the current key and previous combinations
        for j in 0..8 {
            if schedule[j] > 0 {
                // If the current key and previous keys form a valid combination
                let new_combination = j | current_key; // Bitwise OR to form new combinations
                schedule_next[new_combination] = (schedule_next[new_combination] + schedule[j]) % MODULO;
            }
        }

        // Update the schedule for the next iteration
        schedule = schedule_next;
        // Also include the current key as a new combination
        schedule[current_key] = (schedule[current_key] + 1) % MODULO;
    }

    // Calculate the final cumulative value
    let cumulative_value: i32 = schedule.iter().sum();
    cumulative_value % MODULO
}

fn main() {
    let n = 5; // Example input length
    let input = "JOIJI"; // Example input string
    let result = process_keys(n, input);
    println!("{}", result);
}
