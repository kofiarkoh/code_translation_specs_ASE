use std::io::{self, Write};

fn time_print(time: i32) {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;
    // Output formatted time as HH:MM:SS
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn main() {
    let mut input = String::new();
    
    loop {
        // Read input from stdin
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        
        // Parse input values
        let values: Vec<i32> = input
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();
        
        if values.len() != 3 {
            continue; // Skip invalid input
        }

        let (t, h, s) = (values[0], values[1], values[2]);

        // Check for termination condition
        if t == -1 && h == -1 && s == -1 {
            break;
        }

        // Calculate total time in seconds
        let time = t * 3600 + h * 60 + s;

        // Calculate remaining time and one-third of that time
        time_print(7200 - time);
        time_print(7200 - time / 3);
    }
}
