use std::io::{self, BufRead};

fn line_intersection(ax: i32, ay: i32, bx: i32, by: i32, cx: i32, cy: i32, dx: i32, dy: i32) -> Option<(f64, f64)> {
    let acx = cx - ax;
    let acy = cy - ay;
    let bunbo = (bx - ax) * (dy - cy) - (by - ay) * (dx - cx);
    
    if bunbo != 0 {
        let r = ((dy - cy) * acx - (dx - cx) * acy);
        let s = ((by - ay) * acx - (bx - ax) * acy);
        
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
        
        coordinates.extend(nums);
        
        if coordinates.len() >= 6 {
            let (x1, y1, x2, y2) = (
                &coordinates[0..3],
                &coordinates[3..6],
                &coordinates[6..9],
                &coordinates[9..12],
            );

            let mut intersections = vec![];

            for i in 0..3 {
                if let Some((x, y)) = line_intersection(
                    x1[i], y1[i], x2[i], y2[i],
                    x1[(i + 1) % 3], y1[(i + 1) % 3], x2[(i + 1) % 3], y2[(i + 1) % 3]
                ) {
                    intersections.push((x, y));
                } else {
                    intersections.clear();
                    break;
                }
            }

            if intersections.len() == 3 {
                let area = (0..3).map(|i| {
                    (intersections[i].0 - intersections[(i + 1) % 3].0) * 
                    (intersections[i].1 + intersections[(i + 1) % 3].1) / 2.0
                }).sum::<f64>().abs();

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

            coordinates.clear();
        }
    }
}
