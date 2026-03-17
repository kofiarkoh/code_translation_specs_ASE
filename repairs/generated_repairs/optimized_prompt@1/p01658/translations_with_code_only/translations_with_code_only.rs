use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read M and N
    let first_line = lines.next().unwrap().unwrap();
    let mut nums = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let m = nums.next().unwrap();
    let n = nums.next().unwrap();

    let mut a = 0;
    let mut A = vec![0; n];

    for i in 0..m {
        // Read the array A
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();

        for j in 0..n {
            A[j] = values[j];
            if i != 0 {
                A[j] ^= 1; // Flip bits for subsequent rows
            }
        }

        let (mut l, mut r) = (0, n as isize - 1);
        while l <= r {
            if A[l as usize] == 1 {
                a += 1;
                A[l as usize] ^= 1; // Flip the bit at l
                l += 1; // Move l to the right
            } else if A[r as usize] == 1 {
                a += 1;
                A[r as usize] ^= 1; // Flip the bit at r
                r -= 1; // Move r to the left
            } else {
                // If both ends are 0, we should not flip A[l]
                l += 1; // Move l to the right
            }
        }
    }

    // Print the result
    println!("{}", a);
}
