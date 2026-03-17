fn main() {
    use std::io::{self, BufRead};

    // Read input from standard input
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        
        // Read the number of intervals
        let num_intervals: usize = parts.next().unwrap().parse().unwrap();
        
        // Terminate if the input is zero
        if num_intervals == 0 {
            break;
        }

        // Initialize an array to count overlaps for each second of the day
        let mut time_array = vec![0; 86400]; // 86400 seconds in a day

        // Process each interval
        for _ in 0..num_intervals {
            let h1: usize = parts.next().unwrap().parse().unwrap();
            let m1: usize = parts.next().unwrap().parse().unwrap();
            let s1: usize = parts.next().unwrap().parse().unwrap();
            let h2: usize = parts.next().unwrap().parse().unwrap();
            let m2: usize = parts.next().unwrap().parse().unwrap();
            let s2: usize = parts.next().unwrap().parse().unwrap();

            // Convert times to seconds
            let start = h1 * 3600 + m1 * 60 + s1;
            let end = h2 * 3600 + m2 * 60 + s2;

            // Increment the count for the start time
            time_array[start] += 1;

            // Decrement the count for the end time (to mark the end of the interval)
            if end < 86400 {
                time_array[end] -= 1;
            }
        }

        // Calculate the maximum overlap
        let mut max_overlap = 0;
        let mut current_overlap = 0;

        for count in time_array {
            current_overlap += count;
            if current_overlap > max_overlap {
                max_overlap = current_overlap;
            }
        }

        // Output the maximum overlap
        println!("{}", max_overlap);
    }
}
