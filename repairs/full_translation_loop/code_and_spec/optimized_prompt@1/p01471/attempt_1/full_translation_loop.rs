use std::io::{self, BufRead};
use std::f64;

#[derive(Debug)]
struct Item {
    weight: i32,
    value: i32,
    ratio: f64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut total_weight_capacity: f64 = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut items: Vec<Item> = Vec::new();
    let mut total_value: f64 = 0.0;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();

        if weight >= 0 && value <= 0 {
            continue; // Ignore items with non-positive values and non-negative weights
        }

        if weight < 0 && value > 0 {
            total_value += value as f64;
            total_weight_capacity -= weight as f64; // weight is negative, so we add
        } else {
            let (w, v) = if weight < 0 {
                (-weight, -value) // Convert to positive weight and negative value
            } else {
                (weight, value)
            };
            let ratio = v as f64 / w as f64;
            items.push(Item { weight: w, value: v, ratio });
        }
    }

    // Sort items by value-to-weight ratio in descending order
    items.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

    for item in items.iter() {
        if total_weight_capacity <= 0.0 {
            break; // No more capacity in the knapsack
        }
        let fraction = total_weight_capacity / item.weight as f64;
        let fraction = fraction.min(1.0); // Take the minimum of 1.0 and the calculated fraction
        total_weight_capacity -= fraction * item.weight as f64;
        total_value += fraction * item.value as f64;
    }

    println!("{:.6}", total_value); // Print the result with six decimal places
}
