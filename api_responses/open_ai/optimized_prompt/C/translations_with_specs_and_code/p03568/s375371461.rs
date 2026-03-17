use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut lines = stdin.lock().lines();

    // Read the first line for the count of integers
    let first_line = lines.next().expect("No input provided").expect("Failed to read line");
    let n: usize = first_line.trim().parse().expect("First input must be a non-negative integer");

    let mut even_count = 0;

    // Read the subsequent integers
    for _ in 0..n {
        let line = lines.next().expect("Not enough integers provided").expect("Failed to read line");
        let a: i32 = line.trim().parse().expect("Input must be an integer");
        if a % 2 == 0 {
            even_count += 1;
        }
    }

    // Calculate 3^N
    let exp = 3u64.pow(n as u32);
    // Calculate 2^(count of even integers)
    let odd = 2u64.pow(even_count as u32);

    // Output the result
    println!("{}", exp - odd);
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` to read input line by line, which is idiomatic for handling standard input in Rust.
2. **Type Safety**: The code uses `usize` for `n` to ensure it is non-negative, as specified. The count of even integers is stored in a `u64` to handle larger powers without overflow.
3. **Power Calculation**: The `pow` method is used for calculating powers of 3 and 2, which is safe and avoids the need for manual loops.
4. **Error Handling**: The code checks for input validity and provides clear error messages if the input does not meet the specifications.
5. **Output**: The output format is preserved exactly as specified, ensuring that the result is printed as a single integer.
