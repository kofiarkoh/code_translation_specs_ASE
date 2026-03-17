use std::cmp::Ordering;
use std::io::{self, BufRead};

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
            continue; // Ignore items with non-positive values and positive weights
        }
        if weight < 0 && value > 0 {
            total_value += value as f64;
            total_weight_capacity -= weight as f64; // Reduce capacity by negative weight
        } else {
            let (w, v) = if weight < 0 {
                (-weight, -value) // Treat negative weight and value as positive
            } else {
                (weight, value)
            };
            let ratio = v as f64 / w as f64;
            items.push(Item { weight: w, value: v, ratio });
        }
    }

    // Sort items by ratio in descending order
    items.sort_by(|a, b| {
        if (b.ratio - a.ratio).abs() < 1e-8 {
            Ordering::Equal
        } else if a.ratio <= b.ratio {
            Ordering::Greater
        } else {
            Ordering::Less
        }
    });

    for item in items.iter() {
        if total_weight_capacity <= 0.0 {
            break; // Stop if no capacity left
        }
        let fraction = total_weight_capacity / item.weight as f64;
        let fraction = if fraction > 1.0 { 1.0 } else { fraction };
        total_weight_capacity -= fraction * item.weight as f64;
        total_value += fraction * item.value as f64;
    }

    // Print the result with precision
    println!("{:.6}", total_value);
}
