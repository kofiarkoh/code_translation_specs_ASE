use std::io;

fn main() {
    let mut input = String::new();
    
    // Read N from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: i32 = input.trim().parse().expect("Please enter a valid integer for N");

    input.clear();
    
    // Read A and B from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Please enter valid integers for A and B"))
        .collect();
    
    let a = parts[0];
    let b = parts[1];

    let mut count = 0;

    // Iterate from 1 to N
    for i in 1..=n {
        // Calculate the digit sum
        let sum = digit_sum(i);
        
        // Check if the digit sum is within the range [A, B]
        if a <= sum && sum <= b {
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
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `std::io` for reading input, which is idiomatic in Rust. The input is read as a string and then parsed into integers.
2. **Digit Sum Calculation**: Created a separate function `digit_sum` to encapsulate the logic for calculating the sum of digits, which improves readability and maintains separation of concerns.
3. **Looping and Conditions**: The loop iterates from 1 to N, checking the digit sum against the bounds A and B, similar to the C code.
4. **Output**: Used `println!` for output, which is the standard way to print in Rust.
5. **Safety**: The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees. The digit extraction is done using a simple loop, which is safe and idiomatic.
6. **Error Handling**: The code assumes valid input as per the specification, similar to the C code, which does not handle invalid input gracefully.
