// Import necessary libraries
use rand::Rng; // For randomization based on system time
use std::time::{SystemTime, UNIX_EPOCH};

// Function to calculate the optimal schedule
fn calculate_optimal_schedule(days: usize, costs: &[i32], satisfaction_scores: &[Vec<i32>]) -> Vec<usize> {
    // Ensure the number of days is valid
    assert!(days > 0, "Number of days must be a positive integer.");
    let types_count = costs.len();
    
    // Initialize the schedule with the first type for each day
    let mut schedule: Vec<usize> = vec![0; days];
    
    // Random number generator seeded with current time
    let seed = SystemTime::now().duration_since(UNIX_EPOCH).unwrap().as_nanos() as u64;
    let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
    
    // Function to calculate total satisfaction score based on the current schedule
    fn calculate_satisfaction(schedule: &[usize], satisfaction_scores: &[Vec<i32>]) -> i32 {
        schedule.iter()
            .enumerate()
            .map(|(day, &type_index)| satisfaction_scores[day][type_index])
            .sum()
    }
    
    // Initial satisfaction score
    let mut best_score = calculate_satisfaction(&schedule, satisfaction_scores);
    
    // Attempt to improve the schedule by swapping types
    for _ in 0..1000 { // Arbitrary number of iterations for randomization
        let day1 = rng.gen_range(0..days);
        let day2 = rng.gen_range(0..days);
        
        // Swap types for two days
        schedule.swap(day1, day2);
        
        // Calculate new satisfaction score
        let new_score = calculate_satisfaction(&schedule, satisfaction_scores);
        
        // If the new score is better, keep the new schedule
        if new_score > best_score {
            best_score = new_score;
        } else {
            // Otherwise, swap back to maintain the previous schedule
            schedule.swap(day1, day2);
        }
    }
    
    // Return the optimal schedule
    schedule
}

// Main function to demonstrate the usage
fn main() {
    // Example inputs
    let days = 5;
    let costs = vec![1, 2, 3]; // Costs for each type
    let satisfaction_scores = vec![
        vec![10, 20, 30], // Satisfaction scores for day 0
        vec![15, 25, 35], // Satisfaction scores for day 1
        vec![20, 30, 40], // Satisfaction scores for day 2
        vec![25, 35, 45], // Satisfaction scores for day 3
        vec![30, 40, 50], // Satisfaction scores for day 4
    ];
    
    // Calculate the optimal schedule
    let optimal_schedule = calculate_optimal_schedule(days, &costs, &satisfaction_scores);
    
    // Print the optimal schedule
    println!("{:?}", optimal_schedule);
}
