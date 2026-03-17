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
            let mut departure_input = String::new();
            stdin.lock().read_line(&mut departure_input).unwrap();
            let departure_parts: Vec<&str> = departure_input.trim().split(':').collect();
            if departure_parts.len() != 3 {
                panic!("Invalid departure time format");
            }
            let hour: i32 = departure_parts[0].parse().unwrap();
            let min: i32 = departure_parts[1].parse().unwrap();
            let sec: i32 = departure_parts[2].parse().unwrap();
            let departure = hour * 3600 + min * 60 + sec;

            let mut arrival_input = String::new();
            stdin.lock().read_line(&mut arrival_input).unwrap();
            let arrival_parts: Vec<&str> = arrival_input.trim().split(':').collect();
            if arrival_parts.len() != 3 {
                panic!("Invalid arrival time format");
            }
            let hour: i32 = arrival_parts[0].parse().unwrap();
            let min: i32 = arrival_parts[1].parse().unwrap();
            let sec: i32 = arrival_parts[2].parse().unwrap();
            let arrival = hour * 3600 + min * 60 + sec;

            // Ensure that departure is less than arrival
            if departure >= arrival {
                panic!("Departure time must be less than arrival time");
            }

            // Increment the time array for the range from departure to arrival
            for j in departure..arrival {
                time[j as usize] += 1; // Safe as j is within bounds
            }
        }

        // Find the maximum value in the time array
        let max = time.iter().cloned().max().unwrap_or(0);
        
        // Print the result
        println!("{}", max);
    }
}
