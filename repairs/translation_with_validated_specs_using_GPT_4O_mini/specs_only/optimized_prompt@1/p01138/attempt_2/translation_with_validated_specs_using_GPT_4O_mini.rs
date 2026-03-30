use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        
        // Read the number of intervals
        let n: usize = parts.next().unwrap().parse().unwrap();
        if n == 0 {
            break; // Terminate gracefully if the first input integer is zero
        }

        // Initialize a fixed-size array for tracking overlaps for each second of the day
        let mut time_array = [0; 86400]; // 24 hours * 60 minutes * 60 seconds

        for _ in 0..n {
            let departure = parts.next().unwrap();
            let arrival = parts.next().unwrap();

            // Convert time strings to seconds since the start of the day
            let dep_seconds = time_to_seconds(departure);
            let arr_seconds = time_to_seconds(arrival);

            // Increment the count for the departure second
            time_array[dep_seconds] += 1;
            // Decrement the count for the second after arrival
            if arr_seconds < 86400 {
                time_array[arr_seconds] -= 1; // Ensure we don't go out of bounds
            }
        }

        // Calculate the maximum number of overlapping intervals
        let mut max_overlap = 0;
        let mut current_overlap = 0;

        for count in time_array.iter() {
            current_overlap += count;
            if current_overlap > max_overlap {
                max_overlap = current_overlap;
            }
        }

        // Output the maximum overlap
        println!("{}", max_overlap);
    }
}

// Helper function to convert time in "HH:MM:SS" format to seconds
fn time_to_seconds(time: &str) -> usize {
    let parts: Vec<&str> = time.split(':').collect();
    if parts.len() != 3 {
        return 0; // Return 0 if the format is incorrect, to avoid panic
    }
    let hours: usize = parts[0].parse().unwrap_or(0);
    let minutes: usize = parts[1].parse().unwrap_or(0);
    let seconds: usize = parts[2].parse().unwrap_or(0);
    hours * 3600 + minutes * 60 + seconds
}
