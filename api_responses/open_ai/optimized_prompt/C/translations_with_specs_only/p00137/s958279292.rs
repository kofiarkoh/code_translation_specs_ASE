fn main() {
    // Read the number of cases from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the number of cases; it must be a positive integer
    let num_cases: usize = input.trim().parse().expect("Input must be a positive integer");

    // Ensure we have a valid number of cases
    if num_cases == 0 {
        return; // No cases to process
    }

    // Process each case
    for case_number in 1..=num_cases {
        // Read the input value for the current case
        let mut input_value = String::new();
        std::io::stdin().read_line(&mut input_value).expect("Failed to read line");
        
        // Parse the input value; it must be a valid integer
        let base_value: i32 = input_value.trim().parse().expect("Input must be a valid integer");

        // Output the case header
        println!("Case {}:", case_number);

        // Compute and output ten derived values from the input value
        for i in 0..10 {
            // The transformation is not specified, so we will use a simple example
            // Here we use a simple transformation: base_value + i
            // This can be adjusted based on the actual transformation needed
            let computed_value = base_value + i; // Example transformation
            println!("{}", computed_value);
        }
    }
}
