fn main() {
    // Example input values; in a real scenario, these would be read from standard input.
    let num: usize = 10; // Number of characters in the binary string
    let changes: usize = 2; // Number of '0's that can be changed to '1's
    let binary_string: &str = "1001101001"; // A binary string of length `num`

    // Check for the precondition that num is non-negative and matches the length of the string
    if num == 0 {
        println!("0");
        return;
    }

    // Ensure the binary string length matches num
    assert_eq!(binary_string.len(), num);

    // Initialize arrays to hold the segments of '1's and cumulative counts
    let mut arr: Vec<usize> = Vec::new();
    let mut cum: Vec<usize> = Vec::new();

    // Process the binary string to populate arr and cum
    let mut count = 0;
    for c in binary_string.chars() {
        if c == '1' {
            count += 1;
        } else {
            if count > 0 {
                arr.push(count);
                cum.push(arr.iter().sum());
                count = 0;
            }
        }
    }
    // Push the last segment if it ends with '1's
    if count > 0 {
        arr.push(count);
        cum.push(arr.iter().sum());
    }

    // Calculate the maximum length of contiguous '1's after allowed changes
    let mut max_length = 0;
    let mut left = 0;
    let mut right = 0;
    let mut zero_count = 0;

    while right < arr.len() {
        // Count the number of '0's between segments of '1's
        if right > 0 {
            zero_count += 1; // Count the '0' between segments
        }

        // If we exceed the allowed changes, move the left pointer
        while zero_count > changes {
            left += 1;
            if left < right {
                zero_count -= 1; // Remove the '0' we are skipping
            }
        }

        // Calculate the current length of contiguous '1's
        let current_length = cum[right] + (right - left) - zero_count;
        max_length = max_length.max(current_length);

        // Move the right pointer to expand the window
        right += 1;
    }

    // Output the maximum length of contiguous '1's
    println!("{}", max_length);
}
