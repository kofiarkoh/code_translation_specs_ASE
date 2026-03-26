use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32,
}

impl Item {
    // Calculate value-to-weight ratio
    fn value_per_weight(&self) -> f64 {
        if self.weight == 0 {
            return 0.0; // Avoid division by zero; treat as zero value
        }
        self.value as f64 / self.weight as f64
    }
}

fn fractional_knapsack(n: usize, w: f64, items: &mut Vec<Item>) -> f64 {
    // Sort items by value-to-weight ratio in descending order
    items.sort_by(|a, b| {
        b.value_per_weight()
            .partial_cmp(&a.value_per_weight())
            .unwrap_or(Ordering::Equal)
    });

    let mut total_value = 0.0;
    let mut remaining_weight = w;

    for item in items {
        if remaining_weight <= 0.0 {
            break; // No more weight capacity
        }

        if item.weight <= remaining_weight as i32 {
            // Take the whole item
            total_value += item.value as f64;
            remaining_weight -= item.weight as f64;
        } else {
            // Take the fraction of the item
            total_value += item.value_per_weight() * remaining_weight;
            remaining_weight = 0.0; // All weight capacity used
        }
    }

    total_value
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the first line for N and W
    let first_line = reader.lines().next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let w: f64 = parts.next().unwrap().parse().unwrap();

    // Read the items
    let mut items = Vec::with_capacity(n);
    for _ in 0..n {
        let line = reader.lines().next().unwrap().unwrap();
        let mut item_parts = line.split_whitespace();
        let weight: i32 = item_parts.next().unwrap().parse().unwrap();
        let value: i32 = item_parts.next().unwrap().parse().unwrap();
        items.push(Item { weight, value });
    }

    // Calculate the maximum value
    let max_value = fractional_knapsack(n, w, &mut items);

    // Output the result
    println!("{:.6}", max_value); // Format output to six decimal places
}
