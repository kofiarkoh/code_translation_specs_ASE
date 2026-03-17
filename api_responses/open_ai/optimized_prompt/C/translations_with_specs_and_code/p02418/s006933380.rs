use std::io::{self, Write};

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();

    // Read the first string
    io::stdin().read_line(&mut str1).expect("Failed to read line");
    // Read the second string
    io::stdin().read_line(&mut str2).expect("Failed to read line");

    // Trim whitespace and ensure strings do not exceed 100 characters
    let str1 = str1.trim();
    let str2 = str2.trim();

    // Preconditions: both strings must be non-empty and not exceed 100 characters
    if str1.is_empty() || str2.is_empty() || str1.len() > 100 || str2.len() > 100 {
        eprintln!("Input strings must be non-empty and not exceed 100 characters.");
        return;
    }

    // Create the concatenated string by repeating str1
    let concatenated = format!("{}{}", str1, str1);

    // Check if str2 is a substring of the concatenated string
    if concatenated.contains(str2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
```

### Explanation of Translation Choices:
1. **Input Handling**: Used `read_line` to read input strings, ensuring we can handle the maximum length and trim whitespace.
2. **Precondition Checks**: Added checks to ensure both strings are non-empty and do not exceed 100 characters, as specified.
3. **String Concatenation**: Used `format!` to concatenate `str1` with itself, which is idiomatic in Rust and avoids manual buffer management.
4. **Substring Check**: Used the `contains` method, which is safe and idiomatic for checking substrings in Rust.
5. **Output**: Preserved the output format by using `println!` for "Yes" or "No" based on the substring check.
6. **Error Handling**: Added a simple error message for invalid input, although the specification did not define error behavior explicitly.
