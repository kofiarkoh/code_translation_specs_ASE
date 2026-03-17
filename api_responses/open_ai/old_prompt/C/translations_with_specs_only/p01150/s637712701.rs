use std::io;

fn main() {
    let mut input = String::new();
    // Read input until a zero is encountered
    while io::stdin().read_line(&mut input).is_ok() {
        let n: i32 = match input.trim().parse() {
            Ok(num) if num > 0 && num <= 16 => num,
            Ok(0) => break, // Termination signal
            _ => {
                input.clear(); // Ignore invalid input
                continue;
            }
        };

        // Calculate and print the number of valid seating arrangements
        let arrangements = count_arrangements(n);
        println!("{}", arrangements);
        input.clear(); // Clear input for the next iteration
    }
}

// Function to count valid seating arrangements using memoization
fn count_arrangements(n: i32) -> i64 {
    // Memoization array: 70 x 9 x 3 x 3
    let mut memo = vec![vec![vec![vec![-1; 3]; 3]; 9]; 70];

    // Start the recursive calculation
    calculate_arrangements(n, 0, 0, 0, &mut memo)
}

// Recursive function to calculate arrangements
fn calculate_arrangements(n: i32, pos: i32, left: i32, right: i32, memo: &mut Vec<Vec<Vec<Vec<i64>>>>) -> i64 {
    // Base case: if all princes are seated
    if pos == n {
        return 1;
    }

    // Check memoization
    if memo[pos as usize][left as usize][right as usize] != -1 {
        return memo[pos as usize][left as usize][right as usize];
    }

    let mut result = 0;

    // Try to seat a prince at the current position
    // Here we assume left and right represent the seating status
    // The logic for seating can be adjusted based on specific rules
    if left < 3 {
        result += calculate_arrangements(n, pos + 1, left + 1, right, memo);
    }
    if right < 3 {
        result += calculate_arrangements(n, pos + 1, left, right + 1, memo);
    }

    // Store the result in memoization array
    memo[pos as usize][left as usize][right as usize] = result;
    result
}
