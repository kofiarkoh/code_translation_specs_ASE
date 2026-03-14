use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let handle = stdin.lock();

    for line in handle.lines() {
        let n: i64 = line.unwrap().trim().parse().unwrap();
        if n == 0 {
            break; // Termination condition
        }
        let count = count_factors_of_5(n);
        println!("{}", count);
    }
}

fn count_factors_of_5(n: i64) -> i64 {
    let mut count = 0;
    for i in (5..=n).step_by(1) {
        let mut b = i;
        while b > 0 {
            if b % 5 != 0 {
                break;
            }
            count += 1;
            b /= 5;
        }
    }
    count
}
```

### Explanation of Translation Choices:
1. **Input Handling**: The Rust code uses `std::io` to read input, which is idiomatic for handling standard input in Rust. The loop continues until a zero is encountered, matching the specification.
  
2. **Counting Factors of 5**: The logic for counting factors of 5 is encapsulated in the `count_factors_of_5` function, which mirrors the C code's logic while ensuring that the semantics are preserved.

3. **Data Types**: The C code uses `long`, which is typically a 64-bit integer on many platforms. In Rust, `i64` is used to ensure compatibility with larger integers, as the specification does not limit the size of the input.

4. **Output Formatting**: The output is printed using `println!`, which automatically adds a newline, preserving the output format specified.

5. **Error Handling**: The code assumes valid input as per the specification (non-negative integers), and thus does not include additional error handling for invalid input. The use of `unwrap()` is safe here because the specification guarantees valid input until a zero is encountered.

6. **Looping and Conditions**: The loop structure and conditions are directly translated from C to Rust, ensuring that the logic remains intact.

7. **Avoiding Unsafe Code**: The translation does not require any `unsafe` blocks, adhering to the goal of using safe Rust.
