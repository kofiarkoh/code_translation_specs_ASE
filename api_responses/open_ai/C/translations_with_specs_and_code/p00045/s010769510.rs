use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut total_price: i64 = 0; // Using i64 to match long long int
    let mut total_quantity: i32 = 0; // Using i32 for quantity
    let mut count: i32 = 0; // Count of pairs processed

    for line in reader.lines() {
        // Read each line from standard input
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();

        // Ensure we have exactly two parts
        if parts.len() != 2 {
            continue; // Skip invalid input
        }

        // Parse price and quantity
        if let (Ok(price), Ok(quantity)) = (parts[0].parse::<i32>(), parts[1].parse::<i32>()) {
            total_price += (price as i64) * (quantity as i64);
            total_quantity += quantity;
            count += 1;
        }
    }

    // Output the results
    if count > 0 {
        println!("{}\n{:.0}", total_price, total_quantity as f64 / count as f64);
    }
}
