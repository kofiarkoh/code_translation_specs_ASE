use std::io::{self, Read};

fn partition_sum(paper: &str, n: usize, mask: u32) -> i32 {
    // Mirrors C's func(): interpret bits 0..n-1 as "cut after position i".
    // Bit n-1 is always set by the caller, ensuring the last group is closed.
    let bytes = paper.as_bytes();
    let mut sum: i32 = 0;
    let mut start: usize = 0;

    for i in 0..n {
        if (mask & (1u32 << i)) != 0 {
            // group is paper[start..=i]
            let part = &bytes[start..=i];
            // C uses atoi; spec says behavior follows underlying conversion.
            // Input is intended digits, so parse as base-10 i32.
            let val: i32 = std::str::from_utf8(part).unwrap().parse().unwrap();
            sum += val;
            start = i + 1;
        }
    }
    sum
}

fn main() {
    // Read all input tokens (behavior unspecified on malformed input, as in C).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let target_opt = it.next();
        if target_opt.is_none() {
            break;
        }
        let target: i32 = target_opt.unwrap().parse().unwrap();
        let paper = it.next().unwrap(); // per spec, each instance supplies both tokens

        if target == 0 {
            break; // terminating instance produces no output
        }

        let n = paper.len();
        // C uses a fixed buffer paper[10]; spec assumes it fits. We don't enforce.
        // Also, C enumerates i from 1<<(n-1) to (1<<n)-1, ensuring last bit set.
        let mut best: i32 = -1;
        let mut best_mask: u32 = 0;
        let mut cnt: i32 = 0;

        // For n>=1, (1<<(n-1)) is valid. Spec guarantees nonempty string.
        let start_mask: u32 = 1u32 << (n - 1);
        let end_mask: u32 = 1u32 << n;

        for mask in start_mask..end_mask {
            let t = partition_sum(paper, n, mask);
            if t <= target {
                if best < t {
                    best = t;
                    best_mask = mask;
                    cnt = 1;
                } else if best == t {
                    cnt += 1;
                }
            }
        }

        if cnt != 1 {
            if cnt == 0 {
                out.push_str("error\n");
            } else {
                out.push_str("rejected\n");
            }
        } else {
            out.push_str(&format!("{} ", best));
            // Print digits with spaces at cut positions; newline after last group.
            let bytes = paper.as_bytes();
            for i in 0..n {
                out.push(bytes[i] as char);
                if (best_mask & (1u32 << i)) != 0 {
                    if i == n - 1 {
                        out.push('\n');
                    } else {
                        out.push(' ');
                    }
                }
            }
        }
    }

    print!("{out}");
}
