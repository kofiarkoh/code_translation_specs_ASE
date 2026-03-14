use std::io::{self, BufRead};

fn gaiseki(x0: f64, y0: f64, x1: f64, y1: f64, x2: f64, y2: f64) -> f64 {
    (x0 - x1) * (y0 - y1) - (x0 - x2) * (y0 - y2)
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut coords = line.split(',').map(|s| s.parse::<f64>().expect("Failed to parse coordinate"));

        let x0 = coords.next().unwrap();
        let y0 = coords.next().unwrap();
        let x1 = coords.next().unwrap();
        let y1 = coords.next().unwrap();
        let x2 = coords.next().unwrap();
        let y2 = coords.next().unwrap();
        let x3 = coords.next().unwrap();
        let y3 = coords.next().unwrap();

        if gaiseki(x0, y0, x1, y1, x3, y3) >= 0.0 &&
           gaiseki(x1, y1, x2, y2, x3, y3) >= 0.0 &&
           gaiseki(x2, y2, x0, y0, x3, y3) >= 0.0 {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
