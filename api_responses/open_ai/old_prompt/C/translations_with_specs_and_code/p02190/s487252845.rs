use std::io::{self, BufRead};
use std::collections::HashSet;

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    // Initialize a vector to hold the integers
    let mut a: Vec<i32> = Vec::with_capacity(n);

    // Read the integers into the vector
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        a.push(value);
    }

    // Sort the vector
    a.sort();

    // Count unique integers
    let mut unique_count = 0;
    let mut prev = None;

    for &value in &a {
        if Some(value) != prev {
            unique_count += 1;
            prev = Some(value);
        }
    }

    // Print the count of unique integers
    println!("{}", unique_count);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `std::io` for reading input, which is idiomatic in Rust. The input is read line by line, and parsed into integers.
2. **Vector Initialization**: Used `Vec<i32>` to store integers, which is dynamically sized and safe.
3. **Sorting**: Used the built-in `sort()` method for sorting the vector, which is safe and efficient.
4. **Counting Unique Integers**: Utilized an `Option<i32>` to track the previous integer, ensuring that we only count unique values.
5. **Output**: Used `println!` to print the result, ensuring the output format matches the specification.
6. **Error Handling**: The code assumes valid input as per the specification. If invalid input is provided, it will panic, which aligns with the undefined behavior noted in the specification.
