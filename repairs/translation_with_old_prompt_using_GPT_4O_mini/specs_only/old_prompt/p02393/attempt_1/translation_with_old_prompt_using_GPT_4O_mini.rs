use std::io;

fn main() {
    // Read three integers from user input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Sort the numbers using a simple comparison method
    // This is done without using any sorting algorithms or data structures
    if numbers[0] > numbers[1] {
        numbers.swap(0, 1);
    }
    if numbers[0] > numbers[2] {
        numbers.swap(0, 2);
    }
    if numbers[1] > numbers[2] {
        numbers.swap(1, 2);
    }

    // Print the sorted integers in ascending order
    println!("{} {} {}", numbers[0], numbers[1], numbers[2]);
}
