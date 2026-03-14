use std::io::{self, BufRead};

fn gcd(a: u32, b: u32) -> u32 {
    let mut a = a;
    let mut b = b;
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        if let Ok(line) = line {
            let nums: Vec<u32> = line.split_whitespace()
                .filter_map(|s| s.parse().ok())
                .collect();

            if nums.len() == 2 {
                let a = nums[0];
                let b = nums[1];

                // Compute GCD
                let gcd_value = gcd(a, b);
                // Compute product divided by GCD
                let product_divided_by_gcd = (a / gcd_value) * (b / gcd_value);

                // Output the results
                println!("{} {}", gcd_value, product_divided_by_gcd);
            }
        }
    }
}
```

### Explanation of Translation Choices:
1. **GCD Calculation**: The GCD is computed using a simple iterative method, which is safe and idiomatic in Rust. This preserves the semantics of the original C code.
2. **Input Handling**: The program reads from standard input until EOF, processing each line. It splits the line into words, attempts to parse them as `u32`, and collects them into a vector. This ensures that only valid non-negative integers are processed, adhering to the preconditions.
3. **Output Formatting**: The output format is preserved exactly as specified, ensuring that the results are printed in the required format.
4. **Error Handling**: The program does not explicitly handle errors related to input format, as specified. It simply ignores lines that do not contain exactly two valid integers.
5. **Safety**: The code avoids any use of `unsafe` blocks, ensuring that it remains safe and idiomatic. The use of `u32` guarantees non-negative integers, aligning with the specification's preconditions.
