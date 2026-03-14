use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    // Read the number of matrices to process
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();

    for _ in 0..n {
        let mut s = [[0; 12]; 12]; // Primary matrix
        let mut a = [[0; 12]; 12]; // Secondary matrix

        // Read the 10x10 matrix
        for i in 0..10 {
            let line = lines.next().unwrap().unwrap();
            let values: Vec<i32> = line.split_whitespace()
                .map(|x| x.parse().unwrap())
                .collect();
            for j in 0..10 {
                s[i][j] = values[j];
            }
        }

        // Process the matrix
        for i in 0..9 {
            for j in 0..10 {
                if s[i][j] % 2 == 1 {
                    a[i + 1][j] = 1;
                    s[i][j] += 1;
                    s[i + 1][j] += 1;
                    if j > 0 {
                        s[i + 1][j - 1] += 1; // Check bounds for left neighbor
                    }
                    if j < 9 {
                        s[i + 1][j + 1] += 1; // Check bounds for right neighbor
                    }
                    s[i + 2][j] += 1;
                }
            }
        }

        // Output the modified secondary matrix
        for i in 0..10 {
            for j in 0..10 {
                print!("{} ", a[i][j]);
            }
            println!();
        }
    }
}
```

### Notes:
1. **Matrix Size**: The Rust code uses a 12x12 array for both matrices to match the C code's structure, although only a 10x10 section is utilized.
2. **Input Handling**: The input is read using `BufRead` for efficiency, and the parsing is done with error handling in mind.
3. **Bounds Checking**: The Rust code includes bounds checking for accessing neighboring elements in the matrix to prevent out-of-bounds errors, which is a common safety feature in Rust.
4. **Output Formatting**: The output format is preserved exactly as specified, ensuring that each element is followed by a space and each row ends with a newline.
5. **Error Handling**: The code assumes valid input as per the specification, as no error handling was specified in the original C code or the specification.
