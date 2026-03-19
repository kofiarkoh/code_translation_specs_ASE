use std::io::{self, Read};

fn main() {
    // Read entire stdin, then take the first whitespace-delimited token (as in scanf("%s")).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let token = input.split_whitespace().next().unwrap(); // per spec: at least one token exists

    // Count frequencies over 256 possible character values.
    // The C code indexes by `char` values; per spec, behavior is only defined for values in [0,255].
    let mut freq = [0i32; 256];
    let mut remaining: i32 = 0;
    for &byte in token.as_bytes() {
        freq[byte as usize] += 1;
        remaining += 1;
    }

    // Simulate the C logic:
    // Repeatedly scan i=0..255, consuming at most one occurrence per value per scan.
    // Maintain `c` as the current run length (consecutive i with b[i] consumed; gaps reset and apply penalty).
    let mut a: i32 = 0;
    let mut c: i32 = 0;

    while remaining > 0 {
        for i in 0..256 {
            if freq[i] > 0 {
                a += 1;
                remaining -= 1;
                c += 1;
                freq[i] -= 1;
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
