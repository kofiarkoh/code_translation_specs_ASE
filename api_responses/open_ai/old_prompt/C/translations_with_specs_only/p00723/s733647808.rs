// Define a constant for the maximum number of unique strings
const MAX_STRINGS: usize = 1024;
const MAX_LENGTH: usize = 80;

// Global array to store unique strings
static mut UNIQUE_STRINGS: [&str; MAX_STRINGS] = Default::default();
static mut UNIQUE_COUNT: usize = 0;

// Function to process strings and count unique permutations
fn process_strings(n: usize, strings: Vec<&str>) -> Vec<usize> {
    let mut results = Vec::new();

    // Ensure we do not process more strings than allowed
    let count = n.min(MAX_STRINGS);
    
    for i in 0..count {
        let s = strings[i];
        let unique_count = generate_permutations(s);
        results.push(unique_count);
    }

    results
}

// Function to generate unique permutations of a string
fn generate_permutations(s: &str) -> usize {
    unsafe {
        // Reset the unique count
        UNIQUE_COUNT = 0;

        // Generate permutations by reversing segments and swapping characters
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        // Use a set to track unique permutations
        let mut seen = std::collections::HashSet::new();

        // Generate all permutations (this is a simplified version)
        for i in 0..len {
            for j in i..len {
                let mut perm = chars.clone();
                perm.swap(i, j); // Swap characters
                let perm_str: String = perm.iter().collect();

                // Add to the set if unique
                if seen.insert(perm_str.clone()) {
                    if UNIQUE_COUNT < MAX_STRINGS {
                        UNIQUE_STRINGS[UNIQUE_COUNT] = perm_str.as_str();
                        UNIQUE_COUNT += 1;
                    }
                }
            }
        }

        UNIQUE_COUNT // Return the count of unique permutations
    }
}

// Example usage
fn main() {
    let n = 3; // Number of strings to process
    let strings = vec!["abc", "aab", "xyz"]; // Example strings
    let results = process_strings(n, strings);

    for count in results {
        println!("{}", count); // Output the count of unique permutations
    }
}
