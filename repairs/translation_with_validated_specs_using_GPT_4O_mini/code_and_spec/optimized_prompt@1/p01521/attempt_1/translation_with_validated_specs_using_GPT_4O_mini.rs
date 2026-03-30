use std::io::{self, Write};

fn main() {
    // Read input from stdin
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Trim the input to remove any trailing newline characters
    let input = input.trim();

    // Initialize counts for 'o' and 'x'
    let mut count_o = 0;
    let mut count_x = 0;

    // Iterate over the characters in the input string
    for (i, c) in input.chars().enumerate() {
        // Count occurrences of 'o' and 'x', ensuring we only count unique consecutive characters
        if (i == 0 || c != input.chars().nth(i - 1).unwrap()) {
            match c {
                'o' => count_o += 1,
                'x' => count_x += 1,
                _ => {}
            }
        }
    }

    // Determine the output character based on the counts
    let output = if count_x > count_o {
        'x'
    } else {
        'o'
    };

    // Print the output character
    print!("{}", output);
    io::stdout().flush().expect("Failed to flush stdout");
}
