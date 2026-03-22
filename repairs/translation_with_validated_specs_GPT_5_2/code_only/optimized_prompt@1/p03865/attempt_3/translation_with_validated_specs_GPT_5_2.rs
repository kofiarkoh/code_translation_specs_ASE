use std::io::{self, Read};

fn main() {
    // Read entire stdin (the original uses read(0, buf, 114514)).
    // Using read_to_end matches typical contest inputs and avoids partial reads.
    let mut buf = Vec::new();
    io::stdin().read_to_end(&mut buf).unwrap_or(0);

    // C: n = read(...); n--;
    // Here n is the number of bytes read, then decremented by 1.
    // Use wrapping to emulate C's potential underflow behavior.
    let n_read = buf.len() as isize;
    let n = n_read.wrapping_sub(1);

    // C expression:
    // puts(n+(buf[0]==buf[n-1])&1?"First":"Second");
    //
    // Correct C precedence:
    //   & has LOWER precedence than +, but HIGHER precedence than ?:.
    // So it parses as:
    //   ( (n + (buf[0]==buf[n-1])) & 1 ) ? "First" : "Second"
    //
    // Also, buf[n-1] uses the decremented n, so index is (n-1) == (n_read-2).
    //
    // If n_read < 2, the C program has UB. We avoid panics and choose a
    // deterministic approximation: treat the comparison as false.
    let eq = if buf.len() >= 2 {
        let idx = (n_read - 2) as usize;
        (buf[0] == buf[idx]) as isize
    } else {
        0
    };

    let cond = (n.wrapping_add(eq) & 1) != 0;
    if cond {
        print!("First\n");
    } else {
        print!("Second\n");
    }
}
