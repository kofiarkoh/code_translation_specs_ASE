use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of line segment pairs
    let n: usize = lines.next().unwrap().unwrap().trim().parse().expect("Invalid input");

    for _ in 0..n {
        // Read the coordinates of the endpoints of the two line segments
        let line = lines.next().unwrap().unwrap();
        let coords: Vec<f64> = line
            .split_whitespace()
            .map(|s| s.parse().expect("Invalid coordinate"))
            .collect();

        let (x1, y1, x2, y2, x3, y3, x4, y4) = (
            coords[0], coords[1], coords[2], coords[3],
            coords[4], coords[5], coords[6], coords[7],
        );

        // Check for vertical lines (undefined slope)
        if (x2 - x1).abs() < f64::EPSILON || (x4 - x3).abs() < f64::EPSILON {
            // If either line segment is vertical, we cannot determine parallelism
            // according to the specification, we do not handle this case explicitly.
            // Thus, we will skip this iteration.
            continue;
        }

        // Calculate slopes
        let sl1 = (y2 - y1) / (x2 - x1);
        let sl2 = (y4 - y3) / (x4 - x3);

        // Compare slopes
        if (sl1 - sl2).abs() < f64::EPSILON {
            println!("YES");
        } else {
            println!("NO");
        }
    }
}
```

### Notes:
1. **Input Handling**: The Rust code reads input using `std::io`, which is idiomatic for handling standard input in Rust.
2. **Slope Calculation**: The slopes are calculated only if the x-coordinates are distinct, as per the specification. If either line segment is vertical (i.e., the x-coordinates are equal), the program skips the iteration without outputting anything, which aligns with the specification's note on undefined behavior.
3. **Floating-Point Comparison**: The comparison of slopes uses a small epsilon value to account for floating-point precision issues, which is a common practice in Rust.
4. **Error Handling**: The code assumes valid input as per the specification and does not handle errors explicitly beyond parsing.
