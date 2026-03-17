use std::io;

fn main() {
    let mut input = String::new();
    
    // Read the upper limit N
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid input for N");

    input.clear();
    
    // Read the range A and B
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let parts: Vec<&str> = input.trim().split_whitespace().collect();
    let a: usize = parts[0].parse().expect("Invalid input for A");
    let b: usize = parts[1].parse().expect("Invalid input for B");

    let mut count = 0;

    // Iterate through each integer from 1 to N
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
fn digit_sum(mut num: usize) -> usize {
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
2. **Digit Sum Calculation**: Implemented a separate function `digit_sum` to calculate the sum of digits, which is more idiomatic and modular than manually extracting digits as in the C code.
3. **Looping and Conditions**: The loop iterates from 1 to `N` using Rust's range syntax, and the condition checks are straightforward, preserving the logic from the C code.
4. **Output**: Used `println!` for output, which matches the behavior of `printf` in C.
5. **Error Handling**: The program will panic if the input is invalid, which aligns with the specification's undefined behavior for invalid inputs.
