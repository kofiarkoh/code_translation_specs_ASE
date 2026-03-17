fn reorder_bit_string(bit: &mut Vec<char>, runlen: &[usize], len: usize, runlenlen: usize, start: char) -> usize {
    // Check for edge case where the bit string length is 1
    if len == 1 {
        return 0; // No flips needed for a single bit
    }

    // Function to recursively calculate the minimum flips
    fn min_flips(bit: &mut Vec<char>, runlen: &[usize], index: usize, current_bit: char) -> usize {
        // Base case: if we have processed all runs
        if index >= runlen.len() {
            return 0; // No more flips needed
        }

        // Calculate the expected bit for the current run
        let expected_bit = if current_bit == '0' { '1' } else { '0' };
        let run_length = runlen[index];

        // Count flips needed for the current run
        let mut flips = 0;
        for i in 0..run_length {
            if bit[i] != current_bit {
                flips += 1; // Increment flips if the bit does not match the expected
            }
        }

        // Modify the bit string to reflect the current run
        for i in 0..run_length {
            bit[i] = current_bit; // Set the bits to the current expected value
        }

        // Recur for the next run with the toggled bit
        flips + min_flips(bit, runlen, index + 1, expected_bit)
    }

    // Start the recursive process with the initial bit
    min_flips(bit, runlen, 0, start)
}

fn main() {
    let mut bit_string = vec!['0', '1', '0', '1']; // Example bit string
    let run_lengths = vec![2, 2]; // Example run lengths
    let len = bit_string.len();
    let runlenlen = run_lengths.len();
    let start = '0'; // Starting bit

    let min_flips = reorder_bit_string(&mut bit_string, &run_lengths, len, runlenlen, start);
    println!("{}", min_flips); // Output the minimum number of flips
}
