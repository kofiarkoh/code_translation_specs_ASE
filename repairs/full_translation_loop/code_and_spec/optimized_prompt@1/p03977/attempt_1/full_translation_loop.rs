use std::io::{self, Read, Write};

fn main() {
    // Buffer for input and output
    let mut buf = [0u8; 10000];
    let mut out = Vec::with_capacity(20000);
    
    // Read input
    let size = io::stdin().read(&mut buf).expect("Failed to read input");
    let mut sz = 0;

    // Process input
    while sz < size && buf[sz] >= b'0' {
        sz += 1;
    }

    while sz < size && buf[sz] >= b'0' {
        let mut n = 0;
        let mut d = 0;

        // Read N
        loop {
            if sz >= size || buf[sz] < b'0' {
                break;
            }
            n = n * 10 + (buf[sz] - b'0') as i32;
            sz += 1;
        }

        // Read D
        loop {
            if sz >= size || buf[sz] < b'0' {
                break;
            }
            d = d * 10 + (buf[sz] - b'0') as i32;
            sz += 1;
        }

        // Compute the answer based on the specification
        let ans = if n % 2 == 1 {
            127 * (n - 1) + d
        } else {
            127 * n - d
        };

        // Convert the answer to string and store it in the output buffer
        let mut result = ans.to_string();
        result.push('\n');
        out.extend_from_slice(result.as_bytes());
    }

    // Write output
    io::stdout().write_all(&out).expect("Failed to write output");
}
