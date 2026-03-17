// Battery management system simulation
fn main() {
    // Example inputs
    let num_batteries = 3; // Number of batteries
    let time_limit = 10; // Time limit for the simulation

    // Battery usage and recharge times
    let battery_cycles = vec![
        vec![2, 3, 5], // Battery 1: usage at 2, recharge at 3, usage at 5
        vec![1, 4, 6], // Battery 2: usage at 1, recharge at 4, usage at 6
        vec![2, 3],    // Battery 3: usage at 2, recharge at 3
    ];

    // Call the function to simulate battery usage
    let total_usages = simulate_battery_management(num_batteries, time_limit, battery_cycles);
    println!("{}", total_usages);
}

// Function to simulate the battery management system
fn simulate_battery_management(num_batteries: usize, time_limit: usize, battery_cycles: Vec<Vec<usize>>) -> usize {
    // Preconditions check
    if num_batteries == 0 || time_limit == 0 {
        return 0; // Terminate if inputs are non-positive
    }

    let mut total_usages = 0;

    // Iterate through each battery's cycles
    for cycles in battery_cycles {
        let mut active = true; // Battery starts as active
        let mut current_time = 0; // Track the current time
        let mut cycle_iter = cycles.iter();

        // Process each cycle
        while let Some(&usage_time) = cycle_iter.next() {
            if usage_time > time_limit {
                break; // Stop if usage time exceeds time limit
            }

            // Count usage if battery is active
            if active {
                total_usages += 1;
            }

            // Move time forward to usage time
            current_time = usage_time;

            // Check for recharge time
            if let Some(&recharge_time) = cycle_iter.next() {
                if recharge_time > time_limit {
                    break; // Stop if recharge time exceeds time limit
                }

                // Transition to inactive after usage
                active = false;

                // Move time forward to recharge time
                current_time = recharge_time;

                // Transition back to active after recharge
                active = true;
            }
        }
    }

    total_usages
}
