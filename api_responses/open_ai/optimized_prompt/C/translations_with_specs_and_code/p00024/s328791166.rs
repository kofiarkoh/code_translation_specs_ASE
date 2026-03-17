use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        match line {
            Ok(input) => {
                // Attempt to parse the input as a floating-point number
                if let Ok(v) = input.trim().parse::<f32>() {
                    // Compute the integer value based on the specified formula
                    let n = ((4.9 * v * v / 9.8 / 9.8) + 5.0) / 5.0;
                    let result = n.ceil() as i32 + 1; // Ensure we round up and add 1
                    println!("{}", result);
                }
                // If parsing fails, we ignore the input and continue
            }
            Err(_) => {
                // Handle any potential errors in reading lines (not specified in the spec)
                continue;
            }
        }
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` to read from standard input, which is idiomatic and safe. The `BufRead` trait allows us to read lines efficiently.
2. **Floating-Point Parsing**: The input is trimmed and parsed as a `f32`, which corresponds to the `float` type in C. If parsing fails, the input is ignored, adhering to the specification.
3. **Computation**: The formula from the C code is preserved. The result is computed using floating-point arithmetic, and `ceil()` is used to ensure we round up before converting to an integer and adding 1.
4. **Output**: The output format is preserved exactly as specified, with each result printed on a new line.
5. **Error Handling**: The program continues reading inputs even if some are invalid, as specified. Any errors in reading lines are ignored, which aligns with the behavior described in the specification.
