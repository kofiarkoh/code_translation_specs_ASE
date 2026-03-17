use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        let mut time = vec![0; 24 * 3600]; // Array to count overlaps for each second of the day

        for _ in 0..n {
            let mut depature = String::new();
            let mut arrival = String::new();

            // Read departure time
            reader.read_line(&mut depature).unwrap();
            let (h, m, s) = parse_time(&depature);
            let depature_seconds = h * 3600 + m * 60 + s;

            // Read arrival time
            reader.read_line(&mut arrival).unwrap();
            let (h, m, s) = parse_time(&arrival);
            let arrival_seconds = h * 3600 + m * 60 + s;

            // Increment the time array for the interval
            for j in depature_seconds..arrival_seconds {
                time[j] += 1;
            }
        }

        // Find the maximum overlap
        let max_overlap = time.iter().cloned().max().unwrap_or(0);
        println!("{}", max_overlap);
    }
}

// Helper function to parse time from a string in "H:M:S" format
fn parse_time(time_str: &str) -> (i32, i32, i32) {
    let parts: Vec<i32> = time_str.trim()
        .split(':')
        .map(|part| part.parse().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}
