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

    // Read N and W
    let first_line = lines.next().unwrap().unwrap();
    let mut parts = first_line.split_whitespace();
    let n: usize = parts.next().unwrap().parse().unwrap();
    let mut w_capacity: f64 = parts.next().unwrap().parse().unwrap();

    let mut items: Vec<Item> = Vec::new();
    let mut total_value = 0.0;

    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let mut parts = line.split_whitespace();
        let weight: i32 = parts.next().unwrap().parse().unwrap();
        let value: i32 = parts.next().unwrap().parse().unwrap();

        if weight >= 0 && value <= 0 {
            continue; // Skip items with non-positive value and non-negative weight
        }
        if weight < 0 && value > 0 {
            total_value += value as f64;
            w_capacity -= weight as f64; // weight is negative, so we add
        } else {
            let (w, v) = if weight < 0 {
                (-weight, -value) // Convert to positive weight and negative value
            } else {
                (weight, value)
            };
            items.push(Item {
                weight: w,
                value: v,
                ratio: v as f64 / w as f64,
            });
        }
    }

    // Sort items by value-to-weight ratio in descending order
    items.sort_by(|a, b| b.ratio.partial_cmp(&a.ratio).unwrap());

    for item in items {
        if w_capacity <= 0.0 {
            break; // No more capacity
        }
        let fraction = (w_capacity / item.weight as f64).min(1.0);
        w_capacity -= fraction * item.weight as f64;
        total_value += fraction * item.value as f64;
    }

    // Print the result
    println!("{:.6}", total_value);
}
