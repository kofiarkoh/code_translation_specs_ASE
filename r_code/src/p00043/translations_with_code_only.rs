use std::io::{self, Read};

fn main() {
    // Read all input and iterate over whitespace-separated tokens (matches scanf "%s" behavior)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut out = String::new();

    for token in input.split_whitespace() {
        // Corresponds to C arrays: p[14], result[14], used[14], ans_buf[9], top
        // Use i8 for small integer digits, bool for used, Vec<i8> for ans_buf.
        let mut p = [0i8; 14];
        let mut result = [0i8; 14];
        let mut used = [false; 14];
        let mut ans_buf: Vec<i8> = Vec::with_capacity(9);
        let mut solved = false;

        // init(): for i = 0..12: p[i] -= '0'; used[i] = 0; top = p[13] = used[13] = 0;
        // We emulate by parsing up to 13 characters; if token shorter, missing positions become 0 (as C would have undefined behavior).
        let bytes = token.as_bytes();
        for i in 0..13 {
            p[i] = if i < bytes.len() {
                // convert ASCII digit char to numeric value like (c - '0')
                (bytes[i] as i8) - ('0' as i8)
            } else {
                0
            };
            used[i] = false;
        }
        p[13] = 0;
        used[13] = false;
        // ans_buf/top empty

        // dupCh: return true if count >= 4
        let dup_ch = |p: &[i8; 14], n: i8| -> bool {
            let mut count = 0;
            for i in 0..13 {
                if p[i] == n {
                    count += 1;
                }
            }
            count >= 4
        };

        // triCh macro semantics:
        // returns true when the triple is INVALID (i.e., not all equal and not strictly descending by 1)
        let tri_ch_invalid = |result: &[i8; 14], n: usize| -> bool {
            // accesses result[n-2], result[n-1], result[n]
            let a = result[n - 2];
            let b = result[n - 1];
            let c = result[n];
            if a == b && b == c {
                return false;
            }
            if a == b + 1 && b == c + 1 {
                return false;
            }
            true
        };

        // recursive calc function
        fn calc(
            n: usize,
            p: &mut [i8; 14],
            result: &mut [i8; 14],
            used: &mut [bool; 14],
            ans_buf: &mut Vec<i8>,
            solved: &mut bool,
            tri_ch_invalid: &dyn Fn(&[i8; 14], usize) -> bool,
        ) {
            if *solved {
                return;
            }
            if n == 2 && (result[0] != result[1]) {
                return;
            }
            if n == 5 && tri_ch_invalid(result, 4) {
                return;
            }
            if n == 8 && tri_ch_invalid(result, 7) {
                return;
            }
            if n == 11 && tri_ch_invalid(result, 10) {
                return;
            }
            if n < 14 {
                for i in 0..14 {
                    if *solved {
                        break;
                    }
                    if used[i] {
                        continue;
                    }
                    used[i] = true;
                    result[n] = p[i];
                    calc(
                        n + 1,
                        p,
                        result,
                        used,
                        ans_buf,
                        solved,
                        tri_ch_invalid,
                    );
                    used[i] = false;
                }
            } else {
                // n == 14
                if tri_ch_invalid(result, 13) {
                    return;
                }
                *solved = true;
                ans_buf.push(p[13]);
            }
        }

        // solve(): try i = 1..9
        for i in 1..=9 {
            if dup_ch(&p, i) {
                continue;
            }
            p[13] = i;
            solved = false;
            // call calc(0)
            calc(
                0,
                &mut p,
                &mut result,
                &mut used,
                &mut ans_buf,
                &mut solved,
                &tri_ch_invalid,
            );
        }

        // printResult(): if no answers print "0", else print digits separated by spaces, then newline
        if ans_buf.is_empty() {
            out.push_str("0\n");
        } else {
            // first without leading space
            let first_char = (ans_buf[0] as u8 + b'0') as char;
            out.push(first_char);
            for v in ans_buf.iter().skip(1) {
                out.push(' ');
                out.push(((*v) as u8 + b'0') as char);
            }
            out.push('\n');
        }
    }

    // Write all output at once
    print!("{}", out);
}
