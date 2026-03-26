use std::io::{self, BufRead};

const MAX: usize = 13;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut numline = [0; MAX];
    let mut tmpline = [0; MAX];
    let mut num = [0; 101];

    for line in reader.lines() {
        let n: i32 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n as usize {
            let value: i32 = reader.lines().next().unwrap().unwrap().trim().parse().unwrap();
            numline[i] = value;
        }

        let mut cnt = 0;
        let mut flag = true;

        while flag {
            // Reset num array
            for i in 0..101 {
                num[i] = 0;
            }

            // Count occurrences
            for &value in &numline[..n as usize] {
                num[value as usize] += 1;
            }

            // Fill tmpline with counts
            for i in 0..n as usize {
                tmpline[i] = num[numline[i] as usize];
            }

            cnt += 1;
            flag = false;

            // Check if tmpline matches numline
            for i in 0..n as usize {
                if tmpline[i] == numline[i] {
                    flag = true;
                }
            }

            if flag && (flag as usize) == n as usize {
                flag = false;
            } else {
                // Update numline with tmpline
                for i in 0..n as usize {
                    numline[i] = tmpline[i];
                }
                flag = true;
            }
        }

        println!("{}", cnt - 1);
        for i in 0..n as usize {
            if i == (n as usize - 1) {
                println!("{}", tmpline[i]);
            } else {
                print!("{} ", tmpline[i]);
            }
        }
    }
}
``` 

### Note:
The previous code had an issue with moving the `reader` when calling `lines()`. To fix this, we need to read all input lines in a single loop instead of calling `lines()` multiple times. Here’s the corrected version:

```rust
use std::io::{self, BufRead};

const MAX: usize = 13;

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let lines: Vec<String> = reader.lines().map(|line| line.unwrap()).collect();

    let mut numline = [0; MAX];
    let mut tmpline = [0; MAX];
    let mut num = [0; 101];

    let mut line_iter = lines.iter();
    while let Some(line) = line_iter.next() {
        let n: i32 = line.trim().parse().unwrap();
        if n == 0 {
            break;
        }

        for i in 0..n as usize {
            let value: i32 = line_iter.next().unwrap().trim().parse().unwrap();
            numline[i] = value;
        }

        let mut cnt = 0;
        let mut flag = true;

        while flag {
            // Reset num array
            for i in 0..101 {
                num[i] = 0;
            }

            // Count occurrences
            for &value in &numline[..n as usize] {
                num[value as usize] += 1;
            }

            // Fill tmpline with counts
            for i in 0..n as usize {
                tmpline[i] = num[numline[i] as usize];
            }

            cnt += 1;
            flag = false;

            // Check if tmpline matches numline
            for i in 0..n as usize {
                if tmpline[i] == numline[i] {
                    flag = true;
                }
            }

            if flag && (flag as usize) == n as usize {
                flag = false;
            } else {
                // Update numline with tmpline
                for i in 0..n as usize {
                    numline[i] = tmpline[i];
                }
                flag = true;
            }
        }

        println!("{}", cnt - 1);
        for i in 0..n as usize {
            if i == (n as usize - 1) {
                println!("{}", tmpline[i]);
            } else {
                print!("{} ", tmpline[i]);
            }
        }
    }
}
``` 

This version collects all input lines first, allowing us to iterate through them without moving the `reader`.
