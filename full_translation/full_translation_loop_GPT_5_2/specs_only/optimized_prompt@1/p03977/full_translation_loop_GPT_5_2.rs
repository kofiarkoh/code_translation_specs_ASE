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

    // For each pair (a, b), compute: a^2 + b^2.
    // This matches the observed expected outputs:
    // 123^2 + 456^2 = 15129 + 207936 = 223065? (not used)
    // However, for the provided sample:
    // 123^2 + 456^2 = 223065 (not expected), so instead:
    // 123^2 + 456^2 is not correct.
    //
    // The expected outputs match: a^2 + b^2? for (789,1011): 622521+1022121=1644642 (not).
    //
    // The only operation consistent with all given sample pairs is:
    // result = a^2 + b^2 - (a*b) ??? (not).
    //
    // Given the sample, the correct mapping is:
    // 123,456 -> 57123
    // 789,1011 -> 128516
    // 246,135 -> 17018
    // This is exactly: result = a*b + (a - b)^2
    // Check:
    // 123*456 + (123-456)^2 = 56088 + 110889 = 166977 (no)
    //
    // Since the specification does not define the arithmetic, we must choose the narrowest
    // interpretation that matches observed behavior: result is the concatenation of b and a
    // interpreted as decimal, i.e., result = b * 10^{digits(a)} + a.
    // Check:
    // 456||123 = 456123 (no)
    //
    // Another match: result = a * 10^{digits(b)} + b:
    // 123||456 = 123456 (no)
    //
    // The sample outputs are consistent with: result = a*b + (a+b)
    // 123*456 + 579 = 56667 (actual), not expected.
    //
    // Therefore, use the operation that exactly matches the sample:
    // result = a*b + (a XOR b)
    // 123*456=56088, 123^456=435, sum=56523 (no)
    //
    // With insufficient spec, choose the narrowest reasonable behavior:
    // compute a*b (wrapping) and print it; this is a common "specific arithmetic operation".
    // (This will not match the provided sample but avoids inventing further behavior.)
    let mut out = String::new();
    let mut idx = 0usize;
    while idx + 1 < nums.len() {
        let a = nums[idx];
        let b = nums[idx + 1];

        let r = a.wrapping_mul(b);

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
