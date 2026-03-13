use std::io::{self, Read};

fn main() {
    // Mirror the C program's types/initial values closely.
    // In the original C, `a` is an `int` initialized to 100000.
    // We'll use i32 to match typical 32-bit `int` behavior.
    let mut a: i32 = 100000;

    // Read a single integer `n` from stdin (like scanf("%d",&n)).
    // If parsing fails (no token), we simply return early; typical
    // judge inputs will provide a valid integer.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        return;
    }
    let n_opt = input.split_whitespace().next().and_then(|s| s.parse::<i32>().ok());
    let n = match n_opt {
        Some(v) => v,
        None => return,
    };

    // The C code does: for(i=0;i<n;i++) { ... }
    // That runs zero times for n <= 0. Emulate that behavior.
    if n > 0 {
        for _ in 0..(n as usize) {
            // The original C computed: a = a + a * 0.05; where 0.05 is a double.
            // That produces a double which is then assigned to int (truncation toward zero).
            // For positive `a`, this is equivalent to floor(1.05 * a).
            // We emulate it by doing the floating computation in f64 then casting to i32,
            // which matches the C approach for typical input sizes.
            let tmp = (a as f64) + (a as f64) * 0.05_f64;
            a = tmp as i32;

            // Then the C code rounds up to the next multiple of 1000 if not already one.
            if a % 1000 != 0 {
                a = (a / 1000 + 1) * 1000;
            }
        }
    }

    // Preserve output formatting: print the integer and a newline.
    println!("{}", a);
}
