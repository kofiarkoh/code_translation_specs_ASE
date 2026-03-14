use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut word_map: HashMap<String, Vec<u32>> = HashMap::new();

    // Read input pairs of words and integers
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split_whitespace().collect();
        
        // Expecting pairs of (word, integer)
        if parts.len() == 2 {
            let word = parts[0].to_string();
            let number: u32 = parts[1].parse().expect("Failed to parse integer");

            // Insert the number into the vector associated with the word
            word_map.entry(word).or_insert_with(Vec::new).push(number);
        }
    }

    // Collect unique words and sort them
    let mut unique_words: Vec<String> = word_map.keys().cloned().collect();
    unique_words.sort();

    // Output each unique word followed by its associated integers
    for word in unique_words {
        let numbers = &word_map[&word];
        if numbers.is_empty() {
            println!("{}", word);
        } else {
            println!("{}", word);
            for number in numbers {
                println!("{}", number);
            }
        }
    }
}
```

### Explanation of Choices:
1. **Data Structure**: A `HashMap<String, Vec<u32>>` is used to store words and their associated integers. This allows for easy accumulation of integers for each word while maintaining the integrity of the data.
  
2. **Input Handling**: The program reads from standard input until EOF, processing each line into a word-integer pair. It assumes the input is well-formed as per the specification.

3. **Sorting and Output**: After collecting the words, they are sorted lexicographically. The output format adheres to the specification, ensuring each word is followed by its integers on new lines.

4. **Error Handling**: The program does not handle malformed input, as specified. It will panic if the input is not as expected, which aligns with the undefined behavior clause in the specification.

5. **Idiomatic Rust**: The code avoids `unsafe` blocks, adhering to safe Rust practices while ensuring the program's behavior matches the specification closely.
