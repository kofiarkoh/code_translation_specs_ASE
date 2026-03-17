// main.rs

const MAX: usize = 50001;

fn make_prime_table(table: &mut [u8; MAX]) {
    for i in 0..MAX {
        table[i] = 0;
    }

    for i in 2..MAX {
        if table[i] == 0 {
            let mut j = 2;
            while i * j < MAX {
                table[i * j] = 1;
                j += 1;
            }
        }
    }
}

fn main() {
    let mut ptable = [0u8; MAX];
    make_prime_table(&mut ptable);

    let stdin = std::io::stdin();
    let mut input = String::new();

    loop {
        input.clear();
        if stdin.read_line(&mut input).is_err() {
            break; // Handle read error
        }

        let n: i32 = match input.trim().parse() {
            Ok(num) => num,
            Err(_) => continue, // Ignore invalid input
        };

        if n == 0 {
            break;
        } else {
            let mut cnt = 0;
            for i in 2..=n / 2 {
                if ptable[i as usize] == 0 && ptable[(n - i) as usize] == 0 {
                    cnt += 1;
                }
            }
            println!("{}", cnt);
        }
    }
}
```

### Explanation of Translation Choices:
1. **Array Initialization**: The C code initializes an array to zero. In Rust, we use an array of `u8` initialized with zeros.
2. **Loop Constructs**: The `for` loops in C are translated to Rust's `for` loops, maintaining the same logic.
3. **Input Handling**: The C code uses `scanf`, while Rust uses `std::io::stdin()` for reading input. We handle potential read errors gracefully.
4. **Integer Types**: The C code uses `int`, which is typically a 32-bit signed integer. In Rust, we use `i32` for similar behavior.
5. **Array Indexing**: We ensure that array accesses are safe by casting `i` and `n - i` to `usize` when indexing into `ptable`.
6. **Output**: The `printf` function is replaced with `println!`, preserving the output format.
7. **Loop Control**: The loop continues until a `0` is read, similar to the C code's behavior. Invalid inputs are ignored, which is a common practice in user input handling.
