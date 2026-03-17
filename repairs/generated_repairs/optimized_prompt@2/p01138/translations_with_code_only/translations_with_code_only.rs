use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut time = [0; 24 * 3600]; // Array to hold the count of departures/arrivals

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = input.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        // Reset the time array
        time.fill(0);

        for _ in 0..n {
            let mut depature_input = String::new();
            stdin.lock().read_line(&mut depature_input).unwrap();
            let depature_parts: Vec<&str> = depature_input.trim().split(':').collect();
            let hour: i32 = depature_parts[0].parse().unwrap_or(0);
            let min: i32 = depature_parts[1].parse().unwrap_or(0);
            let sec: i32 = depature_parts[2].parse().unwrap_or(0);
            let depature = hour * 3600 + min * 60 + sec;

            let mut arrival_input = String::new();
            stdin.lock().read_line(&mut arrival_input).unwrap();
            let arrival_parts: Vec<&str> = arrival_input.trim().split(':').collect();
            let hour: i32 = arrival_parts[0].parse().unwrap_or(0);
            let min: i32 = arrival_parts[1].parse().unwrap_or(0);
            let sec: i32 = arrival_parts[2].parse().unwrap_or(0);
            let arrival = hour * 3600 + min * 60 + sec;

            // Ensure arrival is greater than departure
            if arrival > depature {
                // Increment the time array for the range from depature to arrival
                for j in depature..arrival {
                    if j >= 0 && j < time.len() as i32 {
                        time[j as usize] += 1; // Safe as j is within bounds
                    }
                }
            }
        }

        // Find the maximum value in the time array
        let max = time.iter().cloned().max().unwrap_or(0);
        
        // Print the result
        println!("{}", max);
    }
}
