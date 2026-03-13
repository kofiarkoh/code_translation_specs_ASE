use std::io::{self, Read};

fn main() {
    // Read entire stdin so we can emulate scanf("%c,%c", &a, &b) semantics,
    // including leaving a non-matching character unread on a matching failure.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    let len = input.len();
    let mut pos = 0usize;

    // ball positions: ball[0]=1 means ball under A, etc.
    let mut ball = [1i32, 0, 0];

    // Variables corresponding to the C program.
    // Initialize to a deterministic value; the original C had them
    // uninitialized, but typical inputs assign them before use.
    let mut a: u8 = 0;
    let mut b: u8 = 0;
    let mut x: usize = 0;
    let mut y: usize = 0;

    loop {
        // Emulate scanf("%c,%c",&a,&b)
        // Return EOF only if we cannot read the first character (no assignments).
        if pos >= len {
            break; // scanf would return EOF here -> loop ends
        }

        // Read first %c (always consumes one byte if available)
        a = input[pos];
        pos += 1;

        // Now attempt to match the literal ','.
        // If there's no next byte (EOF), scanf would return 1 (one assignment)
        // and the non-existing comma isn't consumed.
        if pos >= len {
            // EOF after assigning a: behave like scanf returning 1 (loop continues)
            // b remains unchanged.
        } else if input[pos] != b',' {
            // Matching failure on literal comma: do NOT consume the non-matching byte.
            // That non-matching byte will be the next a in the subsequent iteration.
            // This matches scanf behavior: a was consumed/assigned, the mismatching
            // character is left in the stream.
        } else {
            // Comma matched and consumed
            pos += 1;
            // Attempt to read second %c
            if pos >= len {
                // EOF after comma: scanf would have returned 1 (only a assigned).
                // b remains unchanged.
            } else {
                b = input[pos];
                pos += 1;
            }
        }

        // Map chars to indices like the C switch statements.
        match a {
            b'A' => x = 0,
            b'B' => x = 1,
            b'C' => x = 2,
            _ => {
                // If a is none of A/B/C, the original C switch left x unchanged.
                // We preserve that behavior by not modifying x.
            }
        }
        match b {
            b'A' => y = 0,
            b'B' => y = 1,
            b'C' => y = 2,
            _ => {
                // Similarly, if b not A/B/C, leave y unchanged.
            }
        }

        // Swap ball[x] and ball[y]
        let temp = ball[x];
        ball[x] = ball[y];
        ball[y] = temp;

        // Continue loop; termination occurs only when scanf would return EOF
        // (i.e., when there is no character available to assign to a).
    }

    // Print result exactly as in the C program: single letter followed by newline.
    if ball[0] == 1 {
        println!("A");
    } else if ball[1] == 1 {
        println!("B");
    } else {
        println!("C");
    }
}
