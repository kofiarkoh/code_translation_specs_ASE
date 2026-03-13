use std::io::{self, Read, Write};

fn c_strlen(arr: &[u8; 33]) -> usize {
    // Find first NUL (0) like C's '\0' termination
    arr.iter().position(|&b| b == 0).unwrap_or(arr.len())
}

fn c_strequal(a: &[u8; 33], b: &[u8; 33]) -> bool {
    let la = c_strlen(a);
    let lb = c_strlen(b);
    if la != lb {
        return false;
    }
    a[..la] == b[..lb]
}

fn main() {
    // s[1000][33] zero-initialized
    let mut s = [[0u8; 33]; 1000];

    let mut i: usize = 0;
    let mut count: usize = 0;
    let mut max: i32 = 0;
    let mut ans1: usize = 100;
    let mut ans2: usize = 100;

    // Read all stdin bytes and process until first newline (like repeated scanf("%c",...))
    let mut input = Vec::new();
    if io::stdin().read_to_end(&mut input).is_err() {
        // If read fails, behave as if no input -> print empty strings like C would when nothing was read?
        // We'll just proceed with defaults.
    }

    for &ch in &input {
        if ch == b'\n' {
            break;
        }
        if ch == b' ' {
            // increment word counter and reset position
            count = count.saturating_add(1); // keep safe; C would overflow UB if >999
            i = 0;
        }

        // Only operate within the original buffer bounds to avoid UB; this preserves typical usage.
        if count < 1000 {
            // Update s[count][i] if i in bounds
            if i < 33 {
                s[count][i] = ch;
            }
            // Update max/ans2 using the same expression as C: if(max < 1 + i) ...
            // Use i as usize converted to i32
            let candidate_len = (i as i32) + 1;
            if max < candidate_len {
                max = candidate_len;
                ans2 = count;
            }
        } else {
            // If count >= 1000, the original C code would write out-of-bounds (UB).
            // We choose to ignore further writes to keep Rust safe.
        }

        if ch != b' ' {
            i = i.saturating_add(1); // keep safe from overflow
        }
    }

    // Now reproduce the duplicate-counting loop exactly as the C code:
    max = 0;
    // Outer loop: for(i=0;i<count;i++)
    for outer in 0..count {
        // inner: for(j=i+1,c=0;j<count;j++)
        let mut c = 0;
        let mut j = outer + 1;
        while j < count {
            if c_strequal(&s[outer], &s[j]) {
                c += 1;
            }
            if max < c {
                max = c;
                ans1 = outer;
            }
            j += 1;
        }
    }

    // Print s[ans1] and s[ans2] as C would with "%s %s\n"
    // That means writing bytes up to first NUL, then a space, then next bytes up to first NUL, then newline.
    let mut out = io::stdout();

    // Helper to write a C-string (bytes up to first NUL)
    let write_cstr = |w: &mut dyn Write, arr: &[u8; 33]| -> io::Result<()> {
        let len = c_strlen(arr);
        if len > 0 {
            w.write_all(&arr[..len])?;
        }
        Ok(())
    };

    // ans1 and ans2 are indices into s (may be 100 as initialized)
    // Ensure we don't index out of bounds (ans may be <= 999)
    let a1 = if ans1 < 1000 { ans1 } else { 100 }; // keep within bounds; mirrors C's behaviour of indexing s[100] if ans1==100
    let a2 = if ans2 < 1000 { ans2 } else { 100 };

    let _ = write_cstr(&mut out, &s[a1]);
    let _ = out.write_all(b" ");
    let _ = write_cstr(&mut out, &s[a2]);
    let _ = out.write_all(b"\n");
}
