use std::cmp::Ordering;

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32,
}

impl Item {
    // Calculate the value-to-weight ratio, returning None for items with zero weight
    fn value_per_weight(&self) -> Option<f64> {
        if self.weight == 0 {
            None
        } else {
            Some(self.value as f64 / self.weight as f64)
        }
    }
}

fn fractional_knapsack(num_items: usize, max_capacity: f64, items: Vec<Item>) -> f64 {
    // If there are no items or the capacity is zero, return 0.0
    if num_items == 0 || max_capacity <= 0.0 {
        return 0.0;
    }

    // Filter out invalid items and sort by value-to-weight ratio in descending order
    let mut valid_items: Vec<Item> = items
        .into_iter()
        .filter(|item| item.value > 0 || item.weight < 0) // Keep items with negative weights or positive values
        .collect();

    valid_items.sort_by(|a, b| {
        let ratio_a = a.value_per_weight().unwrap_or(0.0);
        let ratio_b = b.value_per_weight().unwrap_or(0.0);
        ratio_b.partial_cmp(&ratio_a).unwrap_or(Ordering::Equal) // Sort in descending order
    });

    let mut total_value = 0.0;
    let mut remaining_capacity = max_capacity;

    for item in valid_items {
        if remaining_capacity <= 0.0 {
            break; // Stop if the knapsack is full
        }

        if item.weight < 0 {
            // Negative weight increases capacity
            remaining_capacity -= item.weight as f64; // Increase capacity
            continue; // Skip to the next item
        }

        if item.weight as f64 <= remaining_capacity {
            // If the whole item can be added
            total_value += item.value as f64;
            remaining_capacity -= item.weight as f64;
        } else {
            // Only a fraction of the item can be added
            let fraction = remaining_capacity / item.weight as f64;
            total_value += item.value as f64 * fraction;
            remaining_capacity = 0.0; // Knapsack is now full
        }
    }

    total_value
}

fn main() {
    // Example usage
    let items = vec![
        Item { weight: 10, value: 60 },
        Item { weight: 20, value: 100 },
        Item { weight: 30, value: 120 },
        Item { weight: 40, value: 240 },
    ];
    let max_capacity = 50.0;
    let max_value = fractional_knapsack(items.len(), max_capacity, items);
    println!("{:.6}", max_value); // Print with six decimal places
}
