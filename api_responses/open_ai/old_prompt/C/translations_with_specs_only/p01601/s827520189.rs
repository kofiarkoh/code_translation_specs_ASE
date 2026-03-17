use std::io;

fn main() {
    // Read input from the user
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a non-negative integer
    let n: u32 = match input.trim().parse() {
        Ok(num) if num >= 0 => num, // Ensure n is non-negative
        _ => {
            // Undefined behavior on invalid input; we do nothing
            return;
        }
    };

    // Find and print the nearest palindrome
    let nearest_palindrome = find_nearest_palindrome(n);
    println!("{}", nearest_palindrome);
}

// Function to find the nearest palindrome to a given number
fn find_nearest_palindrome(n: u32) -> u32 {
    let mut lower = n;
    let mut upper = n;

    // Check for the nearest palindrome in both directions
    loop {
        if is_palindrome(lower) {
            return lower; // Return the first palindrome found going down
        }
        if is_palindrome(upper) {
            return upper; // Return the first palindrome found going up
        }
        // Decrement lower and increment upper
        if lower > 0 {
            lower -= 1; // Ensure lower does not go negative
        }
        upper += 1; // Increment upper
    }
}

// Function to check if a number is a palindrome
fn is_palindrome(num: u32) -> bool {
    let original = num;
    let mut reversed = 0;
    let mut n = num;

    // Reverse the number
    while n > 0 {
        reversed = reversed * 10 + n % 10; // Build the reversed number
        n /= 10; // Remove the last digit
    }

    // Check if the original number is equal to its reverse
    original == reversed
}
