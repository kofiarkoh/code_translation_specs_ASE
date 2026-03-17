use std::io;

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as an integer
    let number: i32 = input.trim().parse().expect("Input must be a valid integer");

    // Calculate the output based on the specified rules
    let output = if number % 11 == 0 {
        // If divisible by 11, output is twice the quotient
        2 * (number / 11)
    } else {
        let remainder = number % 11;
        if remainder >= 1 && remainder <= 6 {
            // If remainder is between 1 and 6, add 1 to twice the quotient
            2 * (number / 11) + 1
        } else {
            // If remainder is between 7 and 10, add 2 to twice the quotient
            2 * (number / 11) + 2
        }
    };

    // Output the result
    println!("{}", output);
}
