use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut coords: Vec<f64> = line
            .split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        // Ensure we have exactly 8 coordinates
        if coords.len() != 8 {
            continue; // Skip invalid input
        }

        let (x0, y0, x1, y1, x2, y2, x3, y3) = (coords[0], coords[1], coords[2], coords[3], coords[4], coords[5], coords[6], coords[7]);

        if (x0 <= x2 && x1 >= x2) || (x0 <= x3 && x1 >= x3) {
            if (y0 <= y2 && y1 >= y2) || (y0 <= y3 && y1 >= y3) {
                println!("YES");
                continue;
            }
        }
        println!("NO");
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` to read from standard input, which is idiomatic for handling input in Rust. It reads lines and splits them into whitespace-separated values, parsing them into `f64` (double-precision floating-point) values.

2. **Error Handling**: The code checks if exactly 8 coordinates are provided. If not, it skips the line, which aligns with the specification that assumes valid input but does not handle invalid formats explicitly.

3. **Logic Preservation**: The logic for checking overlaps is preserved from the C code, ensuring that the conditions for outputting "YES" or "NO" are maintained.

4. **Output Formatting**: The output is formatted exactly as specified, with "YES" or "NO" printed based on the overlap conditions.

5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees. 

This translation maintains the semantics of the original C code while adhering to Rust's idiomatic practices.
