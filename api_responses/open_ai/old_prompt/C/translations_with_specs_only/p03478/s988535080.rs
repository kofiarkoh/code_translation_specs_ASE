fn main() {
    // Read inputs N, A, and B from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse N, A, and B as integers
    let n: i32 = iter.next().unwrap().parse().expect("Invalid input for N");
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input for A");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input for B");

    // Initialize the cumulative sum
    let mut count = 0;

    // Iterate from 1 to N (inclusive)
    for i in 1..=n {
        // Calculate the digit sum of i
        let digit_sum = digit_sum(i);
        
        // Check if the digit sum is within the range [A, B]
        if digit_sum >= a && digit_sum <= b {
            count += i; // Add i to count if the condition is met
        }
    }

    // Print the result
    println!("{}", count);
}

// Function to calculate the sum of digits of a number
fn digit_sum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10; // Add the last digit to sum
        num /= 10;       // Remove the last digit
    }
    sum
}
