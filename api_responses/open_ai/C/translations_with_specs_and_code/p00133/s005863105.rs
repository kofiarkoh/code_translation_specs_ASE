// Rotation of a Pattern in Rust
use std::io::{self, BufRead};

const SIZE: usize = 8;

fn rotate(pattern: &[[char; SIZE]; SIZE], rotated: &mut [[char; SIZE]; SIZE]) {
    for i in 0..SIZE {
        for j in 0..SIZE {
            rotated[i][j] = pattern[SIZE - 1 - j][i];
        }
    }
}

fn display(pattern: &[[char; SIZE]; SIZE]) {
    for row in pattern.iter() {
        println!("{}", row.iter().collect::<String>());
    }
}

fn main() {
    let stdin = io::stdin();
    let mut original_pattern = [[' '; SIZE]; SIZE];
    let mut rotated_pattern = [[' '; SIZE]; SIZE];

    // Read the original pattern
    for i in 0..SIZE {
        let mut input = String::new();
        stdin.lock().read_line(&mut input).expect("Failed to read line");
        // Trim the input to ensure it has exactly 8 characters
        let trimmed = input.trim();
        if trimmed.len() != SIZE {
            panic!("Each row must contain exactly 8 characters.");
        }
        original_pattern[i] = trimmed.chars().collect::<Vec<char>>().try_into().unwrap();
    }

    // Perform rotations and display results
    for i in 1..=3 {
        rotate(&original_pattern, &mut rotated_pattern);
        println!("{}", i * 90);
        display(&rotated_pattern);
        original_pattern.copy_from_slice(&rotated_pattern);
    }
}
```

### Explanation of Translation Choices:
1. **Data Structures**: Used a 2D array of characters (`[[char; SIZE]; SIZE]`) to represent the 8x8 pattern, similar to the C code's `char pat1[8][9]` and `char pat2[8][9]`. The extra space in C was for the null terminator, which is not needed in Rust.

2. **Input Handling**: Used `std::io` for reading input, ensuring that each line is trimmed to exactly 8 characters, as specified.

3. **Rotation Logic**: The rotation function mirrors the logic in the C code, rotating the pattern 90 degrees clockwise.

4. **Display Function**: The display function collects each row into a `String` for printing, preserving the output format.

5. **Safety**: The code avoids `unsafe` blocks, ensuring memory safety and correctness.

6. **Error Handling**: A panic is triggered if the input does not meet the specified precondition of exactly 8 characters per row, which is a safe way to handle this in Rust.

7. **Preserving Semantics**: The original pattern is preserved until the first rotation occurs, and the output format matches the specification exactly.
