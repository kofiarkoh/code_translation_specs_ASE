use std::io::{self, BufRead};
use std::f64;

#[derive(Debug)]
struct Item {
    w: i32,
    v: i32,
    r: f64,
}

fn read_int() -> i32 {
    let stdin = io::stdin();
    let mut input = String::new();
    stdin.lock().read_line(&mut input).unwrap();
    let trimmed = input.trim();
    trimmed.parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse integer from input: {}", trimmed);
        std::process::exit(1);
    })
}

fn zero(a: f64) -> bool {
    a.abs() < 1e-7
}

fn cmp(a: &Item, b: &Item) -> std::cmp::Ordering {
    if (b.r - a.r).abs() < 1e-8 {
        std::cmp::Ordering::Equal
    } else if a.r <= b.r {
        std::cmp::Ordering::Greater
    } else {
        std::cmp::Ordering::Less
    }
}

fn main() {
    let n = read_int();
    let mut w = read_int() as f64;
    let mut items: Vec<Item> = Vec::new();
    let mut ans = 0.0;

    for _ in 0..n {
        let weight = read_int();
        let value = read_int();
        if weight >= 0 && value <= 0 {
            continue;
        }
        if weight < 0 && value > 0 {
            ans += value as f64;
            w -= weight as f64;
        } else {
            let (mut weight, mut value) = (weight, value);
            if weight < 0 {
                weight = -weight;
                value = -value;
                ans -= value as f64;
                w += weight as f64; // Corrected to use weight instead of w
            }
            items.push(Item {
                w: weight,
                v: value,
                r: value as f64 / weight as f64,
            });
        }
    }

    items.sort_by(|a, b| cmp(b, a));

    for item in items.iter() {
        if zero(w) {
            break;
        }
        let mut x = w / item.w as f64;
        if x > 1.0 {
            x = 1.0;
        }
        w -= x * item.w as f64;
        ans += x * item.v as f64;
    }

    println!("{:.6}", ans);
}
