use std::io::{self, Read, Write};

fn main() {
    // Buffer sizes as specified
    const INPUT_BUFFER_SIZE: usize = 10_000;
    const OUTPUT_BUFFER_SIZE: usize = 20_000;

    let mut input_buffer = [0u8; INPUT_BUFFER_SIZE];
    let mut output_buffer = Vec::with_capacity(OUTPUT_BUFFER_SIZE);
    
    // Read input from standard input
    let bytes_read = io::stdin().read(&mut input_buffer).expect("Failed to read from stdin");
    
    let mut sz = 0;
    while sz < bytes_read && input_buffer[sz] >= b'0' {
        sz += 1;
    }

    while sz < bytes_read && input_buffer[sz] >= b'0' {
        let mut n = 0;
        let mut d = 0;

        // Read N
        while sz < bytes_read && input_buffer[sz] >= b'0' {
            n = n * 10 + (input_buffer[sz] - b'0') as i32;
            sz += 1;
        }

        // Read D
        while sz < bytes_read && input_buffer[sz] >= b'0' {
            d = d * 10 + (input_buffer[sz] - b'0') as i32;
            sz += 1;
        }

        // Calculate the answer based on the parity of N
        let ans = if n % 2 == 1 {
            127 * (n - 1) + d
        } else {
            127 * n - d
        };

        // Convert the answer to string and append to output buffer
        let mut result = ans.to_string();
        result.push('\n'); // Append newline as specified
        output_buffer.extend_from_slice(result.as_bytes());
    }

    // Write the output to standard output
    io::stdout().write_all(&output_buffer).expect("Failed to write to stdout");
}
