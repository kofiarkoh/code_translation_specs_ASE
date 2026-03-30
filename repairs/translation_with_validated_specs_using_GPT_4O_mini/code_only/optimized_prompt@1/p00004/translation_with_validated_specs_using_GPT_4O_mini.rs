use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut values = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok());

        // Read six values from the input
        let a = match values.next() {
            Some(v) => v,
            None => break,
        };
        let b = match values.next() {
            Some(v) => v,
            None => break,
        };
        let c = match values.next() {
            Some(v) => v,
            None => break,
        };
        let d = match values.next() {
            Some(v) => v,
            None => break,
        };
        let e = match values.next() {
            Some(v) => v,
            None => break,
        };
        let f = match values.next() {
            Some(v) => v,
            None => break,
        };

        // Perform calculations
        let bai = d / a;
        let aa = a * bai;
        let bb = b * bai;
        let cc = c * bai;
        let y = (f - cc) / (e - bb);
        let x = (c - b * y) / a;

        // Print results with three decimal places
        println!("{:.3} {:.3}", x, y);
    }
}
