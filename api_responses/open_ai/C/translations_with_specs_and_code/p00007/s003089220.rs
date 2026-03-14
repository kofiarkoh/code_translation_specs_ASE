use std::io;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    // Parse the input as a non-negative integer
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    let mut a: f64 = 100000.0; // Start with the initial amount as a floating-point number

    for _ in 0..n {
        a += a * 0.05; // Increase by 5%

        // Round up to the nearest thousand if not already a multiple of 1000
        if a % 1000.0 != 0.0 {
            a = (a / 1000.0).ceil() * 1000.0;
        }
    }

    // Convert the final amount to an integer and print
    println!("{}", a as i32);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The input is read as a string and parsed into a `usize`, ensuring it is a non-negative integer as per the specification.
2. **Floating-Point Arithmetic**: The initial amount `a` is treated as a `f64` to handle the percentage increase accurately.
3. **Rounding Logic**: The rounding logic is implemented using `ceil()` to ensure it rounds up to the nearest thousand.
4. **Output**: The final amount is cast to `i32` for output, which aligns with the expected integer output format.
5. **Safety**: The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees.
