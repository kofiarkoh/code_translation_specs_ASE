use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Rectangle {
    id: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

impl Rectangle {
    // Check if this rectangle overlaps with another
    fn overlaps(&self, other: &Rectangle) -> bool {
        self.x1 < other.x2 && self.x2 > other.x1 && self.y1 < other.y2 && self.y2 > other.y1
    }

    // Check if this rectangle contains another
    fn contains(&self, other: &Rectangle) -> bool {
        self.x1 <= other.x1 && self.x2 >= other.x2 && self.y1 <= other.y1 && self.y2 >= other.y2
    }
}

fn process_rectangles(rectangles: &mut Vec<Rectangle>) -> Vec<usize> {
    let mut order = Vec::new();
    let mut used = vec![false; rectangles.len()];

    // Process rectangles based on their relationships
    for i in 0..rectangles.len() {
        if used[i] {
            continue;
        }
        used[i] = true;
        order.push(rectangles[i].id);

        for j in 0..rectangles.len() {
            if !used[j] && (rectangles[i].overlaps(&rectangles[j]) || rectangles[i].contains(&rectangles[j])) {
                used[j] = true;
                order.push(rectangles[j].id);
            }
        }
    }

    order
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut first_set = true;

    for line in reader.lines() {
        let line = line.unwrap();
        let mut tokens = line.split_whitespace();
        let count: usize = tokens.next().unwrap().parse().unwrap();

        let mut rectangles = Vec::new();

        for id in 0..count {
            let x1: i32 = tokens.next().unwrap().parse().unwrap();
            let y1: i32 = tokens.next().unwrap().parse().unwrap();
            let x2: i32 = tokens.next().unwrap().parse().unwrap();
            let y2: i32 = tokens.next().unwrap().parse().unwrap();
            rectangles.push(Rectangle { id, x1, y1, x2, y2 });
        }

        let order = process_rectangles(&mut rectangles);

        if !first_set {
            println!(); // Separate sets with a newline
        }
        first_set = false;

        for id in order {
            println!("{}", id);
        }
    }
}
