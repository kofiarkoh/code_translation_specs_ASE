fn main() {
    // Read inputs A, B, and X from standard input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let inputs: Vec<i32> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Unpack inputs into A, B, and X
    let (a, b, x) = (inputs[0], inputs[1], inputs[2]);

    // Ensure inputs are non-negative integers as per specification
    if a < 0 || b < 0 || x < 0 {
        panic!("Inputs must be non-negative integers");
    }

    // Calculate the cost based on the relationship between A, B, and X
    let cost = if a > b {
        a * x
    } else {
        b * x
    };

    // Print the result to standard output
    println!("{}", cost);
}
