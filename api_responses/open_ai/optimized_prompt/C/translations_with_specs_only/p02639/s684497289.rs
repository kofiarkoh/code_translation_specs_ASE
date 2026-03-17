fn main() {
    // Create a vector to hold the five integers
    let mut inputs = Vec::new();

    // Read five integers from standard input
    for _ in 0..5 {
        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read line");
        // Parse the input as an integer and push it to the vector
        if let Ok(num) = input.trim().parse::<i32>() {
            inputs.push(num);
        }
    }

    // Check for the first zero and output its position
    for (index, &value) in inputs.iter().enumerate() {
        if value == 0 {
            // Output the position (1-based index)
            println!("{}", index + 1);
            return; // Exit after finding the first zero
        }
    }
    // If no zero is found, no output is produced as per the specification
}
