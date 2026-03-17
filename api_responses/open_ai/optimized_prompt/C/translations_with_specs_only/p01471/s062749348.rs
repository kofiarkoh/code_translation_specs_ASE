use std::cmp::Ordering;

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32,
}

impl Item {
    // Calculate the value-to-weight ratio
    fn ratio(&self) -> f64 {
        if self.weight > 0 {
            self.value as f64 / self.weight as f64
        } else {
            // If weight is non-positive, return a very low ratio to ignore in sorting
            f64::NEG_INFINITY
        }
    }
}

fn fractional_knapsack(num_items: usize, capacity: f64, items: Vec<Item>) -> f64 {
    // Filter out items with non-positive values and positive weights
    let mut filtered_items: Vec<Item> = items
        .into_iter()
        .filter(|item| item.value > 0 || item.weight < 0)
        .collect();

    // Sort items by value-to-weight ratio in descending order
    filtered_items.sort_by(|a, b| b.ratio().partial_cmp(&a.ratio()).unwrap_or(Ordering::Equal));

    let mut total_value = 0.0;
    let mut remaining_capacity = capacity;

    for item in filtered_items {
        if remaining_capacity <= 0.0 {
            break; // No more capacity left
        }

        if item.weight < 0 {
            // If the item has negative weight, it adds value and reduces capacity
            total_value += item.value as f64;
            remaining_capacity += item.weight as f64; // Reduce capacity by negative weight
        } else if item.weight > 0 {
            // If the item has positive weight
            if remaining_capacity >= item.weight as f64 {
                // Take the whole item
                total_value += item.value as f64;
                remaining_capacity -= item.weight as f64;
            } else {
                // Take the fractional part of the item
                let fraction = remaining_capacity / item.weight as f64;
                total_value += fraction * item.value as f64;
                remaining_capacity = 0.0; // Knapsack is full
            }
        }
    }

    total_value
}

fn main() {
    // Example usage
    let num_items = 5; // Number of items
    let capacity = 50.0; // Total weight capacity of the knapsack
    let items = vec![
        Item { weight: 10, value: 60 },
        Item { weight: 20, value: 100 },
        Item { weight: 30, value: 120 },
        Item { weight: -5, value: 30 }, // Negative weight item
        Item { weight: 15, value: 0 },  // Zero value item
    ];

    let max_value = fractional_knapsack(num_items, capacity, items);
    println!("{:.2}", max_value); // Output the maximum value with two decimal precision
}
