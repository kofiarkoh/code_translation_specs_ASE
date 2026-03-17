fn main() {
    // Read the number of iterations from user input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the first input as the number of iterations
    let iterations: usize = input.trim().parse().expect("Invalid input for iterations");

    // Initialize a flag to determine if the condition is possible
    let mut possible = true;

    // Iterate over the number of pairs
    for _ in 0..iterations {
        // Read the next line for the pair of integers
        let mut pair_input = String::new();
        std::io::stdin().read_line(&mut pair_input).expect("Failed to read line");

        // Split the input into two parts and parse them as integers
        let parts: Vec<&str> = pair_input.trim().split_whitespace().collect();
        if parts.len() != 2 {
            // If the input format is incorrect, we do not define behavior
            // Just continue to the next iteration
            continue;
        }

        // Attempt to parse the two integers
        let a: i32 = match parts[0].parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip this pair
        };
        let b: i32 = match parts[1].parse() {
            Ok(num) => num,
            Err(_) => continue, // If parsing fails, skip this pair
        };

        // Evaluate the condition based on the integers
        // Here we assume a simple condition for demonstration purposes
        // The actual condition logic should be defined based on the specification
        if a + b < 0 { // Example condition
            possible = false;
        }
    }

    // Output the result based on the evaluation
    if possible {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
