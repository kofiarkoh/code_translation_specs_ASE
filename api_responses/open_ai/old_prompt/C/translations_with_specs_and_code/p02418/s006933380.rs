use std::io::{self, Write};

fn main() {
    let mut str1 = String::new();
    let mut str2 = String::new();

    // Read the first string
    io::stdout().flush().unwrap(); // Ensure prompt is displayed before input
    io::stdin().read_line(&mut str1).expect("Failed to read input");
    // Read the second string
    io::stdin().read_line(&mut str2).expect("Failed to read input");

    // Trim newline characters from the input
    let str1 = str1.trim();
    let str2 = str2.trim();

    // Check lengths to ensure they are within the specified limits
    if str1.len() > 100 || str2.len() > 100 {
        eprintln!("Input strings exceed maximum length.");
        return;
    }

    // Concatenate str1 with itself
    let concatenated = format!("{}{}", str1, str1);

    // Check if str2 is a substring of the concatenated string
    if concatenated.contains(str2) {
        println!("Yes");
    } else {
        println!("No");
    }
}
```

### Notes:
1. **Input Handling**: The Rust code uses `read_line` to read input, which is idiomatic for handling strings in Rust. The input is trimmed to remove any newline characters.
2. **Length Check**: The code checks if the lengths of `str1` and `str2` exceed the specified limits (100 characters). This is a safeguard against invalid input.
3. **Concatenation**: The concatenation of `str1` with itself is done using `format!`, which is safe and idiomatic in Rust.
4. **Substring Check**: The `contains` method is used to check for the presence of `str2` in the concatenated string, which is a direct and safe way to perform this operation in Rust.
5. **Output**: The output format is preserved as specified, ensuring that "Yes" or "No" is printed based on the substring check.
