// Import necessary libraries for randomization and time measurement
use rand::seq::SliceRandom; // For random shuffling
use rand::thread_rng; // Import thread_rng for random number generation
use std::time::Instant; // For measuring execution time

// Function to calculate the optimal schedule
fn calculate_optimal_schedule(days: usize, costs: &[i32], satisfaction_scores: &[Vec<i32>]) -> Vec<usize> {
    // Initialize the schedule with the first type for each day
    let mut schedule: Vec<usize> = vec![0; days];
    
    // Start measuring time for performance constraint
    let start_time = Instant::now();

    // Main optimization loop
    while start_time.elapsed().as_secs_f32() < 2.0 {
        // Calculate current satisfaction score
        let current_score = calculate_satisfaction_score(&schedule, costs, satisfaction_scores);
        
        // Create a new schedule by shuffling the current schedule
        let mut new_schedule = schedule.clone();
        new_schedule.shuffle(&mut thread_rng()); // Use thread_rng() here

        // Calculate new satisfaction score
        let new_score = calculate_satisfaction_score(&new_schedule, costs, satisfaction_scores);

        // If the new score is better, update the schedule
        if new_score > current_score {
            schedule = new_schedule;
        }
    }

    schedule
}

// Function to calculate the total satisfaction score based on the current schedule
fn calculate_satisfaction_score(schedule: &[usize], costs: &[i32], satisfaction_scores: &[Vec<i32>]) -> i32 {
    let mut total_score = 0;

    for (day, &type_index) in schedule.iter().enumerate() {
        // Ensure type_index is within bounds
        if type_index < satisfaction_scores[day].len() {
            total_score += satisfaction_scores[day][type_index];
        }
    }

    total_score
}

// Main function to demonstrate the usage of the scheduling function
fn main() {
    // Example inputs
    let days = 5; // Number of days
    let costs = vec![10, 20, 30]; // Costs associated with each type
    let satisfaction_scores = vec![
        vec![1, 2, 3], // Satisfaction scores for day 0
        vec![4, 5, 6], // Satisfaction scores for day 1
        vec![7, 8, 9], // Satisfaction scores for day 2
        vec![10, 11, 12], // Satisfaction scores for day 3
        vec![13, 14, 15], // Satisfaction scores for day 4
    ];

    // Calculate the optimal schedule
    let optimal_schedule = calculate_optimal_schedule(days, &costs, &satisfaction_scores);

    // Output the optimal schedule
    println!("{:?}", optimal_schedule);
}

// Add the rand crate to your Cargo.toml
// [dependencies]
// rand = "0.8"
