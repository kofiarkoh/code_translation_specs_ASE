use std::io::{self, Write};

const N: usize = 1_000_000;
static mut A: [i32; N] = [0; N]; // Using unsafe to allow mutable static array

fn partition(p: usize, r: usize) -> usize {
    unsafe {
        let x = A[r];
        let mut i = p as isize - 1; // Use isize for compatibility with index manipulation
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
    let mut input = String::new();
    io::stdin().read_line(&mut input).unwrap();
    let n: usize = input.trim().parse().expect("Failed to parse n");

    unsafe {
        for i in 1..=n {
            input.clear();
            io::stdin().read_line(&mut input).unwrap();
            A[i] = input.trim().parse().expect("Failed to parse A[i]");
        }
    }

    let x = partition(1, n);

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
