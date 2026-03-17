use std::io::{self, Write};

const MEW: usize = 112345;

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: usize = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    let w = &mut [0; 128];
    let v = &mut [0; 128];

    for i in 0..n {
        input.clear();
        io::stdin().read_line(&mut input).expect("Failed to read line");
        let mut iter = input.split_whitespace();
        w[i] = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
        v[i] = iter.next().and_then(|s| s.parse().ok()).unwrap_or(0);
    }

    let mut ayaneru = vec![W + 1; MEW];
    ayaneru[0] = 0;

    for i in 0..n {
        for j in (v[i]..=MEW - 1).rev() {
            if ayaneru[j - v[i]] + w[i] < ayaneru[j] {
                ayaneru[j] = ayaneru[j - v[i]] + w[i];
            }
        }
    }

    for i in (0..=MEW - 1).rev() {
        if ayaneru[i] <= W {
            println!("{}", i);
            return;
        }
    }
    
    println!("error!");
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The C code uses `scanf` for input, which is replaced with `read_line` in Rust. The input is split into whitespace-separated values, similar to how `scanf` would parse them.
2. **Array Initialization**: The `ayaneru` array is initialized with `W + 1` using a vector in Rust, which is more idiomatic than a fixed-size array for this case.
3. **Loop Constructs**: The nested loops are translated directly, preserving the logic and order of operations.
4. **Output Handling**: The `printf` and `puts` functions are replaced with `println!`, which handles formatting and newlines appropriately.
5. **Error Handling**: The C code returns `1` on error, while Rust's `main` function returns `()`, so we handle errors with `expect` and `unwrap_or` to ensure the program behaves similarly.
6. **Safety**: The translation avoids `unsafe` blocks, as all operations can be performed safely in Rust.
