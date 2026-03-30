fn determine_winner(t1: i32, t2: i32, r1: i32, r2: i32) -> String {
    // Check if result values are negative
    if r1 < 0 || r2 < 0 {
        // Compare time values
        if t1 < t2 {
            "Alice".to_string()
        } else if t1 > t2 {
            "Bob".to_string()
        } else {
            "Draw".to_string()
        }
    } else {
        // Compare result values
        if r1 > r2 {
            "Bob".to_string()
        } else if r1 < r2 {
            "Alice".to_string()
        } else {
            "Draw".to_string()
        }
    }
}

fn main() {
    // Example usage
    let t1 = 5;
    let t2 = 10;
    let r1 = -1;
    let r2 = 2;

    // Call the function and print the result
    let result = determine_winner(t1, t2, r1, r2);
    println!("{}", result);
}
