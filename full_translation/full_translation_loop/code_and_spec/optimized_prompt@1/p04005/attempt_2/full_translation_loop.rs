use std::io;

fn main() {
    let mut input = String::new();
    // Read three long integers from standard input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i64> = input
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        panic!("Expected exactly three long integer values");
    }

    // Check if any value is even
    if values[0] % 2 == 0 || values[1] % 2 == 0 || values[2] % 2 == 0 {
        println!("0");
    } else {
        // Find the maximum value and its index
        let mut max_index = 0;
        for i in 1..3 {
            if values[i] > values[max_index] {
                max_index = i;
            }
        }

        // Create a mutable copy of the values to modify
        let mut modified_values = values.clone();
        modified_values[max_index] = 1;

        // Calculate the product of the original values
        let product = values.iter().product::<i64>();
        println!("{}", product);
    }
}
