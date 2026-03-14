use std::io::{self, BufRead};

fn gaiseki(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x0 - x1) * (y0 - y1) - (x0 - x2) * (y0 - y2)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Assuming valid input as per specification
        let coords: Vec<&str> = line.split(',').collect();

        if coords.len() != 8 {
            continue; // Skip lines that do not have exactly 8 values
        }

        let x0: f64 = coords[0].parse().unwrap();
        let y0: f64 = coords[1].parse().unwrap();
        let x1: f64 = coords[2].parse().unwrap();
        let y1: f64 = coords[3].parse().unwrap();
        let x2: f64 = coords[4].parse().unwrap();
        let y2: f64 = coords[5].parse().unwrap();
        let x3: f64 = coords[6].parse().unwrap();
        let y3: f64 = coords[7].parse().unwrap();

        if gaiseki(x0, y0, x1, y1, x3, y3) >= 0.0 &&
           gaiseki(x1, y1, x2, y2, x3, y3) >= 0.0 &&
           gaiseki(x2, y2, x0, y0, x3, y3) >= 0.0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
