use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut car: Vec<i32> = Vec::with_capacity(100);
    let mut out: Vec<i32> = Vec::with_capacity(100);
    
    for line in handle.lines() {
        // Read each line and parse it as an integer
        if let Ok(input) = line {
            if let Ok(n) = input.trim().parse::<i32>() {
                if n == 0 {
                    // Output the last stored value before the zero
                    if let Some(&last) = car.last() {
                        out.push(last);
                        car.pop(); // Remove the last element after outputting
                    }
                } else {
                    // Store the non-zero value
                    if car.len() < 100 {
                        car.push(n);
                    }
                }
            }
        }
    }

    // Output the stored values in reverse order
    for value in out.iter() {
        println!("{}", value);
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `BufRead` to read lines from standard input, which is idiomatic for handling input in Rust. Each line is trimmed and parsed as an integer.
2. **Storage**: The `car` vector is used to store non-zero integers, and `out` is used to store the output values. Both vectors are initialized with a capacity of 100 to adhere to the specification.
3. **Zero Handling**: When a zero is encountered, the last stored value is pushed to the `out` vector, and the last element is removed from `car` to maintain the correct state.
4. **Output**: The output is printed in the order specified, ensuring that each value is printed on a new line.
5. **Error Handling**: The program does not handle invalid input types, as specified. If a non-integer is encountered, it will simply skip that input without crashing.
6. **Safety**: The code avoids `unsafe` blocks, adhering to Rust's safety guarantees. The use of `Vec` ensures that we do not exceed the bounds of the array, as it dynamically manages memory.
