use std::io::{self, Write};

fn rev(n: i32) -> i32 {
    let mut out = 0;
    let mut num = n;

    while num > 0 {
        out = out * 10 + (num % 10);
        num /= 10;
    }

    out
}

fn nearest_palindrome(n: i32) -> i32 {
    let mut i = 0;

    loop {
        if n - i >= 0 {
            let candidate = n - i;
            if rev(candidate) == candidate {
                return candidate;
            }
        }
        let candidate = n + i;
        if rev(candidate) == candidate {
            return candidate;
        }
        i += 1;
    }
}

fn main() {
    let mut input = String::new();
    io::stdout().flush().unwrap(); // Ensure the output buffer is flushed before reading input
    io::stdin().read_line(&mut input).expect("Failed to read line");
    
    let n: i32 = input.trim().parse().expect("Input must be a non-negative integer");

    if n < 0 {
        eprintln!("Input must be a non-negative integer");
        return;
    }

    let palindrome = nearest_palindrome(n);
    println!("{}", palindrome);
}
```

### Explanation of Translation Choices:
1. **Functionality Preservation**: The `rev` function is directly translated to reverse the digits of an integer. The main logic for finding the nearest palindrome is encapsulated in the `nearest_palindrome` function.
  
2. **Input Handling**: The program reads from standard input and ensures that the input is a non-negative integer, adhering to the precondition specified.

3. **Loop Logic**: The loop in `nearest_palindrome` systematically searches for the nearest palindrome by checking both `n - i` and `n + i`, similar to the C code.

4. **Error Handling**: The Rust code includes a check for negative input, which is not explicitly mentioned in the C code but is necessary to satisfy the precondition.

5. **Output Formatting**: The output is printed in the same format as the C code, ensuring that the newline character is preserved.

6. **Safety**: The code avoids any use of `unsafe` blocks, adhering to Rust's safety guarantees. 

7. **Comments**: Comments are added where necessary to clarify the intent and functionality of the code.
