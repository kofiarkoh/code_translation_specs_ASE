use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let mut iter = input.split_whitespace();
    
    let n: i64 = iter.next().unwrap().parse().expect("Failed to parse n");
    let mut p: i64 = iter.next().unwrap().parse().expect("Failed to parse p");
    let mut ans: i64 = 1;

    for i in 2..=((p as f64).sqrt() as i64) {
        let mut cnt = 0;
        while p % i == 0 {
            cnt += 1;
            p /= i;
        }
        if cnt >= n {
            ans *= i.pow((cnt / n) as u32);
        }
    }
    
    if p > 1 && n == 1 {
        ans *= p;
    }
    
    println!("{}", ans);
}
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `io::stdin().read_line()` to read input, which is idiomatic in Rust for reading from standard input.
2. **Integer Types**: Used `i64` to match the `long` type in C, ensuring we can handle large integers.
3. **Loop Constructs**: The `for` loop and `while` loop are directly translated to their Rust equivalents.
4. **Power Calculation**: Used `i.pow((cnt / n) as u32)` to compute powers, which is safe and idiomatic in Rust.
5. **Output**: Used `println!` for output, preserving the format of the original C `printf`.
6. **Corner Cases**: The logic for handling `p` and `n` is preserved exactly as in the C code, including the check for `p > 1 && n == 1`.
