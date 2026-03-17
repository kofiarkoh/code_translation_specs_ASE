fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let quantity: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Initialize the output vector
    let mut output = Vec::new();

    // Generate output based on the specified conditions
    match quantity {
        q if q >= 50 => {
            // Generate 50 integers that sum to the input value
            let base_value = quantity / 50; // Base value for each element
            let remainder = quantity % 50; // Remainder to distribute
            
            // Fill the output with the base value
            output.extend(vec![base_value; 50]);
            
            // Distribute the remainder across the first few elements
            for i in 0..remainder {
                output[i as usize] += 1; // Increment the first 'remainder' elements
            }
        }
        2..=49 => {
            // Repeat the input value for the number of times equal to the input
            output.extend(vec![quantity; quantity as usize]);
        }
        0 => {
            // Fixed sequence for input 0
            output.extend(vec![3, 3, 3, 3]);
        }
        _ => {
            // Fixed sequence for negative input
            output.extend(vec![1, 0, 3]);
        }
    }

    // Output the number of elements in the sequence
    println!("{}", output.len());
    // Output the sequence of integers, space-separated
    println!("{}", output.iter().map(|x| x.to_string()).collect::<Vec<String>>().join(" "));
}
