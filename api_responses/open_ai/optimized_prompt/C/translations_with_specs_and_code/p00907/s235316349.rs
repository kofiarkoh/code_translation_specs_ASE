// AOJ 1328 Find the Outlier
// Translated from C to Rust

use std::io::{self, BufRead};

const MAX_SIZE: usize = 10;

fn calc(n: usize, v: &[f64], x: usize, k: usize) -> f64 {
    let mut s = 0.0;

    for i in 0..n {
        if i != k && i != x {
            let mut t = v[i];
            for j in 0..n {
                if j != i && j != k && j != x {
                    t = (t * (k as f64 - j as f64)) / (i as f64 - j as f64);
                }
            }
            s += t;
        }
    }
    s
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut values: Vec<f64> = line.split_whitespace()
            .filter_map(|s| s.parse::<f64>().ok())
            .collect();

        if values.is_empty() {
            continue; // Skip empty lines
        }

        let d = values.remove(0) as usize; // Read the first value as the count
        if d == 0 || d + 3 > MAX_SIZE {
            continue; // Invalid input based on the specification
        }

        // Ensure we have enough values
        if values.len() < d + 3 {
            continue; // Not enough values
        }

        let n = d + 3; // Total number of values
        let mut outlier_index = n; // Default to the last index

        for i in 0..n {
            let mut found_outlier = false;

            for j in 0..n {
                if j != i {
                    let t = calc(n, &values, i, j) - values[j];
                    if t < -0.1 || t > 0.1 {
                        found_outlier = true;
                        break;
                    }
                }
            }

            if found_outlier {
                outlier_index = i;
                break;
            }
        }

        println!("{}", outlier_index);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code reads input using `io::stdin()` and processes it line by line, similar to the C code's `scanf`. It filters and parses the input into a vector of floating-point numbers.
2. **Array Size**: The maximum size of the array is defined as a constant (`MAX_SIZE`), ensuring that we adhere to the specification regarding the number of values.
3. **Outlier Calculation**: The `calc` function is translated directly from C, maintaining the same logic for calculating the interpolation.
4. **Output**: The output format is preserved, printing the index of the outlier or the last index as specified.
5. **Error Handling**: The code checks for invalid input conditions (like insufficient values) and skips processing in such cases, as per the specification.
