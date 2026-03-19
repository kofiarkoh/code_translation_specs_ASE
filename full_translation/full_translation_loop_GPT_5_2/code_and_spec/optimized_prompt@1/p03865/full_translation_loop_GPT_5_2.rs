use std::io::{self, Read};

fn main() {
    // Fixed buffer size per spec: 114514 characters (bytes).
    let mut buf = vec![0u8; 114_514];

    // Read up to 114514 bytes from stdin (like C read(0, buf, 114514)).
    let n = match io::stdin().read(&mut buf) {
        Ok(n) => n,
        Err(_) => 0, // Spec says error handling not inferable; keep behavior simple.
    };

    // Spec precondition: input length >= 1 character.
    // The C code has undefined behavior if n == 0; we assume precondition holds.
    if n == 0 {
        return;
    }

    // Compare first and last byte of the read sequence.
    // Note: C code decrements n and then compares buf[0] with buf[n-1],
    // which effectively compares first with last of the original read.
    let first = buf[0];
    let last = buf[n - 1];

    if first == last {
        println!("First");
    } else {
        println!("Second");
    }
}
