use std::io::{self, Write};

fn time_print(time: i32) {
    let hours = time / 3600;
    let minutes = (time % 3600) / 60;
    let seconds = time % 60;
    // Print formatted time with leading zeros
    println!("{:02}:{:02}:{:02}", hours, minutes, seconds);
}

fn main() {
    let mut input = String::new();
    loop {
        // Read input from stdin
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut parts = input.split_whitespace();
        
        // Parse integers from input
        let t: i32 = parts.next().unwrap_or("-1").parse().unwrap_or(-1);
        let h: i32 = parts.next().unwrap_or("-1").parse().unwrap_or(-1);
        let s: i32 = parts.next().unwrap_or("-1").parse().unwrap_or(-1);
        
        // Check for termination condition
        if t == -1 && h == -1 && s == -1 {
            break;
        }
        
        // Calculate time in seconds
        let time = t * 3600 + h * 60 + s;
        time_print(7200 - time);
        time_print(7200 - time / 3);
    }
}
