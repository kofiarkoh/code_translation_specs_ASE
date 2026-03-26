use std::collections::HashSet;

const MAX_STRINGS: usize = 1024;
const MAX_LENGTH: usize = 80;

static mut A: [String; MAX_STRINGS] = Default::default(); // Global array to store unique strings

fn process_strings(n: usize, strings: &[&str]) -> Vec<usize> {
    let mut results = Vec::new();

    for i in 0..n {
        let s = strings[i];
        let mut unique_permutations = HashSet::new();
        let chars: Vec<char> = s.chars().collect();
        let len = chars.len();

        // Generate unique permutations by reversing segments and swapping characters
        for j in 0..len {
            for k in j..len {
                let mut perm = chars.clone();
                perm[j..=k].reverse(); // Reverse the segment from j to k
                unique_permutations.insert(perm.iter().collect::<String>());
            }
        }

        // Store unique permutations in the global array
        unsafe {
            let mut ap = 0; // Reset count of unique permutations
            for perm in unique_permutations {
                if ap < MAX_STRINGS {
                    A[ap] = perm;
                    ap += 1;
                }
            }
            results.push(ap); // Store the count of unique permutations
        }
    }

    results
}

// Example usage
fn main() {
    let n = 3;
    let strings = ["abc", "aab", "xyz"];
    let counts = process_strings(n, &strings);
    for count in counts {
        println!("{}", count);
    }
}
