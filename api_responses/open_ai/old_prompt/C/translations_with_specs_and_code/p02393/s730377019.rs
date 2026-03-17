use std::io;

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the three integers from the input
    let mut nums: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Ensure we have exactly three integers
    if nums.len() != 3 {
        eprintln!("Please provide exactly three integers.");
        return;
    }

    // Sort the numbers in ascending order
    nums.sort_unstable();

    // Print the sorted integers
    println!("{} {} {}", nums[0], nums[1], nums[2]);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code reads a line of input and splits it into whitespace-separated values, parsing them into integers. This aligns with the specification that expects three integers.
2. **Sorting Logic**: Instead of manually comparing the integers as in the C code, Rust's `sort_unstable()` method is used, which is idiomatic and efficient for a fixed number of elements.
3. **Error Handling**: The code checks if exactly three integers are provided and prints an error message if not. This is a safeguard against invalid input, although the specification states that invalid input is not handled.
4. **Output Formatting**: The output is formatted to match the specification, ensuring integers are printed in ascending order, separated by spaces.
5. **Safety**: The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees.
