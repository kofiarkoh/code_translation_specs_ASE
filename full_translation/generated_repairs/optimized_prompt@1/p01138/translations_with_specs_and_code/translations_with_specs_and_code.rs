use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut reader = stdin.lock(); // Changed to mutable

    loop {
        let mut input = String::new();
        reader.read_line(&mut input).unwrap();
        let n: usize = input.trim().parse().unwrap();

        if n == 0 {
            break;
        }

        // Initialize an array to count overlaps for each second of the day
        let mut time = vec![0; 24 * 3600];

        for _ in 0..n {
            let mut depature_input = String::new();
            reader.read_line(&mut depature_input).unwrap();
            let (hour, min, sec) = parse_time(&depature_input);
            let depature = hour * 3600 + min * 60 + sec;

            let mut arrival_input = String::new();
            reader.read_line(&mut arrival_input).unwrap();
            let (hour, min, sec) = parse_time(&arrival_input);
            let arrival = hour * 3600 + min * 60 + sec;

            // Increment the count for each second in the interval
            for j in depature..arrival {
                time[j] += 1;
            }
        }

        // Find the maximum overlap
        let max_overlap = time.iter().cloned().max().unwrap_or(0);
        println!("{}", max_overlap);
    }
}

// Helper function to parse time from a string in "H:M:S" format
fn parse_time(input: &str) -> (usize, usize, usize) {
    let parts: Vec<usize> = input.trim()
        .split(':')
        .map(|s| s.parse().unwrap())
        .collect();
    (parts[0], parts[1], parts[2])
}
