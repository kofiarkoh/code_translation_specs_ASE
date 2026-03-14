use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let mut x: f64 = 0.0; // Cumulative x coordinate
    let mut y: f64 = 0.0; // Cumulative y coordinate

    for line in stdin.lock().lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();

        if parts.len() != 2 {
            continue; // Ignore invalid input
        }

        let distance: f64 = match parts[0].trim().parse() {
            Ok(d) => d,
            Err(_) => continue, // Ignore invalid distance
        };

        let angle: f64 = match parts[1].trim().parse() {
            Ok(a) => a,
            Err(_) => continue, // Ignore invalid angle
        };

        // Termination condition
        if distance == 0.0 && angle == 0.0 {
            break;
        }

        // Update coordinates based on distance and angle
        let radians = angle.to_radians(); // Convert angle to radians
        x += distance * radians.cos(); // Update x coordinate
        y += distance * radians.sin(); // Update y coordinate
    }

    // Output the final coordinates, rounded down to the nearest integer
    println!("{},{}", x.floor() as i32, y.floor() as i32);
}
```

### Explanation of Choices:
1. **Input Handling**: The program reads from standard input and expects pairs of values separated by a comma. It ignores any invalid input lines.
2. **Termination Condition**: The loop breaks when both distance and angle are zero, as specified.
3. **Coordinate Calculation**: The coordinates are updated using trigonometric functions based on the angle converted to radians.
4. **Output**: The final coordinates are rounded down using `floor()` and cast to `i32` to match the requirement for integer output.
5. **Error Handling**: The program gracefully handles invalid input by skipping those lines without crashing.
