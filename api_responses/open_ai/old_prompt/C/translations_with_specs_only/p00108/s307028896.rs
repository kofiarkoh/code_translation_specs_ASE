fn main() {
    // Read the input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the first integer to get the number of integers in the sequence
    let n: usize = input.trim().parse().expect("Invalid input");
    
    // Check the valid range for n
    if n < 1 || n > 13 {
        return; // Exit if n is out of bounds
    }

    // Read the sequence of integers
    let mut numline: Vec<usize> = Vec::new();
    input.clear();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    numline.extend(input.trim().split_whitespace().map(|s| s.parse::<usize>().expect("Invalid input")));

    // Check that we have exactly n integers
    if numline.len() != n {
        return; // Exit if the number of integers does not match n
    }

    // Ensure all integers are within the valid range
    for &num in &numline {
        if num > 100 {
            return; // Exit if any number is out of bounds
        }
    }

    let mut iterations = 0;
    loop {
        // Create a temporary vector to hold the counts
        let mut counts = vec![0; 101]; // Since numbers are between 0 and 100
        for &num in &numline {
            counts[num] += 1; // Count occurrences of each number
        }

        // Create a new vector for the transformed sequence
        let new_numline: Vec<usize> = counts.iter().map(|&count| count).collect();

        // Check if the new sequence is the same as the old one
        if new_numline == numline {
            break; // Exit the loop if no changes
        }

        // Update numline with the new sequence
        numline = new_numline;
        iterations += 1; // Increment the iteration count
    }

    // Output the number of iterations and the final transformed sequence
    println!("{}", iterations);
    println!("{}", numline.iter().map(usize::to_string).collect::<Vec<String>>().join(" "));
}
