use std::io::{self, Read, Write};

fn main() {
    // C globals: zero-initialized.
    let mut buf = [0u8; 10000];
    let mut out = [0u8; 20000];
    let mut r = [0u8; 10];
    let mut sz: usize = 0;
    let mut outsz: usize = 0;

    // Emulate `read(0, buf, 10000)`:
    // - Reads up to 10000 bytes.
    // - If fewer bytes are read, the rest remain 0 (already true).
    let mut stdin = io::stdin();
    let mut tmp = Vec::new();
    stdin.read_to_end(&mut tmp).unwrap();
    let nread = tmp.len().min(10000);
    buf[..nread].copy_from_slice(&tmp[..nread]);

    // while(buf[sz++] >= '0');
    while buf[sz] >= b'0' {
        sz += 1;
        // In C, sz++ happens even when condition fails; here we only increment on true.
        // But the loop stops when condition is false; in C, sz is incremented once more.
        // So we need to emulate post-increment: increment after checking.
    }
    // Emulate the extra increment from `sz++` in the condition evaluation.
    sz += 1;

    // while(buf[sz] >= '0')
    while sz < buf.len() && buf[sz] >= b'0' {
        // Parse N
        let mut n: i32 = 0;
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            // C `int` arithmetic; use wrapping to emulate overflow behavior.
            n = n.wrapping_mul(10).wrapping_add((k - b'0') as i32);
        }

        // Parse D
        let mut d: i32 = 0;
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            d = d.wrapping_mul(10).wrapping_add((k - b'0') as i32);
        }

        // int ans = N&1 ? 127*--N + D : 127*N - D;
        let ans: i32 = if (n & 1) != 0 {
            n = n.wrapping_sub(1); // --N
            127i32.wrapping_mul(n).wrapping_add(d)
        } else {
            127i32.wrapping_mul(n).wrapping_sub(d)
        };

        // Convert ans to decimal like the C code:
        // do { r[--ri] = ans%10 + '0'; } while(ans/=10);
        // Note: In C, negative values would produce non-digit bytes; we preserve that behavior.
        let mut ri: usize = 10;
        let mut a = ans;
        loop {
            ri -= 1;
            let digit = a.wrapping_rem(10); // matches C's `%` on signed ints (trunc toward zero)
            r[ri] = (digit.wrapping_add(b'0' as i32)) as u8;
            a = a.wrapping_div(10); // matches C's `/` on signed ints (trunc toward zero)
            if a == 0 {
                break;
            }
        }

        while ri < 10 {
            if outsz < out.len() {
                out[outsz] = r[ri];
            }
            outsz += 1;
            ri += 1;
        }
        if outsz < out.len() {
            out[outsz] = b'\n';
        }
        outsz += 1;
    }

    // write(1, out, outsz);
    // If outsz exceeds 20000, C would read past `out` (UB). We clamp to avoid panic,
    // but preserve output for the in-bounds portion.
    let to_write = outsz.min(out.len());
    let mut stdout = io::stdout();
    stdout.write_all(&out[..to_write]).unwrap();
}
