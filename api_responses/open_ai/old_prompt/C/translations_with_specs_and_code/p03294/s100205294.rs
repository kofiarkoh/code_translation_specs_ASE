use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the first line to get the number of integers N
    let mut lines = handle.lines();
    let N: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is a positive integer
    if N == 0 {
        eprintln!("N must be a positive integer.");
        return;
    }

    // Create a vector to hold the integers
    let mut a = Vec::with_capacity(N);
    
    // Read the next N integers
    for _ in 0..N {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().expect("Invalid integer input");
        a.push(value);
    }

    // Calculate the sum of each integer minus one
    let ans: i32 = a.iter().map(|&x| x - 1).sum();

    // Print the result
    println!("{}", ans);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` for reading input, which is idiomatic and safe. It reads all input lines and parses them accordingly.
2. **Vector Usage**: Instead of using a fixed-size array as in C, a `Vec` is used to dynamically allocate memory for the integers. This is more idiomatic in Rust and avoids potential issues with stack overflow for large inputs.
3. **Error Handling**: The code checks if `N` is zero and prints an error message. This aligns with the specification's precondition that `N` must be a positive integer.
4. **Sum Calculation**: The sum is calculated using an iterator and the `map` function, which is a safe and idiomatic way to transform and sum elements in Rust.
5. **Output**: The output format is preserved exactly as specified, ensuring that the result is printed as a single integer.

This translation adheres to the specification while ensuring safety and idiomatic Rust practices.
