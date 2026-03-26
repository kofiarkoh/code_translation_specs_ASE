use std::io::{self, Write};

fn main() {
    loop {
        // Read input from the user
        let mut input = String::new();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        // Parse the input
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        if parts.len() != 3 {
            continue; // Ignore invalid input
        }

        let h: i32 = match parts[0].parse() {
            Ok(val) if val >= 0 && val <= 23 => val,
            _ => continue, // Ignore invalid hour
        };

        let m: i32 = match parts[1].parse() {
            Ok(val) if val >= 0 && val <= 59 => val,
            _ => continue, // Ignore invalid minute
        };

        let s: i32 = match parts[2].parse() {
            Ok(val) if val >= 0 && val <= 59 => val,
            _ => continue, // Ignore invalid second
        };

        // Check for termination condition
        if h == -1 && m == -1 && s == -1 {
            break; // Exit the loop
        }

        // Calculate current seconds and remaining time
        let cur_sec = h * 3600 + m * 60 + s;
        let remaining = 2 * 3600 - cur_sec;

        // Output the remaining time in HH:MM:SS format
        println!("{:02}:{:02}:{:02}", remaining / 3600, (remaining % 3600) / 60, remaining % 60);

        // Calculate and output the remaining time multiplied by three
        let remaining_tripled = remaining * 3;
        println!("{:02}:{:02}:{:02}", remaining_tripled / 3600, (remaining_tripled % 3600) / 60, remaining_tripled % 60);
    }
}
``` 

### Changes Made:
1. **Removed Prompt**: The prompt for user input was removed to match the expected behavior of reading input directly without additional prompts, which may have caused the timeout.
2. **Input Handling**: The loop continues to read input until the termination condition is met, ensuring it processes inputs efficiently.
3. **Error Handling**: The program continues to ignore invalid inputs without additional prompts, which should help avoid unnecessary delays.
