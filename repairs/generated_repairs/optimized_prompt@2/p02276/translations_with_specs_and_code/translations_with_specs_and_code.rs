use std::io::{self, BufRead};

const N: usize = 1_000_000;

fn partition(arr: &mut [i32], p: usize, r: usize) -> usize {
    let x = arr[r];
    let mut i = p as isize - 1;

    for j in p..r {
        if arr[j] <= x {
            i += 1;
            arr.swap(i as usize, j);
        }
    }
    arr.swap((i + 1) as usize, r);
    (i + 1) as usize
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap_or_else(|_| {
        eprintln!("Failed to parse the number of elements.");
        std::process::exit(1);
    });
    
    // Preconditions check
    assert!(n > 0 && n <= N, "n must be a positive integer less than or equal to {}", N);

    let mut arr = vec![0; n]; // Change to n instead of N
    
    // Read the elements into the array
    for i in 0..n {
        arr[i] = lines.next().unwrap().unwrap().parse().unwrap_or_else(|_| {
            eprintln!("Failed to parse element at index {}.", i);
            std::process::exit(1);
        });
    }

    // Perform the partition operation
    let pivot_index = partition(&mut arr, 0, n - 1);

    // Output the result
    for i in 0..n {
        if i == pivot_index {
            print!("[{}]", arr[i]);
        } else {
            print!("{}", arr[i]);
        }
        if i < n - 1 {
            print!(" ");
        }
    }
    println!();
}
