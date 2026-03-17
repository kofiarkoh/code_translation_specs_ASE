use std::io::{self, Write};

fn main() {
    let mut input = String::new();
    
    // Read the length of the strings
    io::stdin().read_line(&mut input).expect("Failed to read line");
    let n: usize = input.trim().parse().expect("Input must be a non-negative integer");

    // Read the two strings
    let mut s = String::new();
    let mut t = String::new();
    
    io::stdin().read_line(&mut s).expect("Failed to read line");
    io::stdin().read_line(&mut t).expect("Failed to read line");

    // Ensure the strings are of length N
    assert_eq!(s.len(), n, "String S must be of length N");
    assert_eq!(t.len(), n, "String T must be of length N");

    let mut ans = 0;
    let mut i = n as isize - 1; // Start from the last index of S
    let mut j;

    while i >= 0 {
        j = i; // Reset j to the current value of i
        while j >= 0 && s.as_bytes()[i as usize] != t.as_bytes()[j as usize] {
            j -= 1;
        }
        if j >= 0 {
            while j >= 0 && s.as_bytes()[i as usize] == t.as_bytes()[j as usize] {
                i -= 1;
                j -= 1;
            }
            if j < 0 {
                ans = 2 * n - (n - i - 1) as usize;
                break;
            }
        } else {
            ans = 2 * n - (n - i - 1) as usize;
            break;
        }
        i -= 1; // Move to the next character in S
    }

    // Output the result
    println!("{}", ans);
}
```

### Notes:
1. **Input Handling**: The Rust code uses `io::stdin()` to read input, similar to `scanf` in C. It reads the length of the strings and the strings themselves.
2. **Assertions**: The code checks that the lengths of the strings match the specified length \( N \) using assertions, which is a safe way to ensure preconditions are met.
3. **String Indexing**: The strings are accessed as byte slices using `as_bytes()`, which is necessary for character comparison.
4. **Loop Logic**: The logic for comparing suffixes and prefixes is preserved from the C code, ensuring that the semantics remain intact.
5. **Output**: The result is printed using `println!`, which matches the output behavior of the original C code.
