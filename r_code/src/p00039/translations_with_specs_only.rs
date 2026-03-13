use std::io::{self, Read};

fn map_byte_to_value(b: u8) -> i32 {
    match b {
        b'I' => 1,
        b'V' => 5,
        b'X' => 10,
        b'L' => 50,
        b'C' => 100,
        b'D' => 500,
        // The Specification treats any other character as 1000 (this includes 'M').
        _ => 1000,
    }
}

fn main() {
    // Read entire stdin into a string, then iterate whitespace-separated tokens.
    // This matches the scanf-style %s tokenization described in the Specification.
    let mut input = String::new();
    if io::stdin().read_to_string(&mut input).is_err() {
        // On read error, exit with non-zero. The Specification assumes normal EOF/read.
        std::process::exit(1);
    }

    // Fixed-size numeric array reused across tokens (size 101 to allow index len to be read).
    // Per Specification, this array persists across tokens and elements beyond the current
    // token's length are NOT cleared, which may lead to observable leftover data effects.
    let mut vals: [i32; 101] = [0; 101];

    // Process tokens one by one.
    for token in input.split_whitespace() {
        let bytes = token.as_bytes();

        // The Specification states tokens longer than 100 characters cause undefined behavior.
        // To stay well-defined in Rust while remaining faithful, we process at most 100 characters.
        // This is a narrow, conservative interpretation; in the original undefined case the
        // program could overflow the buffer. Here we truncate to 100 to avoid UB in Rust.
        let proc_len = if bytes.len() > 100 { 100 } else { bytes.len() };

        // Fill numeric mapping for positions [0 .. proc_len-1]. Do NOT clear vals[proc_len]
        // or beyond; leftover data from prior tokens remains, per Specification.
        for i in 0..proc_len {
            vals[i] = map_byte_to_value(bytes[i]);
        }

        // Compute the integer value using the left-to-right single-pass subtractive rule.
        let mut sum: i32 = 0;
        let mut i: usize = 0;
        while i < proc_len {
            // When comparing vals[i] < vals[i+1], note that vals[proc_len] may contain leftover
            // data from previous tokens (or initial zero). This matches the specified behavior.
            let curr = vals[i];
            let next = vals[i + 1]; // safe: vals length is 101, and proc_len <= 100
            if curr < next {
                sum += next - curr;
                i += 2;
            } else {
                sum += curr;
                i += 1;
            }
        }

        // Print the result for this token followed by a newline.
        println!("{}", sum);
    }

    // Normal termination with exit code 0 (implicit).
}
