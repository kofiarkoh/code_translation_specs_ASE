fn main() {
    // Read input from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Parse the input into three integers
    let numbers: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Failed to parse integer"))
        .collect();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers");
    }

    let a = numbers[1]; // A is the second integer
    let b = numbers[2]; // B is the third integer

    // Calculate the difference
    let difference = b - a;

    // Determine the winner based on the parity of the difference
    if difference % 2 == 0 {
        // If the difference is even, Alice wins
        println!("Alice");
    } else {
        // If the difference is odd, Borys wins
        println!("Borys");
    }

    // The output is flushed automatically when using println!
}
