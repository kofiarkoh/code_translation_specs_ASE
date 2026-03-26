use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();
    
    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    
    // Read the array elements
    let mut a: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();
    
    // Check if the array can be transformed into non-decreasing order
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] -= 1; // Decrement the previous element
        }
        if a[i] < a[i - 1] {
            println!("No");
            return;
        }
    }
    
    println!("Yes");
}
