use std::time::{Duration, Instant};

fn calculate_optimal_schedule(days: usize, costs: &[i32], satisfaction_scores: &[Vec<i32>]) -> Vec<usize> {
    // Initialize the schedule with the first type for each day
    let mut schedule: Vec<usize> = vec![0; days];
    
    // Track the last occurrence of each type
    let mut last_occurrence: Vec<usize> = vec![0; costs.len()];

    // Start the optimization process
    let start_time = Instant::now();
    while start_time.elapsed() < Duration::new(2, 0) {
        // Calculate the current satisfaction score
        let current_score = calculate_satisfaction_score(&schedule, &satisfaction_scores);
        
        // Randomly swap types in the schedule to explore new configurations
        let day1 = rand::random::<usize>() % days; // Use a simple random method
        let day2 = rand::random::<usize>() % days; // Use a simple random method
        
        // Swap the types for day1 and day2
        schedule.swap(day1, day2);
        
        // Calculate the new satisfaction score after the swap
        let new_score = calculate_satisfaction_score(&schedule, &satisfaction_scores);
        
        // If the new score is better, keep the new schedule
        if new_score < current_score {
            // Revert the swap
            schedule.swap(day1, day2);
        } else {
            // Update last occurrence for the type used on day1 and day2
            last_occurrence[schedule[day1]] = day1;
            last_occurrence[schedule[day2]] = day2;
        }
    }
    
    schedule
}

// Helper function to calculate the total satisfaction score based on the current schedule
fn calculate_satisfaction_score(schedule: &[usize], satisfaction_scores: &[Vec<i32>]) -> i32 {
    let mut total_score = 0;
    for (day, &type_index) in schedule.iter().enumerate() {
        total_score += satisfaction_scores[day][type_index];
    }
    total_score
}

fn main() {
    // Example usage
    let days = 5;
    let costs = vec![1, 2, 3];
    let satisfaction_scores = vec![
        vec![10, 20, 30],
        vec![15, 25, 35],
        vec![20, 30, 40],
        vec![25, 35, 45],
        vec![30, 40, 50],
    ];
    
    let optimal_schedule = calculate_optimal_schedule(days, &costs, &satisfaction_scores);
    println!("{:?}", optimal_schedule);
}
