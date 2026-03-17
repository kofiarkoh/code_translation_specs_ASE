fn main() {
    // Read the number of characters to evaluate
    let num_chars: usize = match std::env::args().nth(1) {
        Some(arg) => arg.parse().unwrap_or_else(|_| {
            eprintln!("Error: Invalid integer input.");
            std::process::exit(1);
        }),
        None => {
            eprintln!("Error: No input provided.");
            std::process::exit(1);
        }
    };

    // Read the string of characters
    let input_string: String = match std::env::args().nth(2) {
        Some(arg) => arg,
        None => {
            eprintln!("Error: No string input provided.");
            std::process::exit(1);
        }
    };

    // Check preconditions
    if num_chars > input_string.len() || num_chars > 127 {
        eprintln!("Error: Invalid input size.");
        std::process::exit(1);
    }

    // Evaluate the characters
    let evaluated_chars = &input_string[..num_chars];
    let count_r = evaluated_chars.chars().filter(|&c| c == 'R').count();
    let count_other = evaluated_chars.len() - count_r;

    // Determine the output based on the counts
    let output = if count_r > count_other {
        "Yes"
    } else {
        "No"
    };

    // Print the result
    println!("{}", output);
}
