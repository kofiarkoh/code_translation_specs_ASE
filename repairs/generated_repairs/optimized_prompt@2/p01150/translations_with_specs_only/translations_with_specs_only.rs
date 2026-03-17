use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    
    // Read input until zero is encountered
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let n: u32 = match input.trim().parse() {
            Ok(num) if num >= 0 => num, // Allow zero as valid input
            _ => break, // Exit on invalid input
        };

        // If N is greater than 16, we return 0 as per the specification
        if n > 16 {
            println!("0");
            continue;
        }

        // Calculate the number of valid seating arrangements
        let arrangements = calculate_arrangements(n);
        println!("{}", arrangements);
        
        // Clear input for the next iteration
        input.clear();
    }
}

// Function to calculate valid seating arrangements using memoization
fn calculate_arrangements(n: u32) -> u64 {
    // Memoization map to store previously computed results
    let mut memo: HashMap<u32, u64> = HashMap::new();
    // Call the recursive function to compute arrangements
    compute_arrangements(n, &mut memo)
}

// Recursive function to compute arrangements
fn compute_arrangements(n: u32, memo: &mut HashMap<u32, u64>) -> u64 {
    // Base case: if there are no princes, there's one way to arrange them (do nothing)
    if n == 0 {
        return 1;
    }
    // Check if the result is already computed
    if let Some(&result) = memo.get(&n) {
        return result;
    }

    // Calculate arrangements based on whether n is odd or even
    let arrangements = if n % 2 == 0 {
        // For even n, use the formula: n! / (n/2)!^2
        factorial(n) / (factorial(n / 2) * factorial(n / 2))
    } else {
        // For odd n, use the formula: 0 (as per the specification)
        0
    };

    // Store the computed result in the memoization map
    memo.insert(n, arrangements);
    arrangements
}

// Function to compute factorial
fn factorial(n: u32) -> u64 {
    (1..=n).map(|x| x as u64).product()
}
