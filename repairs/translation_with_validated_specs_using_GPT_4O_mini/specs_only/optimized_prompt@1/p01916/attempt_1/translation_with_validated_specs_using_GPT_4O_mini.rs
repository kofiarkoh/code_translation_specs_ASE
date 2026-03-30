fn count_odd_characters(s: &str) -> usize {
    // We will use a fixed-size array to track the parity of each character's occurrences.
    // There are 26 lowercase alphabetic characters.
    let mut char_count = [0; 26];

    // Iterate over each character in the string.
    for c in s.chars() {
        // Calculate the index for the character 'a' to 'z'.
        let index = (c as usize) - ('a' as usize);
        // Toggle the count for the character at the calculated index.
        char_count[index] ^= 1; // XOR operation to toggle between 0 and 1.
    }

    // Count the number of characters that have an odd count.
    char_count.iter().filter(|&&count| count == 1).count()
}

// Example usage (uncomment to test):
// fn main() {
//     let input = "abacabad";
//     let result = count_odd_characters(input);
//     println!("{}", result); // Should print the count of distinct characters with odd occurrences.
// }
