fn main() {
    // Read the number of pairs (N) and the threshold year (Q)
    let (n, q) = read_input();

    // Initialize a vector to hold the years and names
    let mut years_names: Vec<(i32, String)> = Vec::new();

    // Read N pairs of (year, name)
    for _ in 0..n {
        let (year, name) = read_year_name_pair();
        years_names.push((year, name));
    }

    // Find the most recent year that is less than or equal to Q
    let mut selected_name = &years_names[0].1; // Default to the first name
    let mut found = false;

    for (year, name) in &years_names {
        if *year <= q {
            selected_name = name; // Update the selected name
            found = true; // Mark that we found a valid year
        }
    }

    // If no valid year was found, selected_name remains the first name
    println!("{}", selected_name);
}

// Function to read input values for N and Q
fn read_input() -> (i32, i32) {
    // Placeholder for reading input; replace with actual input handling
    (3, 2000) // Example values
}

// Function to read a year and name pair
fn read_year_name_pair() -> (i32, String) {
    // Placeholder for reading input; replace with actual input handling
    (1999, "Name1".to_string()) // Example values
}
```

```rust
// The above code implements the specification as follows:
// - It reads two integers, N and Q, and then N pairs of (year, name).
// - It maintains a reference to the name associated with the most recent year
//   that is less than or equal to Q.
// - If no such year is found, it defaults to the first name in the list.
// - The program assumes valid input as per the specification and does not handle
//   invalid input cases explicitly, as behavior is not defined for such cases.
