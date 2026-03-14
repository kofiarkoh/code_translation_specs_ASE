use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    let mut total_price: i64 = 0; // Using i64 to accommodate large sums
    let mut total_quantity: i64 = 0; // Using i64 for total quantity
    let mut count: i64 = 0; // Count of processed pairs

    for line in reader.lines() {
        // Read each line from standard input
        if let Ok(line) = line {
            // Split the line by comma and parse the values
            let parts: Vec<&str> = line.split(',').collect();
            if parts.len() == 2 {
                // Attempt to parse price and quantity as integers
                if let (Ok(price), Ok(quantity)) = (parts[0].trim().parse::<i64>(), parts[1].trim().parse::<i64>()) {
                    total_price += price * quantity; // Cumulative price calculation
                    total_quantity += quantity; // Cumulative quantity calculation
                    count += 1; // Increment count of processed pairs
                }
                // If parsing fails, the behavior is undefined as per the specification
            }
        }
    }

    // Calculate average quantity, ensuring we do not divide by zero
    let average_quantity = if count > 0 {
        total_quantity as f64 / count as f64
    } else {
        0.0 // This case should not occur due to precondition
    };

    // Output the results
    println!("Total Price: {}", total_price);
    println!("Average Quantity: {:.0}", average_quantity); // Format to zero decimal places
}
