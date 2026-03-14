use std::collections::HashSet;
use std::io::{self, BufRead};

fn main() {
    // Read exactly ten integers from standard input
    let stdin = io::stdin();
    let input: Vec<i32> = stdin.lock().lines()
        .take(10) // Ensure we only read 10 lines
        .filter_map(|line| line.ok()?.trim().parse().ok()) // Parse integers safely
        .collect();

    // Ensure we have exactly 10 integers
    if input.len() != 10 {
        eprintln!("Error: Expected exactly 10 integers.");
        return;
    }

    // Use a HashSet to collect unique values
    let mut unique_values: HashSet<i32> = HashSet::new();
    for &value in &input {
        unique_values.insert(value);
    }

    // Convert the HashSet to a Vec and sort it in descending order
    let mut sorted_unique: Vec<i32> = unique_values.into_iter().collect();
    sorted_unique.sort_unstable_by(|a, b| b.cmp(a)); // Sort in descending order

    // Prepare the output array for the top three unique values
    let mut top_three = [0; 3]; // Initialize with zeros
    for (i, &value) in sorted_unique.iter().enumerate().take(3) {
        top_three[i] = value; // Fill the top three with available unique values
    }

    // Output the top three values
    for &value in &top_three {
        println!("{}", value);
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads exactly ten integers and checks for this condition. If fewer than ten integers are provided, it outputs an error message.
2. **Unique Values**: A `HashSet` is used to collect unique integers, ensuring that duplicates are automatically filtered out.
3. **Sorting**: The unique values are sorted in descending order to easily access the top three values.
4. **Output**: The output array is initialized with zeros, and only the available unique values are filled in. This handles the case where fewer than three unique integers are present.
5. **Idiomatic Rust**: The code avoids `unsafe` blocks and uses safe constructs, adhering to Rust's safety guarantees.
