use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Assuming valid input as per specification
        let mut values = line.split_whitespace()
            .filter_map(|s| s.parse::<f32>().ok()); // Parse floats, ignoring invalid input

        // Read pairs of values
        if let (Some(x), Some(y)) = (values.next(), values.next()) {
            // Determine the label based on the conditions
            let label = if x < 35.5 && y < 71.0 {
                "AAA"
            } else if x < 37.5 && y < 77.0 {
                "AA"
            } else if x < 40.0 && y < 83.0 {
                "A"
            } else if x < 43.0 && y < 89.0 {
                "B"
            } else if x < 50.0 && y < 105.0 {
                "C"
            } else if x < 55.0 && y < 116.0 {
                "D"
            } else if x < 70.0 && y < 148.0 {
                "E"
            } else {
                "NA"
            };

            // Output the label
            println!("{}", label);
        }
    }
}
