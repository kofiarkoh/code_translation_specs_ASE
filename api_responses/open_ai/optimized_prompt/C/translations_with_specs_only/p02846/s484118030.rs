fn main() {
    // Read six long long integers from input
    let mut input = String::new();
    std::io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly six values
    assert_eq!(values.len(), 6);

    // Unpack the input values into pairs A and B, and time values T
    let (a1, a2) = (values[0], values[1]);
    let (b1, b2) = (values[2], values[3]);
    let (t1, t2) = (values[4], values[5]);

    // Check the condition for "infinity"
    if a1 == b1 && a2 == b2 {
        println!("infinity");
    } else {
        // Calculate the result based on the relationship between A and B
        let result = if a1 > b1 {
            // Modify A and B if A's first element is greater than B's first element
            let a1_modified = a1 - b1;
            let a2_modified = a2 - b2;
            // Return the calculated integer value
            a1_modified + a2_modified
        } else {
            // If A's first element is not greater, return zero
            0
        };

        // Output the calculated integer value
        println!("{}", result);
    }
}
