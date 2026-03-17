use std::io::{self, Write};

fn main() {
    let mut n = String::new();
    let mut string = String::new();

    // Read the integer n
    io::stdin().read_line(&mut n).expect("Failed to read line");
    let n: usize = n.trim().parse().expect("Please enter a valid number");

    // Read the string
    io::stdin().read_line(&mut string).expect("Failed to read line");
    let string = string.trim();

    // Ensure the length of the string matches n
    if n < 3 || string.len() != n {
        println!("0");
        return;
    }

    let mut count = 0;
    let mut i = 0;

    // Count occurrences of "ABC"
    while i <= n - 3 {
        if &string[i..i + 3] == "ABC" {
            count += 1;
            i += 1; // Allow overlapping occurrences
        } else {
            i += 1;
        }
    }

    // Print the result
    println!("{}", count);
}
```

### Notes:
1. **Input Handling**: The Rust code reads input using `std::io`, which is idiomatic for handling standard input in Rust.
2. **String Length Check**: The check for `n < 3` and the string length ensures that we adhere to the specification regarding valid input.
3. **Substring Matching**: The substring matching is done using slicing, which is safe and idiomatic in Rust.
4. **Overlapping Occurrences**: The increment of `i` by 1 allows for overlapping occurrences of "ABC", consistent with the C code's behavior.
5. **Output**: The output format is preserved, ensuring a single integer followed by a newline.
