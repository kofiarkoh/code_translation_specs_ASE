use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read M and N
    let first_line = lines.next().unwrap().unwrap();
    let mut dimensions = first_line.split_whitespace()
        .map(|s| s.parse::<usize>().unwrap());
    let m = dimensions.next().unwrap();
    let n = dimensions.next().unwrap();

    // Ensure M and N are within valid constraints
    assert!(m > 0 && n > 0 && n <= 106, "Invalid dimensions");

    let mut a = 0;
    let mut A = [0; 106]; // Fixed size array

    for i in 0..m {
        // Read the row values
        let row_line = lines.next().unwrap().unwrap();
        let row_values: Vec<usize> = row_line.split_whitespace()
            .map(|s| s.parse::<usize>().unwrap())
            .collect();

        // Ensure the row has the correct number of columns
        assert!(row_values.len() == n, "Row does not have the correct number of columns");

        for j in 0..n {
            A[j] = row_values[j];
            if i != 0 {
                A[j] ^= 1; // Toggle the value if not the first row
            }
        }

        // Process the row from both ends towards the center
        let (mut l, mut r) = (0, n as isize - 1);
        while l <= r {
            if A[l as usize] == 1 {
                a += 1;
                A[l as usize] ^= 1; // Toggle the value
                l += 1;
            } else if A[r as usize] == 1 {
                a += 1;
                A[r as usize] ^= 1; // Toggle the value
                r -= 1;
            } else {
                A[l as usize] ^= 1; // Toggle the value
                l += 1;
            }
        }
    }

    // Output the total count of operations
    println!("{}", a);
}
