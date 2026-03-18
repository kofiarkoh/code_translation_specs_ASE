use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut time = [0; 24 * 3600]; // Array to hold counts for each second of the day

    loop {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).unwrap();
        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Skip invalid input
        };

        if n == 0 {
            break;
        }

        // Reset the time array
        time.fill(0);

        for _ in 0..n {
            let mut depature = String::new();
            stdin.lock().read_line(&mut depature).unwrap();
            let (hour, min, sec) = parse_time(&depature);
            let depature = hour * 3600 + min * 60 + sec;

            let mut arrival = String::new();
            stdin.lock().read_line(&mut arrival).unwrap();
            let (hour, min, sec) = parse_time(&arrival);
            let arrival = hour * 3600 + min * 60 + sec;

            // Increment the time array for each second in the range
            for j in depature..arrival {
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
        .map(|s| s.parse().unwrap_or(0)) // Use 0 for invalid parts
        .collect();
    (parts.get(0).copied().unwrap_or(0), 
     parts.get(1).copied().unwrap_or(0), 
     parts.get(2).copied().unwrap_or(0))
}
``` 

### Changes Made:
1. **Inclusive Range**: The loop that increments the `time` array should include the `arrival` time. This is because the C code increments from `depature` to `arrival - 1`, which means it should include the `arrival` second as well. 
2. **Parsing Logic**: The parsing logic remains the same, but ensure that the input format is strictly followed.

Here’s the corrected loop for incrementing the time array:

```rust
for j in depature..=arrival { // Use `..=` to include the arrival time
    time[j as usize] += 1;
}
``` 

Make sure to replace the loop in the previous code with this corrected version.
