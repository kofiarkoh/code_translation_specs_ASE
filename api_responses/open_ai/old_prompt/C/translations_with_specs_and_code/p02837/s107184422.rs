use std::io::{self, BufRead};

fn counter(z: u32) -> u32 {
    if z == 0 {
        0
    } else {
        counter(z >> 1) + (z & 1)
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of conditions
    let n: usize = lines.next().unwrap().unwrap().parse().unwrap();
    let mut max = 0;

    // Initialize arrays
    let mut a = vec![0; 20];
    let mut x = vec![vec![0; 20]; 20];
    let mut y = vec![vec![0; 20]; 20];

    // Read conditions and constraints
    for i in 0..n {
        let a_i: usize = lines.next().unwrap().unwrap().parse().unwrap();
        a[i] = a_i;
        for j in 0..a_i {
            let line = lines.next().unwrap().unwrap();
            let mut parts = line.split_whitespace();
            let x_ij: usize = parts.next().unwrap().parse().unwrap();
            let y_ij: u32 = parts.next().unwrap().parse().unwrap();
            x[i][j] = x_ij;
            y[i][j] = y_ij;
        }
    }

    // Evaluate all subsets of conditions
    for i in 0..(1 << n) {
        let mut flag = true;
        for j in 0..n {
            if (i & (1 << j)) == 0 {
                continue;
            }
            for k in 0..a[j] {
                if ((i >> (x[j][k] - 1)) & 1) ^ y[j][k] != 0 {
                    flag = false;
                    break;
                }
            }
            if !flag {
                break;
            }
        }
        // If there were no contradictions
        if flag {
            let count = counter(i);
            if max < count {
                max = count;
            }
        }
    }

    // Output the maximum count of valid bits set to 1
    println!("{}", max);
}
```

### Notes:
- The Rust code preserves the semantics of the original C code while adhering to the provided specification.
- The `counter` function is implemented recursively, similar to the C version.
- Input handling is done using `std::io` to read from standard input, ensuring that the format matches the expected behavior.
- The arrays are initialized using `Vec`, which is idiomatic in Rust for dynamic sizing.
- The logic for evaluating subsets and checking constraints is directly translated from C to Rust, maintaining the same structure and flow.
- The output is formatted to match the specification, ensuring that it prints a single integer as required.
