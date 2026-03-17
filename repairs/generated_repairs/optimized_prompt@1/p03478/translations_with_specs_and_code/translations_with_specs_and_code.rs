use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the upper limit N
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    input.clear();
    
    // Read the range A and B
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input for A or B"))
        .collect();
    
    let a = parts[0];
    let b = parts[1];

    let mut count = 0;

    // Iterate through each integer from 1 to N
    for i in 1..=n {
        let sum_of_digits = sum_digits(i);
        if a <= sum_of_digits && sum_of_digits <= b {
            count += i;
        }
    }

    // Print the result
    println!("{}", count);
}

// Function to calculate the sum of digits of a number
fn sum_digits(mut num: usize) -> i32 {
    let mut sum = 0;
    while num > 0 {
        sum += (num % 10) as i32;
        num /= 10;
    }
    sum
}

// Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` for reading input, which is idiomatic in Rust. It handles potential parsing errors with `expect`, which aligns with the specification's undefined behavior for invalid inputs.

2. **Digit Sum Calculation**: The digit sum is calculated using a separate function `sum_digits`, which iteratively extracts digits. This is a safe and clear approach compared to the original C code's manual extraction.

3. **Loop and Conditionals**: The loop iterates from 1 to `N`, checking if the digit sum falls within the specified range `[A, B]`, and accumulates the valid integers in `count`.

4. **Output**: The result is printed in the same format as the C code, ensuring that the output behavior is preserved.

5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety principles while maintaining the required functionality.
