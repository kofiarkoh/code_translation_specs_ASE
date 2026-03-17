use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read M and N
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let m = dimensions.next().unwrap();
    let n = dimensions.next().unwrap();

    let mut total_transitions = 0;

    for i in 0..m {
        let mut a = vec![0; n];

        // Read the row of binary integers
        let row_line = lines.next().unwrap().unwrap();
        for (j, value) in row_line.split_whitespace().enumerate() {
            if j < n { // Ensure we do not exceed the bounds
                a[j] = value.parse::<i32>().unwrap();
                // Modify the input values based on the specified logic
                if i != 0 {
                    a[j] ^= 1; // Flip the value if not the first row
                }
            }
        }

        // Count transitions from 1s to 0s
        let (mut l, mut r) = (0, n as isize - 1);
        while l < n {
            if a[l] == 1 {
                total_transitions += 1;
                if l + 1 < n {
                    a[l + 1] ^= 1; // Flip the next value
                }
                l += 1;
            } else if r >= 0 && a[r as usize] == 1 { // Ensure r is within bounds
                total_transitions += 1;
                a[r as usize] ^= 1; // Flip the current value
                r -= 1;
            } else {
                if l + 1 < n {
                    a[l + 1] ^= 1; // Flip the next value
                }
                l += 1;
            }
        }
    }

    // Output the total count of transitions
    println!("{}", total_transitions);
}
