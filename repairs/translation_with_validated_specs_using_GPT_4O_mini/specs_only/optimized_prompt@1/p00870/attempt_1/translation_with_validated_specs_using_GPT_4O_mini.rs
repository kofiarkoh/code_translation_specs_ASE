use std::collections::HashSet;

fn count_distinct_combinations(n: usize, m: usize, strings: Vec<&str>, concatenated: &str) -> usize {
    // Ensure the input constraints are respected
    if n == 0 && m == 0 {
        return 0; // Termination condition
    }

    // Create a set to store distinct combinations of matched strings
    let mut distinct_combinations = HashSet::new();
    
    // Iterate over the concatenated string to find matches
    let concatenated_length = concatenated.len();
    let mut flags = vec![false; n]; // Flag array to track matches

    // Iterate through all possible starting points in the concatenated string
    for start in 0..concatenated_length {
        // Reset flags for each starting point
        flags.fill(false);
        
        // Try to match each string in order
        let mut matched_count = 0;
        let mut current_index = start;

        while current_index < concatenated_length && matched_count < n {
            // Check if the current string matches the substring starting at current_index
            if concatenated[current_index..].starts_with(strings[matched_count]) {
                // Mark this string as matched
                flags[matched_count] = true;
                matched_count += 1;
                current_index += strings[matched_count - 1].len(); // Move index forward
            } else {
                // If no match, break out of the loop
                break;
            }
        }

        // If we matched all strings, create a bitmask representation of the match
        if matched_count == n {
            let bitmask: usize = flags.iter().enumerate()
                .filter(|&(_, &matched)| matched)
                .map(|(i, _)| 1 << i)
                .sum();
            distinct_combinations.insert(bitmask);
        }
    }

    // Return the count of distinct combinations found
    distinct_combinations.len()
}

fn main() {
    // Example usage
    let n = 3;
    let m = 1;
    let strings = vec!["abc", "def", "ghi"];
    let concatenated = "abcdefghi";
    
    let result = count_distinct_combinations(n, m, strings, concatenated);
    println!("{}", result); // Output the result
}
