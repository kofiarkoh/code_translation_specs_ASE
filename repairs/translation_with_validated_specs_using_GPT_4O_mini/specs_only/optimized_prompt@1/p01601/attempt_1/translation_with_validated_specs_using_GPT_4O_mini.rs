fn is_palindrome(n: u32) -> bool {
    let s = n.to_string();
    s == s.chars().rev().collect::<String>()
}

fn nearest_palindrome(input: u32) -> u32 {
    // Start searching from the input value
    let mut lower = input;
    let mut upper = input;

    // Check if the input itself is a palindrome
    if is_palindrome(input) {
        return input;
    }

    // Search for the nearest palindromes
    loop {
        // Check lower palindrome
        if lower > 0 {
            if is_palindrome(lower) {
                return lower;
            }
            lower -= 1; // Decrement to check the next lower number
        }

        // Check upper palindrome
        if is_palindrome(upper) {
            return upper;
        }
        upper += 1; // Increment to check the next higher number
    }
}

fn main() {
    let input: u32 = 123; // Example input, replace with actual input as needed
    let result = nearest_palindrome(input);
    println!("{}", result); // Output the nearest palindrome
}
