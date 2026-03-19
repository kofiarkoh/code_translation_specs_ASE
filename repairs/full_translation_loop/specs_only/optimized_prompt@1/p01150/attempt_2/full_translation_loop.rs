use std::collections::HashMap;
use std::io;

fn main() {
    let mut input = String::new();
    while io::stdin().read_line(&mut input).unwrap() > 0 {
        let n: usize = input.trim().parse().unwrap();
        if n == 0 {
            break;
        }
        // According to the specification, if N is less than or equal to 16, output should be zero.
        if n <= 16 {
            println!("0");
            continue;
        }
        // Calculate the number of valid seating arrangements for N princes.
        let arrangements = count_arrangements(n);
        println!("{}", arrangements);
    }
}

// Memoization structure to store previously computed results.
fn count_arrangements(n: usize) -> u64 {
    let mut memo = HashMap::new();
    calculate_arrangements(n, &mut memo)
}

// Recursive function to calculate arrangements with memoization.
fn calculate_arrangements(n: usize, memo: &mut HashMap<usize, u64>) -> u64 {
    // Base case: if there are no princes, there's one way to arrange them (do nothing).
    if n == 0 {
        return 1;
    }
    // Check if the result is already computed.
    if let Some(&result) = memo.get(&n) {
        return result;
    }
    // Recursive case: calculate arrangements based on whether n is odd or even.
    let result = if n % 2 == 0 {
        // For even n, the formula is n! / (n/2)! * (n/2)!
        factorial(n) / (factorial(n / 2) * factorial(n / 2))
    } else {
        // For odd n, the formula is 0 (as per the specification).
        0
    };
    // Store the computed result in the memoization structure.
    memo.insert(n, result);
    result
}

// Function to calculate factorial of a number.
fn factorial(n: usize) -> u64 {
    // Use u64 to ensure the product is computed correctly.
    (1..=n).map(|x| x as u64).product()
}
