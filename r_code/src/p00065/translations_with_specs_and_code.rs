use std::collections::HashMap;
use std::io::{self, BufRead};

fn main() {
    // Create a HashMap to store the counts of each unique integer
    let mut counts: HashMap<i32, usize> = HashMap::new();
    let stdin = io::stdin();
    let reader = stdin.lock();

    // Read pairs of integers from standard input
    for line in reader.lines() {
        let line = line.expect("Failed to read line");
        let parts: Vec<&str> = line.split(',').collect();
        
        // Parse the integers and update their counts
        if parts.len() == 2 {
            if let (Ok(first), Ok(second)) = (parts[0].trim().parse::<i32>(), parts[1].trim().parse::<i32>()) {
                *counts.entry(first).or_insert(0) += 1;
                *counts.entry(second).or_insert(0) += 1;
            }
        }
    }

    // Collect and sort the integers that have a count greater than one
    let mut output: Vec<(i32, usize)> = counts.iter()
        .filter(|&(_, &count)| count > 1)
        .map(|(&num, &count)| (num, count))
        .collect();
    
    output.sort_by_key(|&(num, _)| num);

    // Print the results
    for (num, count) in output {
        println!("{} {}", num, count);
    }
}
```

### Explanation of Translation Choices:
1. **Data Structure**: Used a `HashMap` to count occurrences of integers, which is idiomatic in Rust for counting unique items.
2. **Input Handling**: Used `BufRead` to read lines from standard input, which is efficient and handles EOF naturally.
3. **Parsing**: Split the input line by commas and trimmed whitespace before parsing to ensure valid integers are processed.
4. **Sorting and Output**: Collected results into a vector, filtered for counts greater than one, sorted them, and printed in the required format.
5. **Error Handling**: The program does not handle malformed input as specified, so it assumes valid input as per the specification.
