fn main() {
    // Read three integers from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Split the input string into parts and parse them as integers
    let mut numbers = input.split_whitespace()
        .map(|s| s.parse::<i32>().expect("Failed to parse integer"))
        .collect::<Vec<i32>>();

    // Ensure we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers");
    }

    // Unpack the integers
    let _n = numbers[0]; // N is not used in decision-making
    let a = numbers[1]; // Alice's score
    let b = numbers[2]; // Borys's score

    // Calculate the difference and check its parity
    if (b - a) % 2 == 0 {
        // If the difference is even, Alice wins
        println!("Alice");
    } else {
        // If the difference is odd, Borys wins
        println!("Borys");
    }

    // Return 0 upon successful execution
    std::process::exit(0);
}
