use std::io::{self, BufRead};

#[derive(Debug)]
struct Wall {
    x: i32,
    y: i32,
    radius: i32,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read the number of walls
    let mut lines = reader.lines();
    let num_walls: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if num_walls == 0 {
        return; // Terminate if non-positive
    }

    let mut walls = Vec::with_capacity(num_walls);
    
    // Read wall data
    for _ in 0..num_walls {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        walls.push(Wall {
            x: parts[0],
            y: parts[1],
            radius: parts[2],
        });
    }

    // Read the number of queries
    let num_queries: usize = lines.next().unwrap().unwrap().parse().unwrap();
    if num_queries == 0 {
        return; // Terminate if non-positive
    }

    // Process each query
    for _ in 0..num_queries {
        let line = lines.next().unwrap().unwrap();
        let parts: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();
        
        let (x1, y1, x2, y2) = (parts[0], parts[1], parts[2], parts[3]);
        
        if is_hidden_by_wall(x1, y1, x2, y2, &walls) {
            println!("Danger");
        } else {
            println!("Safe");
        }
    }
}

fn is_hidden_by_wall(x1: i32, y1: i32, x2: i32, y2: i32, walls: &[Wall]) -> bool {
    for wall in walls {
        if line_intersects_circle(x1, y1, x2, y2, wall) {
            return true; // Hidden by at least one wall
        }
    }
    false // Not hidden by any wall
}

fn line_intersects_circle(x1: i32, y1: i32, x2: i32, y2: i32, wall: &Wall) -> bool {
    let dx = x2 - x1;
    let dy = y2 - y1;

    let a = dx * dx + dy * dy;
    let b = 2 * (dx * (x1 - wall.x) + dy * (y1 - wall.y));
    let c = (x1 - wall.x) * (x1 - wall.x) + (y1 - wall.y) * (y1 - wall.y) - wall.radius * wall.radius;

    let discriminant = b * b - 4 * a * c;

    // Check if the line intersects the circle
    discriminant >= 0
}
