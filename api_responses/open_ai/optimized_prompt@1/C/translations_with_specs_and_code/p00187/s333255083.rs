use std::io::{self, BufRead};

fn line_intersection(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> Option<(f64, f64)> {
    let acx = cx - ax;
    let acy = cy - ay;
    let bunbo = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
    
    if bunbo != 0 {
        let r = (dy - cy) * acx - (dx - cx) * acy;
        let s = (by - ay) * acx - (bx - ax) * acy;

        let (r, s, bunbo) = if bunbo < 0 {
            (-r, -s, -bunbo)
        } else {
            (r, s, bunbo)
        };

        if r >= 0 && r <= bunbo && s >= 0 && s <= bunbo {
            let x = ax as f64 + r as f64 / bunbo as f64 * (bx - ax) as f64;
            let y = ay as f64 + r as f64 / bunbo as f64 * (by - ay) as f64;
            return Some((x, y));
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut coordinates = vec![];

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if nums.len() == 4 {
            coordinates.push((nums[0], nums[1], nums[2], nums[3]));
        }

        if coordinates.len() == 3 {
            let (x1, y1, x2, y2) = coordinates[0];
            let (x3, y3, x4, y4) = coordinates[1];
            let (x5, y5, x6, y6) = coordinates[2];

            let mut points = [None; 3];
            points[0] = line_intersection(x1, y1, x2, y2, x3, y3, x4, y4);
            points[1] = line_intersection(x3, y3, x4, y4, x5, y5, x6, y6);
            points[2] = line_intersection(x5, y5, x6, y6, x1, y1, x2, y2);

            if points.iter().all(|&p| p.is_some()) {
                let (x0, y0) = points[0].unwrap();
                let (x1, y1) = points[1].unwrap();
                let (x2, y2) = points[2].unwrap();

                let area = ((x0 - x1) * (y0 + y1) + (x1 - x2) * (y1 + y2) + (x2 - x0) * (y2 + y0)) / 2.0;
                let area = area.abs();

                if area < 1e-6 {
                    println!("kyo");
                } else if area < 100000.0 - 1e-6 {
                    println!("syo-kichi");
                } else if area < 1000000.0 - 1e-6 {
                    println!("kichi");
                } else if area < 1900000.0 - 1e-6 {
                    println!("chu-kichi");
                } else {
                    println!("dai-kichi");
                }
            } else {
                println!("kyo");
            }
            coordinates.clear(); // Reset for the next set of inputs
        }
    }
}
