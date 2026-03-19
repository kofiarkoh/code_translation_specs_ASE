fn main() {
    use std::io::{self, BufRead};

    // Read input from stdin
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line to get the number of elements
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Failed to parse the number of elements");
    
    // Read the sequence of integers
    let mut a: Vec<i32> = Vec::new();
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Failed to parse an integer");
        a.push(value);
    }

    // Check if the sequence can be transformed into non-increasing order
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
