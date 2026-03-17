fn determine_player_name(n: i32, a: i32, b: i32) -> &'static str {
    // Calculate the difference between A and B
    let difference = a - b;

    // Check the parity of the difference
    if difference % 2 == 0 {
        // If the difference is even, return "Alice"
        "Alice"
    } else {
        // If the difference is odd, return "Borys"
        "Borys"
    }
}

fn main() {
    // Example usage
    let n = 0; // N is not used in decision-making
    let a = 5;
    let b = 3;

    // Output the name based on the parity of the difference
    let player_name = determine_player_name(n, a, b);
    println!("{}", player_name);
}
