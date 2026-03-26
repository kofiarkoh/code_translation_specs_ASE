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
        let n: usize = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n {
            let coords: Vec<i32> = reader.lines().next().unwrap().unwrap()
                .split_whitespace()
                .map(|s| s.parse().unwrap())
                .collect();
            let (x1, y1, x2, y2) = (coords[0], coords[1], coords[2], coords[3]);
            rectangles.push(Rectangle {
                id: i,
                x1: x1.min(x2),
                y1: y1.min(y2),
                x2: x1.max(x2),
                y2: y1.max(y2),
                used: false,
            });
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

fn calc(rectangles: &mut Vec<Rectangle>, order: &mut Vec<usize>, n: usize) {
    rectangles.sort_by(compare_rectangles);
    order[rectangles[0].id] = order.iter().filter(|&&x| x != 0).count() + 1;
    rectangles[0].used = true;

    let y = rectangles[0].y2;
    let mut x = 501;

    for rect in rectangles.iter() {
        if !rect.used && rect.y1 < y && y < rect.y2 {
            x = rect.x1;
            break;
        }
    }

    let mut tmp = Vec::new();
    for rect in rectangles.iter_mut() {
        if !rect.used && rect.y2 <= y && rect.x2 <= x {
            tmp.push(rect.clone());
            rect.used = true;
        }
    }

    if !tmp.is_empty() {
        calc(&mut tmp, order, tmp.len());
    }

    // Use a separate index to avoid mutable borrow issues
    let mut start_index = 0;
    for i in 0..n {
        if !rectangles[i].used {
            start_index = i;
            break;
        }
    }
    if start_index < n {
        calc(&mut rectangles[start_index..].to_vec(), order, n - start_index);
    }
}

fn main() {
    let mut order = vec![0; 10];
    let rectangles = read_input();

    let mut cno = 0;
    let mut rectangles = rectangles;

    while !rectangles.is_empty() {
        let n = rectangles.len();
        if n == 0 {
            break;
        }

        calc(&mut rectangles, &mut order, n);

        if cno > 0 {
            println!();
        }
        cno += 1;

        for i in 0..n {
            println!("{}", order[i]);
        }
    }
}
