use std::io::{self, BufRead};

fn get_int() -> i32 {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    line.trim().parse().unwrap_or(0) // Returns 0 on parse failure
}

fn get_int2() -> (i32, i32) {
    let stdin = io::stdin();
    let line = stdin.lock().lines().next().unwrap().unwrap();
    let mut nums = line.split_whitespace().map(|s| s.parse().unwrap_or(0));
    (nums.next().unwrap(), nums.next().unwrap())
}

fn main() {
    let (n, k) = get_int2();

    // Ensure k is within valid range
    if k < 0 || k > n {
        return; // No output if k is out of bounds
    }

    for pat in 0..(1 << n) {
        if pat.count_ones() != k as u32 {
            continue;
        }
        print!("{}:", pat);
        for i in 0..n {
            if pat & (1 << i) != 0 {
                print!(" {}", i);
            }
        }
        println!(); // Print newline after each pattern
    }
}
```

### Notes:
1. **Input Handling**: The `get_int` and `get_int2` functions read from standard input and handle parsing. If parsing fails, they return `0` or a tuple of `0`s, respectively, which aligns with the C code's behavior.
2. **Bit Counting**: The Rust method `count_ones()` is used to count the number of bits set to 1, which is idiomatic and safe.
3. **Output Formatting**: The output format is preserved exactly as specified, including the use of `print!` and `println!` for formatting.
4. **Error Handling**: The program checks if `k` is within valid bounds before proceeding to generate patterns, consistent with the specification.
5. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees.
