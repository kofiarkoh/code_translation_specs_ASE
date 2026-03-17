fn main() {
    // Read three integers from the user
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");

    // Split the input string into parts and parse them as integers
    let mut numbers: Vec<i32> = input
        .split_whitespace()
        .take(3) // Ensure we only take exactly three integers
        .filter_map(|s| s.parse().ok()) // Parse and filter out any invalid integers
        .collect();

    // Check if we have exactly three integers
    if numbers.len() != 3 {
        panic!("Expected exactly three integers"); // Panic if the precondition is violated
    }

    // Sort the numbers in non-decreasing order
    numbers.sort();

    // Print the sorted numbers, separated by spaces
    println!("{}", numbers.iter().map(|n| n.to_string()).collect::<Vec<String>>().join(" "));
}
