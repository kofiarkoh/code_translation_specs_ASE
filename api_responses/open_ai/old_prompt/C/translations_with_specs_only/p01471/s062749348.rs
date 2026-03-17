// Code implementation for the Fractional Knapsack Problem

use std::cmp::Ordering;

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32,
}

// Function to calculate the maximum value for the fractional knapsack problem
fn fractional_knapsack(n: usize, w: f64, items: &mut Vec<Item>) -> f64 {
    // Sort items by value-to-weight ratio in descending order
    items.sort_by(|a, b| {
        let ratio_a = a.value as f64 / a.weight as f64;
        let ratio_b = b.value as f64 / b.weight as f64;
        ratio_b.partial_cmp(&ratio_a).unwrap_or(Ordering::Equal)
    });

    let mut total_value = 0.0;
    let mut remaining_weight = w;

    for item in items {
        if remaining_weight <= 0.0 {
            break; // No more weight capacity
        }

        if item.weight as f64 <= remaining_weight {
            // Take the whole item
            total_value += item.value as f64;
            remaining_weight -= item.weight as f64;
        } else {
            // Take the fraction of the item that fits
            total_value += item.value as f64 * (remaining_weight / item.weight as f64);
            remaining_weight = 0.0; // Knapsack is full
        }
    }

    total_value
}

fn main() {
    // Example input
    let n: usize = 3; // Number of items
    let w: f64 = 50.0; // Maximum weight capacity
    let mut items = vec![
        Item { weight: 10, value: 60 },
        Item { weight: 20, value: 100 },
        Item { weight: 30, value: 120 },
    ];

    // Calculate the maximum value
    let max_value = fractional_knapsack(n, w, &mut items);

    // Output the result
    println!("{:.2}", max_value); // Print the maximum value formatted to two decimal places
}
