fn simulate(n: usize, mut p: usize) -> Option<usize> {
    // Preconditions check
    if n == 0 {
        return None; // If n is zero, terminate without output
    }

    // Initialize the count array to zero
    let mut counts = vec![0; n];
    let mut current_index = 0;

    loop {
        // Decrement the counter
        if p == 0 {
            // Check if all other elements have been incremented
            let all_other_zero = counts.iter().enumerate().all(|(i, &count)| i == current_index || count == 0);
            if all_other_zero {
                return Some(current_index); // Return the current index if condition is met
            } else {
                // Reset the decrementing counter based on the current index's count
                p = counts[current_index];
                counts[current_index] = 0; // Reset the count for the current index
            }
        } else {
            // Simulate incrementing the count for the current index
            counts[current_index] += 1;
            p -= 1; // Decrement the counter
        }

        // Move to the next index, wrapping around if necessary
        current_index = (current_index + 1) % n;
    }
}
