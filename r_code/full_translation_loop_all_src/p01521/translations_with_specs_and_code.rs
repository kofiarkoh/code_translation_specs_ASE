use std::io::{self, Read};

fn main() {
    // Read entire stdin (C code uses gets on a fixed buffer; spec says single line).
    // We take the first line if multiple are present.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let line = input.lines().next().unwrap_or("");

    // C code uses a 256-byte array indexed by char value; it effectively counts bytes.
    // It increments only when the current byte differs from the previous byte
    // (i.e., counts runs of consecutive identical characters).
    let bytes = line.as_bytes();
    let mut counts = [0u32; 256];

    for (i, &b) in bytes.iter().enumerate() {
        if i == 0 || b != bytes[i - 1] {
            counts[b as usize] += 1;
        }
    }

    // Spec: output 'x' if count('o') < count('x'), else 'o' (tie prefers 'o' per spec).
    // Note: This matches the C code's condition and output.
    let out = if counts[b'o' as usize] < counts[b'x' as usize] {
        "x"
    } else {
        "o"
    };

    // C uses puts => prints with trailing newline.
    println!("{out}");
}
