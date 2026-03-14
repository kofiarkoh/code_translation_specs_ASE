use std::io::{self, BufRead};

#[derive(Debug, Clone)]
struct Point {
    x: f64,
    y: f64,
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let zahyou_num: usize = line.trim().parse().unwrap();
        if zahyou_num == 0 {
            break;
        }

        let mut points = Vec::new();
        for _ in 0..zahyou_num {
            let mut point_line = String::new();
            reader.read_line(&mut point_line).unwrap();
            let coords: Vec<&str> = point_line.trim().split(',').collect();
            let x: f64 = coords[0].parse().unwrap();
            let y: f64 = coords[1].parse().unwrap();
            points.push(Point { x, y });
        }

        // Sort points
        points.sort_by(|a, b| {
            if a.x != b.x {
                a.x.partial_cmp(&b.x).unwrap()
            } else {
                a.y.partial_cmp(&b.y).unwrap()
            }
        });

        let mut th_down = Vec::new();
        th_down.push(points[0].clone());

        for i in 1..zahyou_num {
            while th_down.len() > 1 {
                let k1 = slope(&points[i], &th_down[th_down.len() - 1]);
                let k2 = slope(&th_down[th_down.len() - 1], &th_down[th_down.len() - 2]);
                if k1 >= k2 {
                    break;
                }
                th_down.pop();
            }
            th_down.push(points[i].clone());
        }

        let mut th_up = Vec::new();
        th_up.push(points[zahyou_num - 1].clone());

        for i in (0..zahyou_num - 1).rev() {
            while th_up.len() > 1 {
                let k1 = slope(&points[i], &th_up[th_up.len() - 1]);
                let k2 = slope(&th_up[th_up.len() - 1], &th_up[th_up.len() - 2]);
                if k1 >= k2 {
                    break;
                }
                th_up.pop();
            }
            th_up.push(points[i].clone());
        }

        let not_in_hull = zahyou_num - (th_down.len() + th_up.len() - 2);
        println!("{}", not_in_hull);
    }
}

// Function to calculate the slope between two points
fn slope(p1: &Point, p2: &Point) -> f64 {
    if p1.x == p2.x {
        // Handle vertical line case
        if p1.y < p2.y {
            f64::INFINITY // Representing positive infinity
        } else if p1.y > p2.y {
            f64::NEG_INFINITY // Representing negative infinity
        } else {
            0.0 // Same point
        }
    } else {
        (p1.y - p2.y) / (p1.x - p2.x)
    }
}
```

### Notes:
1. **Data Structures**: The `Point` struct is used to represent 2D points, similar to the `xy_t` struct in C.
2. **Input Handling**: The program reads input using `std::io`, which is idiomatic in Rust. It handles reading lines and parsing them into floating-point numbers.
3. **Sorting**: The points are sorted using Rust's `sort_by` method, which is safe and idiomatic.
4. **Convex Hull Calculation**: The logic for calculating the convex hull is preserved, with the use of vectors to dynamically manage the hull points.
5. **Slope Calculation**: A separate function `slope` is created to handle the slope calculations, including special cases for vertical lines.
6. **Output**: The output format is preserved exactly as specified, ensuring the correct number of points not in the convex hull is printed.
7. **Error Handling**: The program does not handle invalid input explicitly, as per the specification. It assumes valid input is provided.
