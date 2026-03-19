use std::io::{self, Read, Write};

fn main() {
    // Read up to 10,000 bytes (spec constraint). If more are provided, we only take the first 10,000,
    // matching the C code's fixed buffer read.
    let mut input = Vec::new();
    io::stdin().read_to_end(&mut input).unwrap();
    if input.len() > 10_000 {
        input.truncate(10_000);
    }

    let mut sz: usize = 0;
    let mut out = Vec::<u8>::with_capacity(20_000);

    // C: while(buf[sz++]>='0');
    // This skips initial digits (if any) and stops at first non-digit (< '0' in C's logic).
    // Note: C compares to '0' only, not '9', so any byte >= '0' is treated as "digit-like".
    // We preserve that exact behavior.
    while sz < input.len() && input[sz] >= b'0' {
        sz += 1;
    }
    // In C, sz is post-incremented in the condition; after the loop it points one past the
    // first non-digit. We already advanced to the non-digit, so emulate the post-increment:
    if sz < input.len() {
        sz += 1;
    }

    // C: while(buf[sz]>='0') { ... }
    while sz < input.len() && input[sz] >= b'0' {
        // Parse N: for(;;){ k=buf[sz++]; if(k<'0')break; N=N*10+k-'0'; }
        let mut n: i32 = 0;
        loop {
            if sz >= input.len() {
                break;
            }
            let k = input[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            n = n.wrapping_mul(10).wrapping_add((k - b'0') as i32);
        }

        // Parse D similarly
        let mut d: i32 = 0;
        loop {
            if sz >= input.len() {
                break;
            }
            let k = input[sz];
            sz += 1;
            if k < b'0' {
                break;
            }
            d = d.wrapping_mul(10).wrapping_add((k - b'0') as i32);
        }

        // C: int ans=N&1?127*--N+D:127*N-D;
        // Preserve C's side-effect: if N is odd, pre-decrement N before multiplying.
        let ans: i32 = if (n & 1) != 0 {
            n = n.wrapping_sub(1);
            127i32.wrapping_mul(n).wrapping_add(d)
        } else {
            127i32.wrapping_mul(n).wrapping_sub(d)
        };

        // C prints ans as unsigned decimal digits without sign handling.
        // It also fails for ans==0 (prints nothing) and for negative values (nonsense).
        // Spec says inputs are non-negative integers; we keep the C behavior for ans==0.
        let mut x = ans;
        let mut r = [0u8; 10];
        let mut ri: usize = 10;
        loop {
            let digit = (x % 10) as i32;
            r[ri - 1] = (digit as u8).wrapping_add(b'0');
            ri -= 1;
            x /= 10;
            if x == 0 {
                break;
            }
        }
        out.extend_from_slice(&r[ri..10]);
        out.push(b'\n');

        // Maintain the output size invariant (spec). The C code doesn't enforce it,
        // but the spec states it is maintained; we avoid growing beyond 20,000.
        if out.len() > 20_000 {
            out.truncate(20_000);
            break;
        }
    }

    io::stdout().write_all(&out).unwrap();
}
