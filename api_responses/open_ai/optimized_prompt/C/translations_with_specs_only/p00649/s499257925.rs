use std::io::{self, BufRead};

#[derive(Debug)]
struct Rectangle {
    index: usize,
    x1: i32,
    y1: i32,
    x2: i32,
    y2: i32,
}

// Function to determine the order of rectangles based on their spatial relationships
fn order_rectangles(rectangles: &mut Vec<Rectangle>) {
    // Sort rectangles based on their top-left corner (x1, y1)
    rectangles.sort_by(|a, b| {
        if a.y1 != b.y1 {
            a.y1.cmp(&b.y1)
        } else {
            a.x1.cmp(&b.x1)
        }
    });
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        
        // Read the number of rectangles
        let num_rectangles: usize = iter.next().unwrap().parse().unwrap();
        let mut rectangles = Vec::with_capacity(num_rectangles);

        // Read each rectangle's coordinates
        for i in 0..num_rectangles {
            let x1: i32 = iter.next().unwrap().parse().unwrap();
            let y1: i32 = iter.next().unwrap().parse().unwrap();
            let x2: i32 = iter.next().unwrap().parse().unwrap();
            let y2: i32 = iter.next().unwrap().parse().unwrap();
            rectangles.push(Rectangle { index: i, x1, y1, x2, y2 });
        }

        // Order the rectangles based on their spatial relationships
        order_rectangles(&mut rectangles);

        // Output the order of rectangle indices
        let order: Vec<usize> = rectangles.iter().map(|r| r.index).collect();
        println!("{}", order.iter().map(|i| i.to_string()).collect::<Vec<String>>().join(" "));
    }
}
