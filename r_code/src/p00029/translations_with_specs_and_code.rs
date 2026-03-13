use std::io::{self, Read, Write};

fn cstr_len(buf: &[u8; 33]) -> usize {
    match buf.iter().position(|&b| b == 0) {
        Some(pos) => pos,
        None => 33,
    }
}

fn cstr_eq(a: &[u8; 33], b: &[u8; 33]) -> bool {
    let la = cstr_len(a);
    let lb = cstr_len(b);
    if la != lb {
        return false;
    }
    &a[..la] == &b[..lb]
}

fn cstr_to_string(buf: &[u8; 33]) -> String {
    let len = cstr_len(buf);
    // Use lossless conversion for valid UTF-8, and replacement for invalid bytes,
    // mimicking C's %s printing of raw bytes.
    String::from_utf8_lossy(&buf[..len]).into_owned()
}

fn main() {
    // Fixed-size token buffer: 1000 tokens, each up to 32 bytes + NUL (33).
    let mut s: [[u8; 33]; 1000] = [[0u8; 33]; 1000];

    let mut i: usize = 0;
    let mut max: i32 = 0;
    let mut ans1: usize = 100;
    let mut ans2: usize = 100;
    let mut count: usize = 0;

    // Read stdin byte-by-byte until newline.
    let mut stdin_buf = Vec::new();
    // Read all available input; we'll process until the first newline.
    io::stdin()
        .read_to_end(&mut stdin_buf)
        .expect("failed to read stdin");

    for &byte in &stdin_buf {
        if byte == b'\n' {
            break;
        }

        // The original C code assumes inputs satisfy preconditions (counts/lengths).
        // We keep debug assertions to catch violations in debug builds but otherwise
        // follow the specification that inputs fit the fixed buffers.
        debug_assert!(count < 1000, "token count exceeded 1000");
        debug_assert!(i < 33, "token length exceeded 32");

        if byte == b' ' {
            count = count.wrapping_add(1);
            i = 0;
        }

        // Write the byte into the buffer slot (mimics the C code behavior).
        // This matches the original ordering: when a space is read, count and i are updated
        // and then the code writes s[count][i] = ch.
        s[count][i] = byte;

        // Update longest-token tracking exactly as in the C code: compare max with 1+i.
        if max < (1 + i) as i32 {
            max = (1 + i) as i32;
            ans2 = count;
        }

        if byte != b' ' {
            i = i.wrapping_add(1);
        }
    }

    // Now compute duplicates among tokens with indices [0, count-1], matching the C logic.
    max = 0;
    for idx in 0..count {
        let mut c: i32 = 0;
        for j in (idx + 1)..count {
            if cstr_eq(&s[idx], &s[j]) {
                c += 1;
            }
            if max < c {
                max = c;
                ans1 = idx;
            }
        }
    }

    // Prepare output strings (null-terminated slices converted to String).
    // ans1 and ans2 are initialized to 100 (as in the original); index 100 is within bounds.
    let out1 = cstr_to_string(&s[ans1]);
    let out2 = cstr_to_string(&s[ans2]);

    // Print exactly: "<most-duplicated-token> <longest-token>\n"
    let stdout = io::stdout();
    let mut handle = stdout.lock();
    writeln!(handle, "{} {}", out1, out2).expect("failed to write output");
}
