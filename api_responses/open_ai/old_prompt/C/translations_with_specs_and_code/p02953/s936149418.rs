fn main() {
    use std::io::{self, BufRead};

    // Read input
    let stdin = io::stdin();
    let handle = stdin.lock();
    let mut lines = handle.lines();

    // Read the number of elements
    let n: usize = lines.next().unwrap().unwrap().trim().parse().unwrap();
    let mut a: Vec<i32> = lines
        .take(n)
        .map(|line| line.unwrap().trim().parse().unwrap())
        .collect();

    // Check if the array can be transformed into non-decreasing order
    for i in (1..n).rev() {
        if a[i] < a[i - 1] {
            a[i - 1] -= 1; // Decrement the previous element
        }
        if a[i] < a[i - 1] {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `std::io` for reading input, which is idiomatic in Rust. The input is read line by line and parsed into integers.
2. **Array Declaration**: Used a `Vec<i32>` to store the integers, which is a dynamic array in Rust, allowing for easy manipulation.
3. **Looping**: The loop iterates from the end of the array to the beginning, similar to the C code, ensuring that the logic remains intact.
4. **Decrement Logic**: The decrement operation is preserved, and the check for the non-decreasing condition is directly translated.
5. **Output**: Used `println!` for outputting "Yes" or "No", which matches the required output format.
6. **Safety**: The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees. 

This translation maintains the semantics of the original C code while ensuring it adheres to Rust's idiomatic practices.
