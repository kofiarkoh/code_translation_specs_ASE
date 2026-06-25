use std::io::{self, Read};

const LEN: usize = 128;
const MAX_DIGITS: usize = 80;

fn convert(char_num: &str, int_num: &mut [u8; LEN]) {
    // Stores digits in reverse order (least significant digit at index 0),
    // matching the C code's behavior.
    for (j, b) in char_num.as_bytes().iter().rev().enumerate() {
        // Spec assumes digits only; behavior for non-digits is unspecified.
        int_num[j] = b.wrapping_sub(b'0');
    }
}

fn main() {
    // Read entire stdin, then split by whitespace (matches scanf("%d"/"%s") tokenization).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Preconditions say this exists; if not, behavior is not well-defined.
    let data_set: usize = match it.next() {
        Some(tok) => tok.parse::<usize>().unwrap_or(0),
        None => return,
    };

    let mut out = String::new();

    for _case_idx in 0..data_set {
        // Spec: if tokens are missing, behavior is not well-defined; we stop.
        let first = match it.next() {
            Some(s) => s,
            None => break,
        };
        let second = match it.next() {
            Some(s) => s,
            None => break,
        };

        if first.len() > MAX_DIGITS || second.len() > MAX_DIGITS {
            out.push_str("overflow\n");
            continue;
        }

        let mut fnum = [0u8; LEN];
        let mut snum = [0u8; LEN];
        let mut sum = [0u8; LEN];

        convert(first, &mut fnum);
        convert(second, &mut snum);

        for i in 0..LEN {
            sum[i] = fnum[i].wrapping_add(snum[i]);
        }

        // Carry propagation. The C code writes sum[i+1] for i=LEN-1 if needed,
        // which would be out-of-bounds; under the spec's constraints (<=80 digits
        // operands), the carry cannot reach beyond index 80, so we keep it safe.
        for i in 0..(LEN - 1) {
            if sum[i] >= 10 {
                sum[i] %= 10;
                sum[i + 1] = sum[i + 1].wrapping_add(1);
            }
        }

        // Count leading zeros from the most significant end, but keep at least one digit.
        let mut k = 0usize;
        let mut j = LEN - 1;
        while j > 0 && sum[j] == 0 {
            k += 1;
            j -= 1;
        }

        // C logic: overflow if k < LEN - 80, i.e., if highest non-zero digit index >= 80.
        if k < LEN - MAX_DIGITS {
            out.push_str("overflow\n");
        } else {
            // Print from most significant non-zero down to 0 (or just "0").
            for l in (0..=(LEN - k - 1)).rev() {
                out.push(char::from(b'0' + sum[l]));
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
