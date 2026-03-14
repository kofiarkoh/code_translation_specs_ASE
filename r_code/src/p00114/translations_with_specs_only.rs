use std::io::{self, BufRead};

fn gcd(a: i64, b: i64) -> i64 {
    if b == 0 {
        a
    } else {
        gcd(b, a % b)
    }
}

fn lcm(a: i64, b: i64) -> i64 {
    (a / gcd(a, b)) * b
}

fn cycle_count(n: i64, base: i64) -> i64 {
    let mut count = 0;
    let mut current = n;

    // Count cycles until we return to 1 under modular arithmetic
    while current != 1 {
        current = current % base;
        if current == 0 {
            current = base; // Wrap around to base if current is 0
        }
        count += 1;
    }
    count
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let nums: Vec<i64> = line.split_whitespace()
            .filter_map(|s| s.parse().ok())
            .collect();

        if nums.len() != 6 {
            continue; // Skip lines that do not have exactly 6 integers
        }

        let (a1, b1, a2, b2, a3, b3) = (nums[0], nums[1], nums[2], nums[3], nums[4], nums[5]);

        // Termination condition: all inputs are zero
        if a1 == 0 && b1 == 0 && a2 == 0 && b2 == 0 && a3 == 0 && b3 == 0 {
            break;
        }

        // Calculate cycle counts
        let count1 = cycle_count(a1, b1);
        let count2 = cycle_count(a2, b2);
        let count3 = cycle_count(a3, b3);

        // Calculate LCM of the cycle counts
        let result = lcm(lcm(count1, count2), count3);
        println!("{}", result);
    }
}
```

### Explanation of Choices:
1. **Cycle Count Calculation**: The `cycle_count` function computes the number of iterations until the integer returns to 1 under modular arithmetic. It handles the case where the current value becomes 0 by wrapping it to the base.
  
2. **LCM Calculation**: The `lcm` function uses the GCD to compute the least common multiple, ensuring that the calculations remain within the bounds of 64-bit signed integers.

3. **Input Handling**: The program reads lines of input, splits them into integers, and checks for the termination condition (all zeros). It skips lines that do not contain exactly six integers.

4. **Graceful Termination**: The program exits the loop when all input values are zero, as specified.

5. **No Unsafe Code**: The implementation avoids `unsafe` Rust, adhering to safe programming practices while maintaining the required functionality.
