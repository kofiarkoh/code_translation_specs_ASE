use std::io::{self, BufRead};

#[derive(Debug, Clone)] // Derive Clone to allow cloning of Rectangle
struct Rectangle {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
    used: bool,
}

fn read_input() -> Vec<Rectangle> {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut rectangles = Vec::new();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());
        if let Some(n) = iter.next() {
            for id in 0..n {
                if let (Some(x1), Some(y1), Some(x2), Some(y2)) = (iter.next(), iter.next(), iter.next(), iter.next()) {
                    rectangles.push(Rectangle {
                        id: id as usize,
                        x1: x1.min(x2),
                        y1: y1.min(y2),
                        x2: x1.max(x2),
                        y2: y1.max(y2),
                        used: false,
                    });
                }
            }
        }
    }
    rectangles
}

fn compare_rectangles(a: &Rectangle, b: &Rectangle) -> std::cmp::Ordering {
    if a.x1 != b.x1 {
        a.x1.cmp(&b.x1)
    } else {
        a.y1.cmp(&b.y1)
    }
}

fn calculate_order(rectangles: &mut Vec<Rectangle>, order: &mut Vec<usize>) {
    rectangles.sort_by(compare_rectangles);
    let first = &mut rectangles[0];
    order[first.id] = order.len() + 1; // 1-based index
    first.used = true;

    let y = first.y2; // No need for mutability here
    let mut x = 501;

    for rect in rectangles.iter() {
        if rect.used {
            continue;
        }
        if rect.y1 < y && y < rect.y2 {
            x = rect.x1;
            break;
        }
    }

    let mut tmp = Vec::new();
    for rect in rectangles.iter_mut() { // Use iter_mut to allow mutable access
        if rect.used {
            continue;
        }
        if rect.y2 <= y && rect.x2 <= x {
            tmp.push(rect.clone());
            rect.used = true; // Mark as used
        }
    }

    if !tmp.is_empty() {
        calculate_order(&mut tmp, order);
    }

    for i in 0..rectangles.len() {
        if !rectangles[i].used {
            calculate_order(&mut rectangles[i..].to_vec(), order);
            break;
        }
    }
}

fn main() {
    let mut rectangles = read_input();
    let mut order = vec![0; rectangles.len()];

    calculate_order(&mut rectangles, &mut order);

    for (i, &ord) in order.iter().enumerate() {
        if i > 0 {
            println!();
        }
        println!("{}", ord);
    }
}
