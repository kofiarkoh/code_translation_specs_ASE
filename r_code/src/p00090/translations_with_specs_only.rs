use std::io::{self, BufRead};

#[derive(Debug)]
struct Circle {
    x: f64,
    y: f64,
    radius: f64,
}

impl Circle {
    fn overlaps(&self, other: &Circle) -> bool {
        let distance_squared = (self.x - other.x).powi(2) + (self.y - other.y).powi(2);
        let radius_sum = self.radius + other.radius;
        distance_squared < radius_sum.powi(2)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut parts = line.split_whitespace();
        
        // Read the number of circles
        let num_circles: usize = match parts.next() {
            Some(num) => num.parse().unwrap_or(0),
            None => continue,
        };

        // Terminate on zero input
        if num_circles == 0 {
            break;
        }

        let mut circles = Vec::new();

        // Read circle data
        for _ in 0..num_circles {
            let x: f64 = parts.next().unwrap().parse().unwrap();
            let y: f64 = parts.next().unwrap().parse().unwrap();
            let radius: f64 = 1.0; // Assuming a default radius of 1.0 as it is not specified

            circles.push(Circle { x, y, radius });
        }

        // Calculate maximum overlap
        let max_overlap = calculate_max_overlap(&circles);
        println!("{}", max_overlap + 1); // Increment by one as per specification
    }
}

fn calculate_max_overlap(circles: &[Circle]) -> usize {
    let mut max_overlap = 0;

    for i in 0..circles.len() {
        let mut overlap_count = 0;

        for j in 0..circles.len() {
            if i != j && circles[i].overlaps(&circles[j]) {
                overlap_count += 1;
            }
        }

        max_overlap = max_overlap.max(overlap_count);
    }

    max_overlap
}
```

### Explanation of Choices:
1. **Circle Structure**: A `Circle` struct is defined to hold the coordinates and radius. The radius is assumed to be 1.0 since it was not specified in the input.
2. **Overlap Calculation**: The `overlaps` method checks if two circles overlap based on their coordinates and radius.
3. **Input Handling**: The program reads input until a zero is encountered, handling multiple sets of circles.
4. **Output**: The maximum overlap count is incremented by one before printing, as specified.
5. **Error Handling**: The program assumes valid input as per the specification and does not handle invalid cases explicitly. It uses `unwrap()` for simplicity, which will panic on invalid input, consistent with the specification's lack of error handling requirements.
