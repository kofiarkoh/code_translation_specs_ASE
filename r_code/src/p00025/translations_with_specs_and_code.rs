use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    let mut input_lines = reader.lines();

    while let (Some(line_a), Some(line_b)) = (input_lines.next(), input_lines.next()) {
        let a: Vec<i32> = line_a
            .unwrap()
            .split_whitespace()
            .take(4)
            .filter_map(|s| s.parse().ok())
            .collect();
        let b: Vec<i32> = line_b
            .unwrap()
            .split_whitespace()
            .take(4)
            .filter_map(|s| s.parse().ok())
            .collect();

        if a.len() == 4 && b.len() == 4 {
            let (hit, blow) = count_hits_and_blows(&a, &b);
            println!("{} {}", hit, blow);
        }
    }
}

fn count_hits_and_blows(a: &[i32; 4], b: &[i32; 4]) -> (i32, i32) {
    let mut hit = 0;
    let mut blow = 0;

    for (i, &value_a) in a.iter().enumerate() {
        for (j, &value_b) in b.iter().enumerate() {
            if value_a == value_b {
                if i == j {
                    hit += 1;
                } else {
                    blow += 1;
                }
            }
        }
    }

    (hit, blow)
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `BufRead` to read lines from standard input, which is idiomatic for handling input in Rust. It processes pairs of lines until the end of input is reached.
  
2. **Data Structures**: The C code uses arrays of fixed size (4). In Rust, I used slices (`&[i32; 4]`) to maintain the same structure and ensure that we only work with exactly four integers.

3. **Counting Logic**: The logic for counting hits and blows is preserved in the `count_hits_and_blows` function, which mirrors the nested loop structure of the C code.

4. **Error Handling**: The Rust code assumes valid input as per the specification, similar to the C code. It does not handle invalid input formats or values, consistent with the preconditions.

5. **Output Formatting**: The output format is preserved exactly as specified, ensuring that the counts of hits and blows are printed in the required format.

6. **Safety**: The translation avoids any use of `unsafe` code, adhering to Rust's safety guarantees.
