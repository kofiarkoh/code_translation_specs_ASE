use std::io::{self, Write};

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
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `std::io` for reading input, which is idiomatic in Rust. The input is read as a string and then parsed into integers.
2. **Digit Sum Calculation**: Instead of manually extracting each digit as in the C code, a helper function `digit_sum` is created to compute the sum of digits in a more idiomatic and maintainable way.
3. **Looping and Conditions**: The loop iterates from 1 to N using Rust's range syntax, which is clear and concise.
4. **Output**: The result is printed using `println!`, which automatically handles formatting.
5. **Error Handling**: The code assumes valid input as per the specification, similar to the C code. Invalid input handling is not implemented, as per the spec's behavior.
6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
