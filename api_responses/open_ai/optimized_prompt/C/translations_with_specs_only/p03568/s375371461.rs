fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the first line which is the count of subsequent integers
    let first_line = lines.next().expect("Expected at least one line of input").unwrap();
    let n: usize = first_line.parse().expect("First input must be a non-negative integer");

    // Initialize the count of even integers
    let mut even_count = 0;

    // Read the subsequent integers
    for line in lines.take(n) {
        let number: i32 = line.unwrap().parse().expect("Expected an integer");
        if number % 2 == 0 {
            even_count += 1; // Increment even count if the number is even
        }
    }

    // Calculate the result based on the specification
    let power_of_three = 3_i32.pow(n as u32); // Calculate 3^N
    let power_of_two = 2_i32.pow(even_count as u32); // Calculate 2^(count of even integers)

    // Output the result of the expression 3^N - 2^(count of even integers)
    let result = power_of_three - power_of_two;
    println!("{}", result);
}
