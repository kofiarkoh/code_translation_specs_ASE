use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the first line for the number of subsequent integers
    let first_line = lines.next().unwrap().expect("Failed to read line");
    let n: usize = first_line.trim().parse().expect("Input must be a positive integer");

    // Preconditions check
    if n == 0 {
        panic!("The first input must be a positive integer indicating the number of subsequent integers.");
    }

    let mut ans = 0;

    // Read the subsequent integers
    for _ in 0..n {
        let line = lines.next().unwrap().expect("Failed to read line");
        let value: i32 = line.trim().parse().expect("Input must be a valid integer");

        // Each value is decremented by one and added to the answer
        ans += value - 1;
    }

    // Output the result
    println!("{}", ans);
}
```

### Notes:
- The Rust code adheres to the specification by reading the first integer to determine how many subsequent integers to read.
- It checks that the first input is positive and panics if it is not, which aligns with the specification's precondition.
- Each subsequent integer is decremented by one before being added to the total sum, as specified.
- The output format is preserved exactly as in the C code, ensuring that the result is printed in the same manner.
- Error handling is done using `expect` to ensure that invalid inputs lead to a clear panic message, which is a safe way to handle potential input errors in Rust.
