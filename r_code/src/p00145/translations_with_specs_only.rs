fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of pairs
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for number of pairs");

    // Ensure the number of pairs is positive and within the limit
    assert!(n > 0 && n <= 100, "Number of pairs must be between 1 and 100");

    let mut pairs = Vec::with_capacity(n);

    // Read the pairs of integers
    for _ in 0..n {
        if let Some(Ok(line)) = lines.next() {
            let nums: Vec<i32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();
            if nums.len() == 2 {
                pairs.push((nums[0], nums[1]));
            } else {
                panic!("Each pair must consist of exactly two integers");
            }
        }
    }

    // Compute the aggregate value based on the products of the pairs
    let aggregate_value = pairs.iter()
        .map(|&(a, b)| a * b) // Calculate the product of each pair
        .sum::<i32>(); // Sum the products

    // Output the computed aggregate value
    println!("{}", aggregate_value);
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input and expects the first line to be the number of pairs. It checks that this number is positive and does not exceed 100, adhering to the preconditions and invariants.
2. **Pair Processing**: Each subsequent line is expected to contain exactly two integers. If not, the program panics, reflecting the undefined behavior specified in the case of invalid input.
3. **Aggregate Calculation**: The aggregate value is computed as the sum of the products of the pairs, which aligns with the functional description.
4. **Safety and Idiomatic Rust**: The code avoids `unsafe` blocks, using safe constructs to handle input and computation. The use of `unwrap` and `expect` is justified as they provide clear error messages for invalid input, which is in line with the specification's error handling.
