use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of integers
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input for N");

    // Ensure N is within the valid range
    if n < 1 || n > 100 {
        eprintln!("N must be between 1 and 100");
        return;
    }

    let mut a = vec![0; 101]; // Create a vector to hold up to 101 integers

    // Read the integers into the array
    for i in 1..=n {
        a[i] = lines.next().unwrap().unwrap().trim().parse().expect("Invalid integer input");
    }

    let mut count = 0;
    let mut i = 1; // Start from the first element

    // Count consecutive duplicates
    while i < n {
        if a[i] == a[i + 1] {
            count += 1;
            i += 1; // Skip the next element as it's part of the duplicate
        }
        i += 1; // Move to the next element
    }

    // Print the result
    println!("{}", count);
}
``` 

### Changes Made:
1. **Indexing**: The loop for counting duplicates starts from `1` and checks `i + 1`, which is correct. However, ensure that the input is valid and that the array is accessed correctly.
2. **Error Handling**: The panic message for invalid integer input is retained, but ensure that the input format is strictly followed.
3. **Input Validation**: The program assumes that the input is well-formed as per the specification. If the input is malformed, it will panic. 

Make sure to test with valid input to avoid runtime errors.
