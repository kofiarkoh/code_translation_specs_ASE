use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read dimensions M and N
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let m = dimensions.next().unwrap(); // Number of rows
    let n = dimensions.next().unwrap(); // Number of columns

    let mut total_transitions = 0;

    for i in 0..m {
        let mut a = vec![0; n];

        // Read the binary values for the current row
        let row_line = lines.next().unwrap().unwrap();
        for (j, value) in row_line.split_whitespace().enumerate() {
            a[j] = value.parse::<i32>().unwrap();
            // Modify the input values based on their positions
            if i != 0 {
                a[j] ^= 1; // Flip the value if not the first row
            }
        }

        // Count transitions from 1s to 0s
        let (mut l, mut r) = (0, n as isize - 1);
        while l <= r {
            if a[l as usize] == 1 {
                total_transitions += 1;
                a[l as usize] ^= 1; // Flip the value at position l
                l += 1; // Move to the next position
            } else if a[r as usize] == 1 {
                total_transitions += 1;
                a[r as usize] ^= 1; // Flip the value at position r
                r -= 1; // Move to the previous position
            } else {
                a[l as usize] ^= 1; // Flip the value at position l
                l += 1; // Move to the next position
            }
        }
    }

    // Output the total count of transitions
    println!("{}", total_transitions);
}
