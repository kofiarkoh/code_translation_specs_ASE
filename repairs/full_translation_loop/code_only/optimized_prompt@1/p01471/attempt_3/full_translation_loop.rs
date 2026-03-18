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
    let handle = stdin.lock();
    let mut input = String::new();
    handle.read_line(&mut input).unwrap();
    input.trim().parse().unwrap()
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
    let n: i32 = read_int();
    let mut w: f64 = read_int() as f64;
    let mut items: Vec<Item> = Vec::new();
    let mut ans: f64 = 0.0;

    for _ in 0..n {
        let weight: i32 = read_int();
        let value: i32 = read_int();
        
        if weight >= 0 && value <= 0 {
            continue;
        }
        if weight < 0 && value > 0 {
            ans += value as f64;
            w -= weight as f64;
        } else {
            let (mut w_item, mut v_item) = (weight, value);
            if w_item < 0 {
                w_item = -w_item;
                v_item = -v_item;
                ans -= v_item as f64;
                w += w_item as f64; // Corrected to add to `w` instead of `w_item`
            }
            items.push(Item {
                w: w_item,
                v: v_item,
                r: v_item as f64 / w_item as f64,
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
