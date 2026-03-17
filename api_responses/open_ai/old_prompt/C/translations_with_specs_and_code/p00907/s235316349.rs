// AOJ 1328 Find the Outlier
// Translated from C to Rust

use std::io::{self, BufRead};

fn calc(v: &[f64], n: usize, x: usize, k: usize) -> f64 {
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
        let line = line.unwrap();
        let mut inputs = line.split_whitespace();
        let d: usize = match inputs.next() {
            Some(num) => num.parse().unwrap(),
            None => break,
        };

        if d == 0 {
            break; // Terminate if d is 0
        }

        let n = d + 3;
        let mut v = vec![0.0; n];

        for i in 0..n {
            v[i] = inputs.next().unwrap().parse().unwrap();
        }

        let mut outlier_index = n; // Default to n (last valid index)
        for i in 0..n {
            let mut j = 0;
            loop {
                if j == n {
                    break;
                }
                if j != i {
                    let t = calc(&v, n, i, j) - v[j];
                    if t < -0.1 || t > 0.1 {
                        outlier_index = i; // Found an outlier
                        break;
                    }
                }
                j += 1;
            }
            if outlier_index != n {
                break; // Exit if an outlier is found
            }
        }

        println!("{}", outlier_index);
    }
}
```

### Notes:
- The Rust code preserves the semantics of the original C code while adhering to the provided specification.
- The `calc` function computes the expected value based on the input values, similar to the C implementation.
- Input handling is done using `std::io` to read from standard input, ensuring that the output format matches the specification.
- The loop structure and logic for identifying the outlier are maintained, with careful attention to the conditions specified in the original C code.
- The use of `unwrap()` is safe here because the specification guarantees well-formed input. However, in a production scenario, more robust error handling would be advisable.
