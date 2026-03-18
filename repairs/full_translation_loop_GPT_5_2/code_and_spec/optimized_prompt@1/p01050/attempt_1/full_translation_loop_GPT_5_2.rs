use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (like scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let s = input.split_whitespace().next().unwrap_or("");

    // Frequency table for byte values 0..=255.
    // SPEC says ASCII-like range; C code indexes by char byte value.
    let mut b = [0i32; 256];
    let mut l: i32 = 0;
    for &ch in s.as_bytes() {
        b[ch as usize] += 1;
        l += 1;
    }

    let mut a: i32 = 0;
    let mut c: i32 = 0;

    // Preserve the C logic exactly.
    while l != 0 {
        for i in 0..256 {
            if b[i] != 0 {
                a += 1;
                l -= 1;
                c += 1;
                b[i] -= 1;
            } else {
                if c > 3 {
                    a -= c - 3;
                }
                c = 0;
            }
        }
    }

    print!("{a}\n");
}
