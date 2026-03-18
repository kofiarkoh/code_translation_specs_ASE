use std::io::{self, Read};

const LEN: usize = 128;
const MAX_INPUT_LEN: usize = 80;

fn convert(char_num: &str, int_num: &mut [u8; LEN]) {
    // Store digits in reverse order (least significant digit at index 0)
    for (j, b) in char_num.as_bytes().iter().rev().enumerate() {
        int_num[j] = b.wrapping_sub(b'0');
    }
}

fn main() {
    // Read all input tokens (matches scanf("%d") then repeated scanf("%s"))
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let data_set: usize = match it.next() {
        Some(t) => t.parse().unwrap_or(0),
        None => return,
    };

    let mut out = String::new();

    for _ in 0..data_set {
        let first = match it.next() {
            Some(s) => s,
            None => break, // C code would block; with finite input, we stop.
        };
        let second = match it.next() {
            Some(s) => s,
            None => break,
        };

        // Spec: if either input length exceeds 80 => overflow and continue
        if first.len() > MAX_INPUT_LEN || second.len() > MAX_INPUT_LEN {
            out.push_str("overflow\n");
            continue;
        }

        // C assumes digit strings; spec says non-negative integers.
        // We preserve behavior for valid inputs.
        let mut fnum = [0u8; LEN];
        let mut snum = [0u8; LEN];
        let mut sum = [0u8; LEN];

        convert(first, &mut fnum);
        convert(second, &mut snum);

        for i in 0..LEN {
            sum[i] = fnum[i] + snum[i];
        }

        // Carry propagation. In C, this writes sum[i+1] for i=LEN-1 (out of bounds).
        // We avoid that UB by only propagating carries up to LEN-2; any carry beyond LEN-1
        // is treated as overflow (consistent with the "defined limit" concept).
        let mut carry_out = false;
        for i in 0..LEN {
            if sum[i] >= 10 {
                sum[i] %= 10;
                if i + 1 < LEN {
                    sum[i + 1] = sum[i + 1].saturating_add(1);
                } else {
                    carry_out = true;
                }
            }
        }

        // Count leading zeros from the most significant end (index LEN-1 down to 1)
        let mut k = 0usize;
        let mut j = LEN - 1;
        while j > 0 && sum[j] == 0 {
            k += 1;
            j -= 1;
        }

        // C's overflow condition: if k < LEN - 80 => overflow
        // This effectively means: if the result needs more than 80 digits.
        // Also treat carry beyond LEN as overflow.
        if carry_out || k < LEN - MAX_INPUT_LEN {
            out.push_str("overflow\n");
        } else {
            // Print from most significant non-zero down to 0 (or just "0")
            for l in (0..=(LEN - k - 1)).rev() {
                out.push(char::from(b'0' + sum[l]));
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
