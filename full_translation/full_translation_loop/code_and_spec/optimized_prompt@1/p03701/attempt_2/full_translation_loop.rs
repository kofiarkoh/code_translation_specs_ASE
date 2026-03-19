use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    // Read the number of integers
    let mut lines = handle.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    // Preconditions: n must be positive
    if n == 0 {
        return; // Undefined behavior as per spec
    }

    let mut sum = 0;
    let mut min = 101; // Initialize min to a value greater than any valid input
    let mut modified_values = Vec::with_capacity(n);

    for _ in 0..n {
        let value: i32 = lines.next().unwrap().unwrap().parse().unwrap();
        sum += value;

        // Modify value if divisible by 10
        if value % 10 == 0 {
            modified_values.push(101);
        } else {
            modified_values.push(value);
        }

        // Update minimum value
        let last_value = *modified_values.last().unwrap(); // Dereference to get the value
        if min > last_value {
            min = last_value;
        }
    }

    let mut ans = sum;

    // Postcondition checks
    if sum % 10 == 0 {
        if min == 101 {
            ans = 0;
        } else {
            ans = sum - min;
        }
    }

    // Output the result
    println!("{}", ans);
}
