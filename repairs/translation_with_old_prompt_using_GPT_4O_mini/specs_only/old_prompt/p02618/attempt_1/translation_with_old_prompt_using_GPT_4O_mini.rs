fn optimize_schedule(D: usize, s: [[usize; 27]; 27], c: [usize; 27], last: &mut [usize; 27]) -> [usize; 27] {
    // Initialize the schedule array
    let mut schedule = [0; 27];

    // Ensure D is positive
    if D == 0 {
        return schedule; // Early return if D is not valid
    }

    // Initial scheduling based on some heuristic (e.g., greedy approach)
    for day in 0..D {
        let mut best_type = 1;
        let mut best_score = 0;

        // Find the type with the highest satisfaction score for the current day
        for typ in 1..=26 {
            let score = s[day][typ] - c[typ]; // Calculate score considering cost
            if score > best_score && last[typ] < day { // Ensure type hasn't been scheduled recently
                best_score = score;
                best_type = typ;
            }
        }

        // Schedule the best type for the day
        schedule[day] = best_type;
        last[best_type] = day; // Update last scheduled day for this type
    }

    // Optimization loop to improve the schedule
    for _ in 0..1000 { // Arbitrary number of iterations for optimization
        for day in 0..D {
            let current_type = schedule[day];
            let mut best_type = current_type;
            let mut best_score = 0;

            // Try swapping with other types
            for typ in 1..=26 {
                if typ != current_type && last[typ] < day { // Ensure type hasn't been scheduled recently
                    let score = s[day][typ] - c[typ];
                    if score > best_score {
                        best_score = score;
                        best_type = typ;
                    }
                }
            }

            // If a better type is found, swap it
            if best_type != current_type {
                schedule[day] = best_type;
                last[current_type] = 0; // Reset last for the old type
                last[best_type] = day; // Update last for the new type
            }
        }
    }

    // Print the final schedule
    for day in 0..D {
        println!("{}", schedule[day]);
    }

    schedule // Return the final schedule
}
