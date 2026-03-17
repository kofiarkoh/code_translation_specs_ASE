use std::io::{self, BufRead};

fn get_ints() -> Result<(usize, usize), io::Error> {
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut line = String::new();
    handle.read_line(&mut line)?;
    let mut nums = line.split_whitespace()
        .filter_map(|s| s.parse::<usize>().ok());
    let n = nums.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing n"))?;
    let k = nums.next().ok_or(io::Error::new(io::ErrorKind::InvalidInput, "Missing k"))?;
    Ok((n, k))
}

fn main() -> Result<(), io::Error> {
    let (n, k) = get_ints()?;

    // Check preconditions
    if k > n {
        return Ok(()); // No output if k > n
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
        println!(); // Print newline after each combination
    }
    Ok(())
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The `get_ints` function reads a line from standard input and splits it into two integers, `n` and `k`. This is done using Rust's standard library for I/O, which is safe and idiomatic.

2. **Precondition Check**: The check for `k > n` is performed immediately after reading the inputs. If this condition is true, the program exits without producing any output, adhering to the specification.

3. **Combination Generation**: The loop generates all possible combinations of indices using bit manipulation, similar to the C code. The `count_ones` method is used to count the number of set bits in the pattern, ensuring it matches `k`.

4. **Output Formatting**: The output format is preserved exactly as specified, with the binary pattern followed by the selected indices, and each combination printed on a new line.

5. **Error Handling**: The program returns an error if input reading fails, consistent with the specification's requirements for error behavior.

6. **Safety**: The Rust code avoids any use of `unsafe` blocks, ensuring that all operations are safe and idiomatic.
