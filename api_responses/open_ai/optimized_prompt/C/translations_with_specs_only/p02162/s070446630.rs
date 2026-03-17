fn main() {
    // Read four integer values from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let scores: Vec<i32> = input
        .split_whitespace()
        .filter_map(|s| s.parse().ok())
        .collect();

    // Check if we have exactly four integer values
    if scores.len() != 4 {
        // Terminate without producing output if not enough values
        return;
    }

    // Unpack the scores into respective variables
    let time_a = scores[0];
    let time_b = scores[1];
    let result_a = scores[2];
    let result_b = scores[3];

    // Determine the winner based on the specified logic
    let winner = if time_a < time_b {
        "Alice"
    } else if time_a > time_b {
        "Bob"
    } else {
        // If times are equal, compare results
        if result_a < 0 || result_b < 0 {
            // If results are negative, ignore them as per the specification
            "Draw"
        } else if result_a > result_b {
            "Alice"
        } else if result_a < result_b {
            "Bob"
        } else {
            "Draw"
        }
    };

    // Output the winner
    println!("{}", winner);
}
