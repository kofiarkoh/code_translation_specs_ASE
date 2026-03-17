use std::io::{self, BufRead};

fn prime_count(n: usize) -> usize {
    if n < 2 {
        return 0;
    }
    let mut count = 0;
    for i in 2..=((n as f64).sqrt() as usize) {
        if n % i == 0 {
            count += 1;
        }
    }
    count // Returns the count of prime factors
}

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let l: i32 = match line {
            Ok(input) => input.trim().parse().unwrap_or(0), // Default to 0 on parse error
            Err(_) => continue, // Skip if there's an error reading the line
        };

        let mut lower = l;
        let mut upper = l;

        // Find the largest prime less than or equal to l
        while prime_count(lower as usize) > 0 {
            lower -= 1;
        }

        // Find the smallest prime greater than or equal to l
        while prime_count(upper as usize) > 0 {
            upper += 1;
        }

        // Output the difference
        println!("{}", upper - lower);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code reads from standard input and handles potential parsing errors by defaulting to `0`. This is a safeguard against invalid input, which aligns with the specification's lack of explicit error handling.
2. **Prime Count Function**: The `prime_count` function counts the number of prime factors of a number, returning `0` for numbers less than `2`, which is consistent with the behavior of the original C code.
3. **Loop Logic**: The loops for finding the nearest primes are preserved, ensuring that the logic remains intact.
4. **Output**: The output format is preserved exactly as specified, printing the difference between the nearest primes on a new line.
5. **Safety**: The code avoids any use of `unsafe` Rust, adhering to the requirement for safe, idiomatic Rust.
