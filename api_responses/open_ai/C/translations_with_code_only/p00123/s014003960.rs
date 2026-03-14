use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap(); // Unwrap the Result to get the line
        let mut values = line.split_whitespace(); // Split the line into whitespace-separated values

        // Attempt to parse two floats from the line
        if let (Some(x_str), Some(y_str)) = (values.next(), values.next()) {
            if let (Ok(x), Ok(y)) = (x_str.parse::<f32>(), y_str.parse::<f32>()) {
                // Check conditions and print corresponding output
                if x < 35.5 && y < 71.0 {
                    println!("AAA");
                } else if x < 37.5 && y < 77.0 {
                    println!("AA");
                } else if x < 40.0 && y < 83.0 {
                    println!("A");
                } else if x < 43.0 && y < 89.0 {
                    println!("B");
                } else if x < 50.0 && y < 105.0 {
                    println!("C");
                } else if x < 55.0 && y < 116.0 {
                    println!("D");
                } else if x < 70.0 && y < 148.0 {
                    println!("E");
                } else {
                    println!("NA");
                }
            }
        } else {
            // If parsing fails, we can choose to ignore or handle it as needed
            // In this case, we simply continue to the next line
            continue;
        }
    }
}
