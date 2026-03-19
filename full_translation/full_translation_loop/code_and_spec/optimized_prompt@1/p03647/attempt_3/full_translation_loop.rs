use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first input which indicates the number of iterations
    let first_line = lines.next().unwrap().unwrap();
    let n: i32 = first_line.trim().parse().expect("Invalid input for number of iterations");

    // Initialize a pointer-like structure to track the state
    let mut p = 0;
    let mut possible = true;

    // Process pairs of integers
    for _ in 0..n {
        let line = lines.next().unwrap().unwrap();
        let nums: Vec<i32> = line.split_whitespace()
            .map(|s| s.parse().expect("Invalid input for pair of integers"))
            .collect();

        if nums.len() != 2 {
            // If the input does not conform to the expected format, we terminate
            possible = false;
            break;
        }

        let (a, b) = (nums[0], nums[1]);

        // Evaluate the conditions based on the input pairs
        if a != 0 {
            if b != n {
                p = 0; // Reset p if b is not equal to n
            } else {
                p += 1; // Increment p if b equals n
            }
        } else {
            p = b; // Set p to b if a is 0
        }
    }

    // Output the result based on the evaluation
    if p > 0 {
        println!("POSSIBLE");
    } else {
        println!("IMPOSSIBLE");
    }
}
``` 

### Changes Made:
1. **Logic Correction**: The logic for determining the value of `p` was adjusted to ensure it correctly reflects the conditions specified in the original C code and the specification.
2. **Output Condition**: The output condition was kept as `if p > 0` to determine if the state is "POSSIBLE" or "IMPOSSIBLE". This aligns with the expected behavior based on the input pairs. 

Please test this version against the provided input to verify correctness.
