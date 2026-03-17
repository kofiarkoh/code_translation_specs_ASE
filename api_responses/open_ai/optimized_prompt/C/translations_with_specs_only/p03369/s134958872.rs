fn main() {
    // Read a string of exactly three characters from standard input
    let input = read_input();

    // Count occurrences of 'o' in the input string
    let count_o = input.chars().filter(|&c| c == 'o').count();

    // Calculate the score based on the count of 'o'
    let score = 700 + (100 * count_o) as i32;

    // Output the calculated score
    println!("{}", score);
}

// Function to read a string of exactly three characters
fn read_input() -> String {
    use std::io::{self, Write};

    // Prompt the user for input
    print!("Enter a string of exactly three characters: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim whitespace and ensure the input is exactly three characters
    let trimmed_input = input.trim();
    if trimmed_input.len() != 3 {
        panic!("Input must consist of exactly three characters.");
    }

    trimmed_input.to_string()
}
