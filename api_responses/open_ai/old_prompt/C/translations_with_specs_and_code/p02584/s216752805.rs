use std::io::{self, BufRead};

fn main() {
    // Read input from stdin
    let stdin = io::stdin();
    let input = stdin.lock().lines().next().unwrap().unwrap();
    let mut iter = input.split_whitespace();

    // Parse the input values
    let x: i64 = iter.next().unwrap().parse().unwrap();
    let k: i64 = iter.next().unwrap().parse().unwrap();
    let d: i64 = iter.next().unwrap().parse().unwrap();

    // Preconditions check
    assert!(k >= 0, "k must be non-negative");
    assert!(d > 0, "d must be positive");

    // Calculate the absolute value of x
    let mut now = x.abs();
    
    // Core logic based on the specification
    if now / d <= k {
        now -= d * (now / d);
        let remaining_k = k - (now / d);
        if remaining_k % 2 != 0 {
            now -= d;
        }
    } else {
        now -= d * k;
    }

    // Output the result
    println!("{}", now.abs());
}
```

### Notes:
1. **Input Handling**: The Rust code reads from standard input and splits the input into three parts, parsing them as `i64` to match the `long long` type in C.
2. **Assertions**: Preconditions are checked using assertions to ensure that `k` is non-negative and `d` is positive, as specified.
3. **Absolute Value**: The absolute value of `x` is calculated using the `abs()` method, which is idiomatic in Rust.
4. **Logic Translation**: The core logic is translated directly from C to Rust, ensuring that the semantics are preserved.
5. **Output**: The result is printed in the same format as specified, ensuring that the output matches the expected behavior.
