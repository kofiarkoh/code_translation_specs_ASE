use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    
    for line in reader.lines() {
        let line = line.unwrap();
        let mut nums = line.split_whitespace()
            .filter_map(|s| s.parse::<i32>().ok());
        
        let n = match nums.next() {
            Some(num) if num > 0 => num,
            _ => break, // Terminate if the first integer is zero or invalid
        };
        
        let mut p = match nums.next() {
            Some(num) => num,
            _ => continue, // Skip if the second integer is missing
        };

        let mut i = 0;
        let mut w = p;
        let mut c = vec![0; 50]; // Array to hold counts of eliminations

        loop {
            if w > 0 {
                w -= 1;
                c[i as usize % n as usize] += 1;

                if w == 0 {
                    let mut found = false;
                    for j in 0..n {
                        if j != i % n && c[j as usize] > 0 {
                            found = true;
                            break;
                        }
                    }
                    if !found {
                        println!("{}", i % n);
                        break;
                    }
                }
            } else {
                w += c[i as usize % n as usize];
                c[i as usize % n as usize] = 0;
            }
            i += 1;
        }
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` for reading input, which is idiomatic and safe. It reads lines and splits them into integers, handling potential parsing errors gracefully.
  
2. **Loop and Logic**: The main logic of the C code is preserved in the Rust loop structure. The use of `usize` for indexing is appropriate since Rust does not allow negative indices.

3. **Array Initialization**: The `c` array is initialized as a `Vec<i32>` to hold counts of eliminations, which is more idiomatic in Rust than using a fixed-size array.

4. **Termination Condition**: The program terminates when a zero is encountered as the first integer, consistent with the specification.

5. **Output**: The output format is preserved using `println!`, ensuring that the index of the last remaining participant is printed on a new line.

6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to the requirement for safe Rust code. 

This translation maintains the semantics of the original C code while ensuring it adheres to Rust's safety and idiomatic practices.
