use std::io::{self, Read};

fn main() {
    // C uses fixed-size buffers: B[200] and c[256].
    // Per spec, if input exceeds internal buffer capacity, behavior is undefined.
    // We emulate this by reading a line and assuming it fits within 200 bytes (excluding '\n').
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    // Emulate `gets`: read up to the first line terminator, excluding it.
    let line = input
        .split_terminator(&['\n', '\r'][..])
        .next()
        .unwrap_or("");

    // Fixed-size count array for all possible byte values.
    let mut c = [0u32; 256];

    // Process as bytes to match C `char` indexing behavior.
    let bytes = line.as_bytes();

    // Count run starts: for each position i, if i==0 or bytes[i]!=bytes[i-1], increment c[bytes[i]].
    for i in 0..bytes.len() {
        if i == 0 || bytes[i] != bytes[i - 1] {
            c[bytes[i] as usize] += 1;
        }
    }

    // Output rule per spec: 'x' if runs('o') < runs('x'), else 'o' (ties go to 'o').
    let out = if c[b'o' as usize] < c[b'x' as usize] {
        "x"
    } else {
        "o"
    };

    // Exactly one line of output.
    println!("{out}");
}
