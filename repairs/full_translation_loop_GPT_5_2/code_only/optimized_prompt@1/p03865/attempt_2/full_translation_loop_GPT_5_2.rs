use std::io::{self, Read};

fn main() {
    // C: char buf[114514]; n = read(0, buf, 114514);
    let mut buf = vec![0u8; 114_514];
    let n_read = io::stdin().read(&mut buf).unwrap_or(0);

    // C: n--;  (n is an int; allow negative)
    let mut n: i64 = n_read as i64;
    n -= 1;

    // C expression:
    // puts( n + ( (buf[0] == buf[n-1]) & 1 ) ? "First" : "Second" );
    //
    // Important: in typical inputs, the last byte read is '\n'. The C code does NOT
    // strip it; it compares buf[0] with buf[n-1] where n is (bytes_read - 1),
    // so n-1 indexes (bytes_read - 2), i.e., the last non-newline character.
    //
    // Also, (buf[0]==buf[n-1]) is 0/1, and &1 keeps it 0/1.
    // The conditional is true iff (n + that_bit) != 0.
    let eq_bit: i64 = if n_read >= 2 {
        let last_idx = n_read - 2; // corresponds to buf[n-1] after n-- in C
        if buf[0] == buf[last_idx] { 1 } else { 0 }
    } else {
        // For very short reads, the C code would have UB; choose 0 as a conservative emulation.
        0
    };

    let cond = n + eq_bit;
    if cond != 0 {
        println!("First");
    } else {
        println!("Second");
    }
}
