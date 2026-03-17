use std::io::{self, BufRead};

const N: usize = 1_000_000;
static mut A: [i32; N] = [0; N]; // Using unsafe to allow mutable static array

fn partition(p: usize, r: usize) -> usize {
    unsafe {
        let x = A[r];
        let mut i = p as isize - 1; // Use isize for compatibility with negative indexing
        for j in p..r {
            if A[j] <= x {
                i += 1;
                A.swap(i as usize, j); // Swap elements
            }
        }
        A.swap((i + 1) as usize, r); // Final swap
        (i + 1) as usize // Return the partition index
    }
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read input into the array
    for i in 1..=n {
        let value: i32 = lines.next().unwrap().unwrap().trim().parse().unwrap();
        unsafe {
            A[i] = value;
        }
    }

    let x = partition(1, n);
    
    // Output the array with the partition index highlighted
    for i in 1..=n {
        if i == x {
            print!("[{}]", unsafe { A[i] });
        } else {
            print!("{}", unsafe { A[i] });
        }
        if i < n {
            print!(" ");
        }
    }
    println!(); // Print newline at the end
}
