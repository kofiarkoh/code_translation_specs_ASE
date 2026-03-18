use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read three integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    // Parse the integers
    let a: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let b: i32 = iter.next().unwrap().parse().expect("Invalid input");
    let c: i32 = iter.next().unwrap().parse().expect("Invalid input");

    // Check the precondition: a must be non-zero
    if a != 0 {
        // Perform integer division
        let division_result = b / a;

        // Output the smaller of the division result and c
        let result = if division_result <= c { division_result } else { c };
        println!("{}", result);
    }
    // If a is zero, do not produce any output as per the specification
}
