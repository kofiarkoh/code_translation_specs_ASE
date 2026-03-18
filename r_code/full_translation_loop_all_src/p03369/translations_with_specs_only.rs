fn main() {
    // Read a string of exactly three characters from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Check the precondition: the input must consist of exactly three characters
    if input.len() != 3 {
        // Undefined behavior as per the specification; we will panic to indicate this
        panic!("Input must consist of exactly three characters");
    }

    // Count occurrences of the character 'o'
    let count_o = input.chars().filter(|&c| c == 'o').count();

    // Calculate the output value based on the count of 'o' characters
    let output_value = 700 + 100 * count_o;

    // Output the calculated value
    println!("{}", output_value);
}
