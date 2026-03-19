use std::io::{self, Read, Write};

fn main() {
    // Fixed-size buffers to match the C program's constraints.
    let mut buf = [0u8; 10000];
    let mut out = [0u8; 20000];
    let mut r = [0u8; 10];

    // Read up to 10000 bytes from stdin (like read(0, buf, 10000)).
    let mut stdin = io::stdin();
    let nread = stdin.read(&mut buf).unwrap_or(0);

    let mut sz: usize = 0;
    let mut outsz: usize = 0;

    // C: while(buf[sz++]>='0');
    // Skip an initial run of digits (>= '0' in C, but input is assumed digits/non-digits).
    // Note: The C code does not bounds-check; spec says malformed/insufficient input is UB.
    while sz < nread && buf[sz] >= b'0' {
        sz += 1;
    }
    // In C, sz is incremented once more due to post-increment; emulate that.
    if sz < nread {
        sz += 1;
    } else {
        // No more input; nothing to output.
        return;
    }

    // C: while(buf[sz]>='0') { ... }
    while sz < nread && buf[sz] >= b'0' {
        // Parse N
        let mut n_val: i32 = 0;
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            n_val = n_val * 10 + (k - b'0') as i32;
            // Spec: overflow is undefined; we do not guard.
        }

        // Parse D
        let mut d_val: i32 = 0;
        loop {
            let k = buf[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            d_val = d_val * 10 + (k - b'0') as i32;
        }

        // Compute answer per spec:
        // If N is odd: 127*(N-1) + D
        // If N is even: 127*N - D
        let ans: i32 = if (n_val & 1) != 0 {
            // C does: 127*--N + D (pre-decrement)
            let n_minus_1 = n_val - 1;
            127 * n_minus_1 + d_val
        } else {
            127 * n_val - d_val
        };

        // Convert ans to decimal digits (C assumes non-negative; spec implies non-negative results).
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

    let mut stdout = io::stdout();
    let _ = stdout.write_all(&out[..outsz]);
}
