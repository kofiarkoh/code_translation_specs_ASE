use std::io;

fn main() {
    let mut input = String::new();
    
    // Read N
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Invalid input for N");

    input.clear();
    
    // Read A and B
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut bounds = input.trim().split_whitespace();
    let a: i32 = bounds.next().expect("Missing A").parse().expect("Invalid input for A");
    let b: i32 = bounds.next().expect("Missing B").parse().expect("Invalid input for B");

    let mut count = 0;

    // Iterate from 1 to N
    for i in 1..=n {
        let sum_of_digits = digit_sum(i);
        if a <= sum_of_digits && sum_of_digits <= b {
            count += i;
        }
    }

    // Print the result
    println!("{}", count);
}

// Function to calculate the sum of digits of a number
fn digit_sum(mut num: i32) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += num % 10;
        num /= 10;
    }
    sum
}
