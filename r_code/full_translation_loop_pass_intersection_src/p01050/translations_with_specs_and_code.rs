use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap(); // per spec: at least one token

    // Frequency table for byte values 0..=255.
    // Spec assumes characters map into 0..=255; we use the raw UTF-8 bytes of the token.
    let mut freq = [0i32; 256];
    for &b in s.as_bytes() {
        freq[b as usize] += 1;
    }

    // l = remaining total count
    let mut l: i32 = s.as_bytes().len() as i32;

    // a = accumulated score, c = current run length within a sweep
    let mut a: i32 = 0;
    let mut c: i32 = 0;

    while l > 0 {
        for i in 0..256usize {
            if freq[i] > 0 {
                a += 1;
                l -= 1;
                c += 1;
                freq[i] -= 1;
            } else {
                if c > 3 {
                    a -= c - 3; // cap run contribution at 3
                }
                c = 0;
            }
        }
        // Note: matches the C code/spec behavior: no end-of-sweep cap is applied
        // unless a zero-frequency value is encountered (which always happens if l>0
        // and not all 256 values are present in that sweep).
    }

    println!("{a}");
}
