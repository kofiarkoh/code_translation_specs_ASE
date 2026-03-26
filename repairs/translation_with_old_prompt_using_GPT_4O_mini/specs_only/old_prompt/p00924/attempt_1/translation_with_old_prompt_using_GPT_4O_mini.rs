fn reorder_bit_string(bit: &mut Vec<char>, runlen: &[usize], len: usize, runlenlen: usize, start: char) -> usize {
    // Check for edge case where the length of the bit string is 1
    if len == 1 {
        return 0; // No flips needed for a single bit
    }

    // Initialize the minimum flips counter
    let mut min_flips = 0;

    // The expected bit based on the starting bit
    let mut expected_bit = start;

    // Iterate through the run lengths and modify the bit string accordingly
    let mut index = 0;
    for &run in runlen.iter().take(runlenlen) {
        // Count the number of flips needed for the current run
        for _ in 0..run {
            if index < len {
                if bit[index] != expected_bit {
                    min_flips += 1; // Increment flips if the bit does not match the expected bit
                }
                index += 1;
            }
        }
        // Toggle the expected bit for the next run
        expected_bit = if expected_bit == '0' { '1' } else { '0' };
    }

    // Modify the bit string to reflect the expected configuration
    expected_bit = start;
    index = 0;
    for &run in runlen.iter().take(runlenlen) {
        for _ in 0..run {
            if index < len {
                bit[index] = expected_bit; // Set the bit to the expected value
                index += 1;
            }
        }
        expected_bit = if expected_bit == '0' { '1' } else { '0' };
    }

    min_flips // Return the minimum number of flips required
}

fn main() {
    let mut bit_string = vec!['0', '1', '0', '1'];
    let run_lengths = vec![2, 2];
    let len = bit_string.len();
    let runlenlen = run_lengths.len();
    let start = '0';

    let flips = reorder_bit_string(&mut bit_string, &run_lengths, len, runlenlen, start);
    println!("{}", flips); // Output the minimum number of flips
}
