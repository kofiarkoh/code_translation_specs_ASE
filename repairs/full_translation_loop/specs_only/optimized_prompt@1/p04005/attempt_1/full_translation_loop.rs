use std::io;

fn main() {
    // Read three long integer values from standard input
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let values: Vec<i64> = input
        .trim()
        .split_whitespace()
        .map(|s| s.parse().expect("Invalid input"))
        .collect();

    // Ensure we have exactly three values
    if values.len() != 3 {
        panic!("Expected exactly three long integer values");
    }

    // Check if any value is even
    if values.iter().any(|&x| x % 2 == 0) {
        // If at least one value is even, output "0"
        println!("0");
    } else {
        // If all values are odd, find the maximum value
        let max_value = *values.iter().max().unwrap();
        
        // Calculate the product of the three values
        let product: i64 = values.iter().product();
        
        // Replace the maximum value with 1 in the product calculation
        let adjusted_product = product / max_value; // Remove max_value from product
        let final_product = adjusted_product * 1; // Multiply by 1 (replacing max_value)

        // Output the final product
        println!("{}", final_product);
    }
}
