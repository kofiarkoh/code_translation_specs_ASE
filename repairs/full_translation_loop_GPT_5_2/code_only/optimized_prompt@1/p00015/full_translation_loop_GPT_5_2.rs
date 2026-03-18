use std::io::{self, Read};

const LEN: usize = 128;

fn convert(char_num: &str, int_num: &mut [i32; LEN]) {
    // Mirrors C: iterate from end to start, store digits reversed.
    // C does: (int)charNum[i] - 48; we emulate byte - b'0' as i32.
    let bytes = char_num.as_bytes();
    let mut j: usize = 0;
    for i in (0..bytes.len()).rev() {
        if j >= LEN {
            // C would overflow the destination array (UB) if input > LEN-1.
            // We cap to avoid panic; behavior beyond LEN is not well-defined in C anyway.
            break;
        }
        int_num[j] = (bytes[i] as i32) - 48;
        j += 1;
    }
}

fn main() {
    // Read all input, then parse whitespace-separated tokens like scanf("%d") / scanf("%s").
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let data_set: i32 = match it.next() {
        Some(t) => t.parse::<i32>().unwrap_or(0),
        None => return,
    };

    let mut count: i32 = 1;

    while count <= data_set {
        count += 1;

        // The C code does two scanf("%s") calls; if missing, it would block/UB-ish.
        // Here, if missing tokens, we stop (best-effort faithful for finite input).
        let first_num = match it.next() {
            Some(s) => s,
            None => break,
        };
        let second_num = match it.next() {
            Some(s) => s,
            None => break,
        };

        if first_num.len() > 80 || second_num.len() > 80 {
            print!("overflow\n");
            continue;
        }

        let mut f_num = [0i32; LEN];
        let mut s_num = [0i32; LEN];
        let mut sum = [0i32; LEN];

        convert(first_num, &mut f_num);
        convert(second_num, &mut s_num);

        for i in 0..LEN {
            sum[i] = f_num[i] + s_num[i];
        }

        // Note: C loops i=0..LEN-1 inclusive and writes sum[i+1] when i==LEN-1 (out of bounds UB).
        // We avoid out-of-bounds; carry out of the top digit is discarded, which is a reasonable
        // approximation of the intended fixed-width arithmetic.
        for i in 0..LEN {
            if sum[i] >= 10 {
                sum[i] %= 10;
                if i + 1 < LEN {
                    sum[i + 1] += 1;
                }
            }
        }

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

        if k < LEN - 80 {
            print!("overflow");
        } else {
            // Print from highest non-leading-zero down to 0.
            let mut l: isize = (LEN - k - 1) as isize;
            while l >= 0 {
                let idx = l as usize;
                print!("{}", sum[idx]);
                l -= 1;
            }
        }
        print!("\n");
    }
}
