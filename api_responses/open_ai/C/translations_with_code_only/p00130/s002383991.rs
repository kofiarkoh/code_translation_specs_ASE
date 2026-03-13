use std::io::{self, Read, Write};

fn is_space(b: u8) -> bool {
    matches!(b, b' ' | b'\n' | b'\t' | b'\r' | 0x0b | 0x0c)
}

fn parse_int(buf: &[u8], pos: &mut usize) -> Option<i32> {
    let len = buf.len();
    // Skip leading whitespace (scanf("%d") skips whitespace)
    while *pos < len && is_space(buf[*pos]) {
        *pos += 1;
    }
    if *pos >= len {
        return None;
    }
    // Optional sign
    let mut sign = 1i32;
    if buf[*pos] == b'+' {
        *pos += 1;
    } else if buf[*pos] == b'-' {
        sign = -1;
        *pos += 1;
    }
    // Parse digits
    let mut any = false;
    let mut val: i64 = 0;
    while *pos < len {
        let b = buf[*pos];
        if b >= b'0' && b <= b'9' {
            any = true;
            val = val * 10 + (b - b'0') as i64;
            *pos += 1;
        } else {
            break;
        }
    }
    if !any {
        // scanf would fail; but for our inputs we assume a valid integer is present
        return None;
    }
    // Emulate conversion to C int (i32). We'll cast (may truncate if out-of-range).
    Some((val as i32).wrapping_mul(sign))
}

fn main() {
    // Read entire stdin into buffer so we can emulate the C code's sequential character consumption exactly.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).expect("read stdin");

    let mut pos: usize = 0;
    let len = input.len();

    // Parse n using scanf("%d",&n) semantics.
    let n = match parse_int(&input, &mut pos) {
        Some(v) => v,
        None => {
            // No integer found; nothing to do.
            return;
        }
    };

    // emulate the getchar() after scanf("%d",&n);
    if pos < len {
        pos += 1;
    }

    let mut stdout = io::stdout();
    let mut out_buf = Vec::new();

    // c is a global char in the C code and is zero-initialized.
    let mut c: u8 = 0;

    for _ in 0..n {
        let mut p: i32 = 13;
        let mut l = [0u8; 27];

        // Inner loop: for(; scanf("%c",&l[p])!=-1; )
        loop {
            // emulate scanf("%c",&l[p])
            if pos >= len {
                break; // scanf would return EOF (-1), loop ends
            }
            // Only write into l if p index is within bounds 0..26.
            // The original C code would have undefined behavior if out-of-bounds;
            // we avoid UB by ignoring out-of-range writes.
            if (0..27).contains(&p) {
                l[p as usize] = input[pos];
            }
            pos += 1;

            // emulate ~scanf("%c",&c);
            // If there's another byte, scanf succeeds and updates c and consumes input.
            // If not, scanf would return EOF and c would remain unchanged (it's global),
            // so we leave c as-is and do not advance pos.
            if pos < len {
                c = input[pos];
                pos += 1;
            } else {
                // c remains unchanged
            }

            if c == b'-' || c == b'<' {
                // getchar() => consume one more char if available; ignore the value.
                if pos < len {
                    pos += 1;
                }
                if c == b'-' {
                    p = p.wrapping_add(1);
                } else {
                    p = p.wrapping_sub(1);
                }
            } else {
                break;
            }
        }

        // Find first index j with l[j] != 0
        let mut j_opt: Option<usize> = None;
        for j in 0..27usize {
            if l[j] != 0 {
                j_opt = Some(j);
                break;
            }
        }

        if let Some(j) = j_opt {
            // find end (next zero or end)
            let mut k = j;
            while k < 27 && l[k] != 0 {
                k += 1;
            }
            out_buf.extend_from_slice(&l[j..k]);
            out_buf.push(b'\n'); // puts adds a newline
        } else {
            // No non-zero found: puts of empty string => prints just newline.
            out_buf.push(b'\n');
        }
    }

    // Write everything to stdout
    stdout.write_all(&out_buf).expect("write stdout");
}
