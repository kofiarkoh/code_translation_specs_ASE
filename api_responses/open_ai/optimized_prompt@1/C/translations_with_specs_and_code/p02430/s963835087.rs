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
        if pat.count_ones() as usize != k {
            continue;
        }
        print!("{}:", pat);
        let indices: Vec<usize> = (0..n)
            .filter(|i| pat & (1 << i) != 0)
            .collect();
        for index in indices {
            print!(" {}", index);
        }
        println!(); // Newline after each combination
    }
    Ok(())
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The `get_ints` function reads a line from standard input and splits it into two integers, `n` and `k`. It uses `Result` to handle potential errors, which aligns with Rust's error handling philosophy.

2. **Preconditions Check**: The check for `k > n` is performed immediately after reading the inputs. If this condition is true, the program exits without producing any output, as specified.

3. **Combination Generation**: The loop generates all possible combinations of indices using bit manipulation, similar to the C code. The `count_ones` method is used to count the number of set bits in the pattern, ensuring that exactly `k` indices are selected.

4. **Output Formatting**: The output format is preserved exactly as specified, with the pattern followed by the selected indices, and each combination printed on a new line.

5. **Safety and Idiomatic Rust**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees. The use of iterators and functional programming constructs (like `filter` and `collect`) makes the code more idiomatic.

6. **Error Handling**: The program returns an error if input reading fails, consistent with the specification's requirements for error behavior.
