use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Invalid number of watches");

    if n == 0 {
        return; // Terminate if n is 0, as per specification
    }

    let mut times = Vec::new();

    for _ in 0..n {
        let mut time_input = String::new();
        io::stdin().read_line(&mut time_input).expect("Failed to read time");
        let t: Vec<i32> = time_input
            .trim()
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid time input"))
            .collect();

        // Ensure each element of t is in the range [0, 59]
        if t.len() == 3 && t.iter().all(|&x| x >= 0 && x <= 59) {
            times.push(t);
        }
    }

    let mut min_time = i32::MAX;
    let mut max_time = i32::MIN;

    // Calculate the time in seconds for each watch
    let seconds: Vec<i32> = times.iter()
        .map(|t| t[0] * 3600 + t[1] * 60 + t[2])
        .collect();

    // Compare each pair of watches to find min and max intervals
    for i in 0..n {
        for j in i + 1..n {
            let diff = (seconds[i] - seconds[j]).abs();
            let interval = if diff > 3600 { 7200 - diff } else { diff };

            if interval < min_time {
                min_time = interval;
            }
            if interval > max_time {
                max_time = interval;
            }
        }
    }

    // Convert seconds back to HH:MM:SS format
    let min_h = min_time / 3600;
    let min_m = (min_time % 3600) / 60;
    let min_s = min_time % 60;

    let max_h = max_time / 3600;
    let max_m = (max_time % 3600) / 60;
    let max_s = max_time % 60;

    // Print the results in the specified format
    println!("{:02}:{:02}:{:02} {:02}:{:02}:{:02}", min_h, min_m, min_s, max_h, max_m, max_s);
}
