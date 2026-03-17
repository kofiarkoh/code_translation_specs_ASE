use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut time = [0; 24 * 3600]; // Array to hold time counts

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
            let mut depature = 0;
            let mut arrival = 0;

            // Read departure time
            let mut input = String::new();
            stdin.lock().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split(':').collect();
            let hour: i32 = parts[0].parse().unwrap();
            let min: i32 = parts[1].parse().unwrap();
            let sec: i32 = parts[2].parse().unwrap();
            depature = hour * 3600 + min * 60 + sec;

            // Read arrival time
            input.clear();
            stdin.lock().read_line(&mut input).unwrap();
            let parts: Vec<&str> = input.trim().split(':').collect();
            let hour: i32 = parts[0].parse().unwrap();
            let min: i32 = parts[1].parse().unwrap();
            let sec: i32 = parts[2].parse().unwrap();
            arrival = hour * 3600 + min * 60 + sec;

            // Increment the time array for the range [depature, arrival)
            for j in depature..arrival {
                time[j as usize] += 1;
            }
        }

        // Find the maximum value in the time array
        let max = time.iter().cloned().max().unwrap_or(0);
        
        // Print the result
        println!("{}", max);
    }
}
