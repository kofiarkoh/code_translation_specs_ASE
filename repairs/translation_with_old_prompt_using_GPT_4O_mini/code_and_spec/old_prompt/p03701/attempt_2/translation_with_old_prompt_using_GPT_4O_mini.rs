use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of integers
    let n: usize = lines.next().unwrap().unwrap().parse().expect("Invalid input for n");
    
    // Ensure n is greater than 0
    assert!(n > 0, "n must be greater than 0");

    let mut s = Vec::with_capacity(n);
    let mut k = 0;
    let mut min = 101;

    // Read the integers and compute the sum and minimum
    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().expect("Invalid input for integer");
        
        // Ensure the value is within the specified range
        assert!(value >= 0 && value <= 100, "Integer out of range");

        k += value;

        // Adjust the value if it's a multiple of ten
        if value % 10 == 0 {
            // Set to 101 to indicate it's a multiple of ten
            s.push(101);
        } else {
            s.push(value);
        }

        // Update the minimum value
        if min > *s.last().unwrap() {
            min = *s.last().unwrap();
        }
    }

    let mut ans = k;

    // Modify the answer based on the conditions specified
    if k % 10 == 0 {
        if min == 101 {
            ans = 0;
        } else {
            ans = k - min;
        }
    }

    // Print the result
    println!("{}", ans);
}
