use std::io::{self, BufRead};

fn count_squares(n: i32, c: i32, low: i32) -> i32 {
    // Base case: if we have used 4 squares, check if we have reached the target
    if c == 4 {
        return if n == 0 { 1 } else { 0 };
    }
    // Initialize the count of combinations
    let mut ret = 0;
    // Start from the current low value and explore further
    for i in low..=(n as f64).sqrt() as i32 {
        // Recursive call, reducing n by the square of i and increasing the count of squares used
        ret += count_squares(n - i * i, c + 1, i);
    }
    ret
}

fn main() {
    // Read input from standard input until a zero is encountered
    let stdin = io::stdin();
    let handle = stdin.lock();
    for line in handle.lines() {
        let input = line.unwrap().trim().parse::<i32>().unwrap();
        if input == 0 {
            break; // Terminate on zero input
        }
        // Ensure input is a positive integer
        if input > 0 {
            let result = count_squares(input, 0, 1);
            println!("{}", result); // Print the result for each valid input
        }
    }
}
