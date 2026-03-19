use std::io::{self, Read};

const LEN: usize = 128;

fn convert(char_num: &str, int_num: &mut [u8; LEN]) {
    // Store digits in reverse order (least significant digit at index 0),
    // matching the C code's behavior.
    for (j, b) in char_num.as_bytes().iter().rev().enumerate() {
        int_num[j] = b - b'0';
    }
}

fn main() {
    // Read entire stdin and tokenize by whitespace, similar to scanf("%d %s %s ...").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Spec precondition: present and parseable.
    let data_set: i32 = match it.next() {
        Some(tok) => tok.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    // Process exactly data_set test cases (spec). If tokens are missing, behavior is
    // unspecified by spec; we stop to avoid inventing behavior.
    for _ in 0..data_set.max(0) {
        let first = match it.next() {
            Some(s) => s,
            None => break,
        };
        let second = match it.next() {
            Some(s) => s,
            None => break,
        };

        if first.len() > 80 || second.len() > 80 {
            out.push_str("overflow\n");
            continue;
        }

        let mut fnum = [0u8; LEN];
        let mut snum = [0u8; LEN];
        let mut sum = [0u8; LEN];

        convert(first, &mut fnum);
        convert(second, &mut snum);

        for i in 0..LEN {
            sum[i] = fnum[i] + snum[i];
        }
        for i in 0..LEN {
            if sum[i] >= 10 {
                sum[i] %= 10;
                if i + 1 < LEN {
                    sum[i + 1] += 1;
                }
                // If i+1 == LEN, that would exceed fixed capacity; outside preconditions.
            }
        }

        // Count leading zeros from the most significant end, but keep at least one digit.
        let mut k: usize = 0;
        let mut j: isize = (LEN as isize) - 1;
        while j >= 0 {
            let idx = j as usize;
            if sum[idx] == 0 && idx != 0 {
                k += 1;
                j -= 1;
            } else {
                break;
            }
        }

        // Overflow if the result needs more than 80 digits.
        // This matches the C logic: overflow when k < LEN - 80.
        if k < LEN - 80 {
            out.push_str("overflow\n");
        } else {
            // Print from most significant non-zero down to 0.
            for l in (0..=(LEN - k - 1)).rev() {
                out.push(char::from(b'0' + sum[l]));
            }
            out.push('\n');
        }
    }

    print!("{out}");
}
