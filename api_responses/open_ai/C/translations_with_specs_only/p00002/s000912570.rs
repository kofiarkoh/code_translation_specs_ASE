use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line from standard input
        let line = line.expect("Failed to read line");
        let mut numbers = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok()); // Parse integers, ignore invalid ones

        // Read pairs of integers
        if let (Some(a), Some(b)) = (numbers.next(), numbers.next()) {
            let sum = a + b; // Compute the sum
            let length = sum.to_string().len(); // Get the length of the string representation of the sum
            println!("{}", length); // Output the length
        }
    }
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input until EOF, as specified. It uses `BufRead` for efficient reading.
2. **Integer Parsing**: The integers are parsed using `filter_map` to safely handle invalid inputs by ignoring them, as the specification states that behavior is undefined for invalid integers.
3. **Sum Calculation**: The sum of the two integers is computed directly, and its string representation length is determined using `to_string().len()`.
4. **Output**: The length of the string representation of the sum is printed immediately after processing each valid pair.
5. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees while fulfilling the specification's requirements.
