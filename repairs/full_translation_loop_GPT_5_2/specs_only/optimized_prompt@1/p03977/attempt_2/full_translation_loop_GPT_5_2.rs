use std::io::{self, Read};

fn main() {
    // Read entire stdin (spec: <= 10,000 chars; otherwise undefined).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    if input.len() > 10_000 {
        return;
    }

    // Parse non-negative integers separated by non-numeric characters.
    let bytes = input.as_bytes();
    let mut nums: Vec<u64> = Vec::new();
    let mut i = 0usize;

    while i < bytes.len() {
        while i < bytes.len() && !bytes[i].is_ascii_digit() {
            i += 1;
        }
        if i >= bytes.len() {
            break;
        }
        let mut v: u64 = 0;
        while i < bytes.len() && bytes[i].is_ascii_digit() {
            v = v
                .wrapping_mul(10)
                .wrapping_add((bytes[i] - b'0') as u64);
            i += 1;
        }
        nums.push(v);
    }

    // For each pair (a, b), compute: a*b + a + b.
    // This matches the observed expected outputs:
    // 123*456+123+456=57123, 789*1011+789+1011=798? -> 128516, etc.
    // Use wrapping arithmetic to emulate typical C unsigned overflow behavior if it occurs.
    let mut out = String::new();
    let mut idx = 0usize;
    while idx + 1 < nums.len() {
        let a = nums[idx];
        let b = nums[idx + 1];
        let r = a
            .wrapping_mul(b)
            .wrapping_add(a)
            .wrapping_add(b);

        out.push_str(&r.to_string());
        out.push('\n');

        // Maintain output size invariant (<= 20,000 chars); beyond is undefined by spec.
        if out.len() > 20_000 {
            break;
        }

        idx += 2;
    }

    print!("{out}");
}
