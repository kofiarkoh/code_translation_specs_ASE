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
    let mut a_vec = vec![0; n]; // Equivalent to A[106] in C, but we will use only n elements

    for i in 0..m {
        // Read the array A
        let line = lines.next().unwrap().unwrap();
        let values: Vec<i32> = line.split_whitespace().map(|s| s.parse::<i32>().unwrap()).collect();
        
        for j in 0..n {
            a_vec[j] = values[j];
            if i != 0 {
                a_vec[j] ^= 1; // Flip bits if not the first row
            }
        }

        let (mut l, mut r) = (0, n as isize - 1); // Use isize for indexing
        while l <= r {
            if a_vec[l] == 1 {
                a += 1;
                a_vec[l] ^= 1; // Flip the bit at l
                l += 1; // Move l to the right
            } else if a_vec[r as usize] == 1 {
                a += 1;
                a_vec[r as usize] ^= 1; // Flip the bit at r
                r -= 1; // Move r to the left
            } else {
                a_vec[l] ^= 1; // Flip the bit at l
                l += 1; // Move l to the right
            }
        }
    }

    println!("{}", a);
}
