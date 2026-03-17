use std::io::{self, BufRead};

fn main() {
    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of intervals
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // If n is 0, we exit without processing any intervals
    if n == 0 {
        println!("0");
        return;
    }

    // Initialize a time array to count overlapping intervals for each second in a 24-hour period
    let mut time = [0; 86400]; // 24 * 60 * 60 = 86400 seconds in a day

    // Process each interval
    for _ in 0..n {
        // Read the departure and arrival times
        let dep_time = lines.next().unwrap().unwrap();
        let arr_time = lines.next().unwrap().unwrap();

        // Parse the departure time
        let (dep_hour, dep_min, dep_sec) = parse_time(&dep_time);
        // Parse the arrival time
        let (arr_hour, arr_min, arr_sec) = parse_time(&arr_time);

        // Calculate the start and end seconds of the interval
        let start = dep_hour * 3600 + dep_min * 60 + dep_sec;
        let end = arr_hour * 3600 + arr_min * 60 + arr_sec;

        // Increment the count for each second in the interval
        for second in start..=end {
            time[second] += 1; // Increment the count for the current second
        }
    }

    // Find the maximum count of overlapping intervals
    let max_overlaps = time.iter().copied().max().unwrap_or(0);

    // Output the maximum number of overlapping intervals
    println!("{}", max_overlaps);
}

// Helper function to parse time in HH:MM:SS format
fn parse_time(time_str: &str) -> (i32, i32, i32) {
    let parts: Vec<&str> = time_str.split(':').collect();
    let hour = parts[0].parse::<i32>().unwrap();
    let min = parts[1].parse::<i32>().unwrap();
    let sec = parts[2].parse::<i32>().unwrap();
    (hour, min, sec)
}
