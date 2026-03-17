use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read M and N
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace();
    let m: usize = dimensions.next().unwrap().parse().unwrap();
    let n: usize = dimensions.next().unwrap().parse().unwrap();

    // Ensure M and N are within valid constraints
    assert!(m > 0 && n > 0 && n <= 106, "Invalid dimensions");

    let mut a = 0;
    let mut A = [0; 106]; // Fixed size array

    for i in 0..m {
        // Read the row values
        let row_line = lines.next().unwrap().unwrap();
        let row_values: Vec<i32> = row_line.split_whitespace()
            .map(|s| s.parse().unwrap())
            .collect();

        for j in 0..n {
            A[j] = row_values[j];
            if i != 0 {
                A[j] ^= 1; // Toggle the value if not the first row
            }
        }

        // Process the row from both ends towards the center
        let (mut l, mut r) = (0, n as isize - 1);
        while l <= r as usize {
            if A[l] == 1 {
                a += 1;
                A[l] ^= 1; // Toggle the left value
                l += 1; // Move left pointer
            } else if A[r as usize] == 1 {
                a += 1;
                A[r as usize] ^= 1; // Toggle the right value
                r -= 1; // Move right pointer
            } else {
                A[l] ^= 1; // Toggle the left value
                l += 1; // Move left pointer
            }
        }
    }

    // Output the total count of operations
    println!("{}", a);
}
