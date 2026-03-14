use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut values = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok());

        let x: [f64; 4] = [
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
        ];
        
        let y: [f64; 4] = [
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
            values.next().unwrap_or(0.0),
        ];

        if (x[0] <= x[2] && x[1] >= x[2]) || (x[0] <= x[3] && x[1] >= x[3]) {
            if (y[0] <= y[2] && y[1] >= y[2]) || (y[0] <= y[3] && y[1] >= y[3]) {
                println!("YES");
            } else {
                println!("NO");
            }
        } else {
            println!("NO");
        }
    }
}
