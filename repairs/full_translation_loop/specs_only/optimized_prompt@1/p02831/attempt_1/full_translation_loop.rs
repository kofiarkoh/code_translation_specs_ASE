use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first input (non-negative integer)
    let first_input: i64 = lines.next().unwrap().unwrap().parse().unwrap();
    // Read the second input (positive integer)
    let second_input: i64 = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions check
    if first_input < 0 || second_input <= 0 {
        return; // Exit if preconditions are not met
    }

    // Compute the smallest multiple of first_input that is also a multiple of second_input
    let mut multiple = first_input;
    while multiple % second_input != 0 {
        multiple += first_input; // Increment by first_input to find the next multiple
        // Postcondition: If we exceed a reasonable limit, we can terminate without output
        // This is a safeguard against infinite loops in case of unexpected input
        if multiple > i64::MAX {
            return; // Terminate without output if no valid multiple is found
        }
    }

    // Output the result followed by a newline
    println!("{}", multiple);
}
