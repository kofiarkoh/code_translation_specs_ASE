use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        if let Ok(w) = line.trim().parse::<f64>() {
            let s = if w <= 48.0 {
                "light fly"
            } else if w <= 51.0 {
                "fly"
            } else if w <= 54.0 {
                "bantam"
            } else if w <= 57.0 {
                "feather"
            } else if w <= 60.0 {
                "light"
            } else if w <= 64.0 {
                "light welter"
            } else if w <= 69.0 {
                "welter"
            } else if w <= 75.0 {
                "light middle"
            } else if w <= 81.0 {
                "middle"
            } else if w <= 91.0 {
                "light heavy"
            } else {
                "heavy"
            };

            println!("{}", s);
        }
    }
}
