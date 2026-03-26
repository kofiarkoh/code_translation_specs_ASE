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
            let mut depature = String::new();
            stdin.lock().read_line(&mut depature).unwrap();
            let (hour, min, sec) = parse_time(&depature);
            let depature_seconds = hour * 3600 + min * 60 + sec;

            let mut arrival = String::new();
            stdin.lock().read_line(&mut arrival).unwrap();
            let (hour, min, sec) = parse_time(&arrival);
            let arrival_seconds = hour * 3600 + min * 60 + sec;

            // Increment the time array for the range [depature, arrival)
            for j in depature_seconds..arrival_seconds {
                time[j as usize] += 1;
            }
        }

        // Find the maximum value in the time array
        let max = time.iter().cloned().max().unwrap_or(0);
        println!("{}", max);
    }
}

// Helper function to parse time from a string in "H:M:S" format
fn parse_time(input: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = input.trim().split(':')
        .map(|s| s.parse().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}
