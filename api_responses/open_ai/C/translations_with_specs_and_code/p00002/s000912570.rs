use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();

    for line in reader.lines() {
        // Read each line and attempt to parse two integers
        if let Ok(line) = line {
            let mut nums = line.split_whitespace().filter_map(|s| s.parse::<i32>().ok());
            if let (Some(a), Some(b)) = (nums.next(), nums.next()) {
                // Calculate the sum
                let sum = a + b;
                // Convert the sum to a string and get its length
                let length = sum.to_string().len();
                // Output the length
                println!("{}", length);
            }
        }
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `BufRead` to read lines from standard input, which is idiomatic for handling input in Rust. This approach allows for reading until EOF, as specified.
  
2. **Parsing Integers**: The integers are parsed using `filter_map` to safely handle potential parsing errors. If the input does not consist of valid integers, the program simply skips that line, which aligns with the specification's note on undefined behavior for invalid input.

3. **String Representation**: The sum is converted to a string using `to_string()`, and its length is obtained using `len()`, which is straightforward and safe.

4. **Output**: The output format is preserved exactly as specified, with each length printed on a new line.

5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees while fulfilling the functional requirements of the specification. 

This translation maintains the semantics of the original C code while ensuring that it adheres to Rust's idiomatic practices.
