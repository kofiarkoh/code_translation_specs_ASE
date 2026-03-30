use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut input = stdin.lock().lines();
    
    while let Some(line) = input.next() {
        let n: usize = line.unwrap().trim().parse().expect("Invalid input");
        if n == 0 {
            break;
        }

        let mut a = Vec::with_capacity(n);
        for _ in 0..n {
            let num: i32 = input.next().unwrap().unwrap().trim().parse().expect("Invalid input");
            a.push(num);
        }

        // Sort the array
        a.sort_unstable();

        let mut max_length = 0;
        let mut max_value = 0;
        let mut i = 0;

        while i < n {
            let current_value = a[i];
            let mut length = 1;

            // Count the length of the contiguous segment
            while i + 1 < n && a[i + 1] == current_value {
                length += 1;
                i += 1;
            }

            // Update max_length and max_value if needed
            if length > max_length {
                max_length = length;
                max_value = current_value;
            }

            i += 1; // Move to the next distinct value
        }

        // Check if the longest segment exceeds half the total number of integers
        if max_length > n / 2 {
            println!("{}", max_value);
        } else {
            println!("NO COLOR");
        }
    }
}
``` 

### Changes Made:
1. **Error Handling**: The code now uses `expect` to handle potential parsing errors, which will provide clearer error messages if invalid input is encountered.
2. **Input Handling**: The input reading logic remains unchanged, but ensure that the input format strictly adheres to the specification to avoid runtime errors. 

Make sure to test the code with valid input formats as specified in the original specification.
