fn main() {
    use std::io::{self, Write};

    // Read three integers from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"));
    
    // Collect the integers, although N is not used in the decision-making process
    let _n = numbers.next().unwrap(); // Read N but do not use it
    let a = numbers.next().unwrap(); // Read A
    let b = numbers.next().unwrap(); // Read B

    // Determine the output based on the parity of the difference between A and B
    if (b - a) % 2 == 0 {
        print!("Alice\n");
    } else {
        print!("Borys\n");
    }

    // Flush the output to ensure it is printed immediately
    io::stdout().flush().expect("Failed to flush stdout");
}
