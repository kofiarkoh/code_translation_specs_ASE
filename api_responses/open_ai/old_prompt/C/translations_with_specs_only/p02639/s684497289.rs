fn main() {
    // Read five integers from standard input
    let mut inputs = String::new();
    std::io::stdin().read_line(&mut inputs).expect("Failed to read line");

    // Split the input string into parts and parse them as integers
    let numbers: Vec<i32> = inputs
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Check if we have exactly five integers
    if numbers.len() != 5 {
        // Undefined behavior as per specification; we do nothing
        return;
    }

    // Check each number for zero and print the corresponding index (1-5)
    for (index, &number) in numbers.iter().enumerate() {
        if number == 0 {
            // Print the index + 1 (to convert from 0-based to 1-based)
            println!("{}", index + 1);
            return; // Exit after the first zero is found
        }
    }

    // If no zeros are found, we do not output anything
}
