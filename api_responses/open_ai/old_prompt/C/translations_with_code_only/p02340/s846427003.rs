use std::io::{self, BufRead};
use std::num::ParseIntError;

const BUF_SIZE: usize = 20;
const DIVISOR: u64 = 1_000_000_007;
const NUM_MAX: usize = 1000;
const K_MAX: usize = 1000;

// pdp[# of partitions][# of balls]
static mut PDP: [[u64; NUM_MAX + 1]; K_MAX + 1] = [[0; NUM_MAX + 1]; K_MAX + 1];

fn get_int2() -> Result<(i32, i32), ParseIntError> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut line = String::new();
    
    // Read a line from stdin
    handle.read_line(&mut line).expect("Failed to read line");
    
    // Parse two integers from the line
    let mut iter = line.split_whitespace();
    let a1 = iter.next().unwrap().parse::<i32>()?;
    let a2 = iter.next().unwrap().parse::<i32>()?;
    
    Ok((a1, a2))
}

fn init() {
    unsafe {
        PDP[0][0] = 1;
        for i in 1..=K_MAX {
            for j in 0..=NUM_MAX {
                PDP[i][j] = PDP[i - 1][j];
                if j >= i {
                    PDP[i][j] = (PDP[i][j] + PDP[i][j - i]) % DIVISOR;
                }
            }
        }
    }
}

fn main() {
    let (n, k) = get_int2().expect("Failed to get integers");
    init();
    
    // Print the result, casting to i32 as in the original C code
    unsafe {
        println!("{}", PDP[k as usize][n as usize]);
    }
}
```

### Explanation of Key Choices:
1. **Static Mutable Array**: The `PDP` array is declared as `static mut` to allow for mutable state across function calls, similar to the global array in C. This requires `unsafe` blocks when accessing it, as Rust's safety guarantees are bypassed.
  
2. **Input Handling**: The `get_int2` function reads from standard input and parses two integers, mimicking the behavior of `fgets` and `sscanf` in C. Error handling is included to ensure robustness.

3. **Integer Types**: The use of `u64` for the `PDP` array matches the `uint64_t` type in C, ensuring that the size and behavior are preserved.

4. **Modulo Operation**: The modulo operation is performed in the same way as in C to ensure that the results wrap around correctly, preserving the intended behavior.

5. **Output**: The output is formatted to match the C code, ensuring that the integer is printed as expected.
