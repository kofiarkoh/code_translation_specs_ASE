fn main() {
    // Read the number of iterations from standard input
    let iterations: usize = read_input();

    // Initial amount set to a fixed value (e.g., 1000)
    let initial_amount: i32 = 1000;
    let percentage_increase: f32 = 0.05; // 5% increase per iteration

    // Calculate the final amount after the specified iterations
    let final_amount = calculate_final_amount(initial_amount, percentage_increase, iterations);

    // Output the final amount, rounded up to the nearest thousand
    println!("{}", round_up_to_nearest_thousand(final_amount));
}

fn read_input() -> usize {
    // This function reads a non-negative integer from standard input
    // For simplicity, we assume valid input is provided
    use std::io::{self, BufRead};
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    input.trim().parse().unwrap() // This will panic if input is invalid, but we assume valid input as per spec
}

fn calculate_final_amount(initial: i32, percentage: f32, iterations: usize) -> i32 {
    let mut amount = initial;

    for _ in 0..iterations {
        // Calculate the increase
        let increase = (amount as f32 * percentage).round() as i32;
        amount += increase;

        // Ensure the amount remains non-negative (invariant)
        assert!(amount >= 0);
    }

    amount
}

fn round_up_to_nearest_thousand(value: i32) -> i32 {
    // Round up to the nearest thousand
    if value % 1000 == 0 {
        value
    } else {
        (value / 1000 + 1) * 1000
    }
}
