use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    // Read the number of time entries
    let mut lines = reader.lines();
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();

    for _ in 0..n {
        // Read each time entry in the format "h:m"
        let time_entry = lines.next().unwrap().unwrap();
        let parts: Vec<&str> = time_entry.split(':').collect();
        let h: i32 = parts[0].parse().unwrap();
        let m: i32 = parts[1].parse().unwrap();

        // Calculate the angle between the hour and minute hands
        let angle = (h % 12) as f64 * 360.0 / 12.0 + (m as f64 * 360.0 / 60.0) - (m as f64 * 360.0 / 60.0);
        
        let mut adjusted_angle = angle;
        if adjusted_angle > 180.0 {
            adjusted_angle -= 360.0;
        }

        // Determine the alert level based on the angle
        if adjusted_angle > -30.0 && adjusted_angle < 30.0 {
            println!("alert");
        } else if adjusted_angle > -90.0 && adjusted_angle < 90.0 {
            println!("warning");
        } else {
            println!("safe");
        }
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin` to read input, which is idiomatic for handling standard input in Rust.
2. **Angle Calculation**: The angle calculation follows the same logic as the C code, ensuring that the semantics are preserved.
3. **Output**: The output format matches the C code, ensuring that the newline and spacing are consistent.
4. **Error Handling**: The code assumes valid input as per the specification, without additional error handling for invalid formats or out-of-range values.
5. **Modulus Operation**: The hour is taken modulo 12 to ensure it fits within the 12-hour clock format.
6. **Safety**: The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees.
