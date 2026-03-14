use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut word_map: HashMap<String, Vec<u32>> = HashMap::new();

    // Read input pairs
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let mut parts = line.split_whitespace();
        if let (Some(word), Some(num_str)) = (parts.next(), parts.next()) {
            if let Ok(num) = num_str.parse::<u32>() {
                word_map.entry(word.to_string()).or_insert_with(Vec::new).push(num);
            }
        }
    }

    // Collect and sort words
    let mut words: Vec<String> = word_map.keys().cloned().collect();
    words.sort();

    // Output results
    for word in words {
        let nums = word_map.get(&word).unwrap();
        println!("{}", word);
        if !nums.is_empty() {
            let mut sorted_nums = nums.clone();
            sorted_nums.sort();
            print!("{}", sorted_nums[0]);
            for &num in &sorted_nums[1..] {
                print!(" {}", num);
            }
        }
        println!(); // Print newline after each word and its numbers
    }
}
```

### Explanation of Translation Choices:
1. **Data Structures**: 
   - Used a `HashMap<String, Vec<u32>>` to store words and their associated integers. This allows for easy accumulation of integers for each unique word.

2. **Input Handling**: 
   - Used `BufRead` for efficient reading of input lines. Each line is split into parts to extract the word and integer.

3. **Sorting**: 
   - Collected words into a vector and sorted them using Rust's built-in sorting capabilities.

4. **Output Formatting**: 
   - Ensured that the output format matches the specification, including handling of newlines correctly.

5. **Error Handling**: 
   - The program assumes well-formed input as per the specification, so no additional error handling for malformed input is included.

6. **Safety**: 
   - The code avoids any `unsafe` blocks, adhering to Rust's safety guarantees. 

This translation preserves the semantics of the original C code while adhering to Rust's idiomatic practices.
