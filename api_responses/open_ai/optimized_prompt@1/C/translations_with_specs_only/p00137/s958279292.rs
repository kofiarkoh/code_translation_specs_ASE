fn main() {
    use std::io::{self, Write};

    // Read the number of cases from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of cases; it must be a positive integer
    let num_cases: usize = input.trim().parse().expect("Input must be a positive integer");

    // Process each case
    for case_number in 1..=num_cases {
        // Read the input integer for the current case
        let mut case_input = String::new();
        io::stdin().read_line(&mut case_input).expect("Failed to read line");
        
        // Parse the input integer; behavior is undefined for invalid inputs
        let base_value: i32 = case_input.trim().parse().expect("Input must be a valid integer");

        // Output the case header
        println!("Case {}:", case_number);

        // Compute and output ten derived values based on the input integer
        for i in 0..10 {
            // The transformation is assumed to be a simple arithmetic operation
            // Here we use a simple formula: base_value + i
            // This can be adjusted based on the specific transformation needed
            let derived_value = base_value + i; // Example transformation
            println!("{}", derived_value);
        }
    }
}
