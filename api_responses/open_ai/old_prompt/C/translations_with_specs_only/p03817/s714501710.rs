use std::io::{self, Write};

fn main() {
    // Read input from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");

    // Attempt to parse the input as a long long integer (i64 in Rust)
    let n: i64 = match input.trim().parse() {
        Ok(num) => num,
        Err(_) => {
            // Undefined behavior on invalid input; we could panic or exit
            // Here we choose to panic to reflect the undefined behavior
            panic!("Invalid input; expected a valid long long integer.");
        }
    };

    // Calculate how many times n can be divided by 11
    let mut count = 0;
    let mut value = n;

    // While value is divisible by 11, increment count and divide value by 11
    while value % 11 == 0 {
        count += 1;
        value /= 11;
    }

    // Output the result, which is the count of divisions by 11
    // This is the derived value based on the input n
    println!("{}", count);
}
