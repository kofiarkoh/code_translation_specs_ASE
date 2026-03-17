use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the valid range
    if n < 1 || n > 100 {
        eprintln!("N must be between 1 and 100");
        return;
    }

    // Initialize the array to hold the integers
    let mut a = vec![0; 101];

    // Read the integers into the array
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Invalid integer input");
    }

    // Count consecutive duplicates
    let mut count = 0;
    let mut i = 2; // Start from the second index
    while i <= n {
        if a[i] == a[i - 1] {
            count += 1;
            i += 1; // Skip the next element as it's a duplicate
        }
        i += 1; // Move to the next element
    }

    // Print the result
    println!("{}", count);
}
```

### Notes:
1. **Input Handling**: The Rust code reads from standard input and expects the first line to contain the integer `N`, followed by `N` integers. This matches the specification.
2. **Array Initialization**: The array `a` is initialized with a size of 101, as specified, to accommodate the maximum number of integers.
3. **Consecutive Duplicate Counting**: The logic for counting consecutive duplicates is preserved from the C code, ensuring that the semantics are maintained.
4. **Error Handling**: The code checks if `N` is within the valid range (1 to 100) and prints an error message if not. This is a safeguard against invalid input.
5. **Output**: The output format is preserved, printing the count followed by a newline.
6. **Safety**: The code avoids any use of `unsafe` Rust, adhering to the requirement for safe and idiomatic Rust.
