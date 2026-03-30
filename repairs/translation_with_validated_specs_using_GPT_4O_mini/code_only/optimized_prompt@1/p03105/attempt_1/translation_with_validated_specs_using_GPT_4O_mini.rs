use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    // Read input from stdin
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input into parts and parse them as integers
    let mut iter = input.split_whitespace();
    let a: i32 = iter.next().unwrap().parse().expect("Failed to parse a");
    let b: i32 = iter.next().unwrap().parse().expect("Failed to parse b");
    let c: i32 = iter.next().unwrap().parse().expect("Failed to parse c");

    // Calculate b / a, using integer division
    let division_result = b / a;

    // Check the condition and print the appropriate result
    if division_result <= c {
        println!("{}", division_result);
    } else {
        println!("{}", c);
    }
}
