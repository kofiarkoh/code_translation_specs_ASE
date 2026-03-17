use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Loop to process multiple sets of time intervals
    for line in reader.lines() {
        let line = line.unwrap();
        let n: usize = line.trim().parse().expect("Invalid input for number of intervals");

        // Exit if n is 0
        if n == 0 {
            break;
        }

        // Initialize the time array to count overlapping intervals
        let mut time = vec![0; 24 * 3600];

        for _ in 0..n {
            let mut depature = String::new();
            let mut arrival = String::new();

            // Read departure time
            reader.read_line(&mut depature).expect("Failed to read departure time");
            let (h, m, s) = parse_time(&depature);
            let depature_seconds = h * 3600 + m * 60 + s;

            // Read arrival time
            reader.read_line(&mut arrival).expect("Failed to read arrival time");
            let (h, m, s) = parse_time(&arrival);
            let arrival_seconds = h * 3600 + m * 60 + s;

            // Increment the time array for each second in the interval
            for j in depature_seconds..arrival_seconds {
                time[j] += 1;
            }
        }

        // Find the maximum number of overlapping intervals
        let max_overlaps = time.iter().cloned().max().unwrap_or(0);
        println!("{}", max_overlaps);
    }
}

// Helper function to parse time from "HH:MM:SS" format
fn parse_time(time_str: &str) -> (i32, i32, i32) {
    let parts: Vec<&str> = time_str.trim().split(':').collect();
    let h = parts[0].parse::<i32>().expect("Invalid hour");
    let m = parts[1].parse::<i32>().expect("Invalid minute");
    let s = parts[2].parse::<i32>().expect("Invalid second");
    (h, m, s)
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` to read lines from standard input, which is idiomatic for handling input in Rust.
2. **Time Array**: The `time` array is initialized as a `Vec<i32>` with a length of 86400, which is equivalent to the C array.
3. **Looping and Incrementing**: The logic for incrementing the time array remains the same, ensuring that the semantics are preserved.
4. **Maximum Overlaps**: The maximum value is found using `iter().cloned().max()`, which is a safe and idiomatic way to find the maximum in Rust.
5. **Error Handling**: The code assumes well-formed input as per the specification, similar to the C code, and does not handle invalid input gracefully.
6. **Functionality**: The `parse_time` function is added to encapsulate the logic for parsing time strings, improving readability and maintainability.
