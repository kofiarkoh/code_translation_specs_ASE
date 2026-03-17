use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        let line = line.unwrap();
        let mut iter = line.split_whitespace();
        let w: usize = iter.next().unwrap().parse().unwrap();
        let d: usize = iter.next().unwrap().parse().unwrap();

        if w == 0 {
            break; // Termination condition
        }

        let mut a = [0; 21]; // Array for group 1
        let mut b = [0; 21]; // Array for group 2

        // Read items for group 1
        for _ in 0..w {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            a[t] += 1;
        }

        // Read items for group 2
        for _ in 0..d {
            let t: usize = reader.lines().next().unwrap().unwrap().parse().unwrap();
            b[t] += 1;
        }

        // Calculate the weighted sum
        let mut res = 0;
        for i in 1..=20 {
            res += i * std::cmp::max(a[i], b[i]);
        }

        // Output the result
        println!("{}", res);
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `BufRead` to read input efficiently, which is idiomatic for handling multiple lines of input.
2. **Array Initialization**: The arrays `a` and `b` are initialized to zero, as specified.
3. **Termination Condition**: The loop breaks when `w` is zero, adhering to the specification.
4. **Type Safety**: The code uses `usize` for array indexing and item counts, which is safe and idiomatic in Rust.
5. **Output**: The output format is preserved exactly as specified, ensuring a single integer followed by a newline.
6. **Error Handling**: The code assumes valid input as per the specification, similar to the C code. No additional error handling is implemented since the spec does not require it.
