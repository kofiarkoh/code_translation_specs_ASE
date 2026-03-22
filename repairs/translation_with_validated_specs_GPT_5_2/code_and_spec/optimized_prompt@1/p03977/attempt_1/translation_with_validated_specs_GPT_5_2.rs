use std::io::{self, Read, Write};

fn main() {
    // Fixed-size buffers to mirror the C program's behavior and assumptions.
    let mut buf = [0u8; 10_000];
    let mut out = [0u8; 20_000];
    let mut r = [0u8; 10];

    // Read up to 10,000 bytes. Per spec/C, we do not use the returned count.
    let _ = io::stdin().read(&mut buf);

    let mut sz: usize = 0;
    let mut outsz: usize = 0;

    // Initial skip: consume a run of bytes >= '0', then consume exactly one terminator (< '0').
    // C: while(buf[sz++]>='0');
    while buf[sz] >= b'0' {
        sz += 1;
    }
    sz += 1; // consume the first following byte < '0'

    // Process records only while the current byte is digit-like (>= '0').
    while buf[sz] >= b'0' {
        let mut n: i32 = 0;
        let mut d: i32 = 0;

        // Parse N: consume digit-like bytes, stop at first < '0' (which is consumed).
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            n = n * 10 + (k - b'0') as i32;
        }

        // Parse D similarly.
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            d = d * 10 + (k - b'0') as i32;
        }

        // Compute ans per parity rule. Matches C: N&1?127*--N+D:127*N-D;
        let ans: i32 = if (n & 1) != 0 {
            n -= 1;
            127 * n + d
        } else {
            127 * n - d
        };

        // Render ans as unsigned-like decimal (no sign), using repeated %10 and /=10.
        // Assumption per spec: ans is non-negative and fits in 10 digits.
        let mut ri: usize = 10;
        let mut x = ans;
        loop {
            ri -= 1;
            r[ri] = (x % 10) as u8 + b'0';
            x /= 10;
            if x == 0 {
                break;
            }
        }

        while ri < 10 {
            out[outsz] = r[ri];
            outsz += 1;
            ri += 1;
        }
        out[outsz] = b'\n';
        outsz += 1;
    }

    let _ = io::stdout().write_all(&out[..outsz]);
}
