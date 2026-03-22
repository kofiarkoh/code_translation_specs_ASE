use std::io::{self, Read};

fn find_not_zero(diff: &[u8], start: usize, n: usize) -> Option<usize> {
    for i in start..n {
        if diff[i] != 0 {
            return Some(i);
        }
    }
    None
}

fn rotate_count(diff: &[u8], n: usize, acc: i32, min_: &mut i32) {
    let pos = match find_not_zero(diff, 0, n) {
        None => {
            if *min_ > acc {
                *min_ = acc; // monotonic decrease as better solutions are found
            }
            return;
        }
        Some(p) => p,
    };

    let offs = diff[pos]; // 1..9
    let mut diff2 = [0u8; 10];
    // C code copies 10 bytes regardless of n; we mirror that fixed-size buffer behavior.
    diff2.copy_from_slice(&diff[..10]);

    for i in pos..n {
        // diff2[i] = (diff2[i] + 10 - offs) % 10
        diff2[i] = (diff2[i] + 10 - offs) % 10;
        rotate_count(&diff2, n, acc + 1, min_);
    }
}

fn calc_diff(init: &[u8], target: &[u8], n: usize) -> [u8; 10] {
    let mut diff = [0u8; 10];
    for i in 0..n {
        let a = init[i] - b'0';
        let b = target[i] - b'0';
        diff[i] = (b + 10 - a) % 10;
    }
    diff
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let Some(n_tok) = it.next() else { break };
        let n: usize = n_tok.parse().unwrap();
        if n == 0 {
            break;
        }

        let init_s = it.next().unwrap();
        let target_s = it.next().unwrap();

        // Preconditions guarantee length <= 10 and exact lengths; we rely on that.
        let init = init_s.as_bytes();
        let target = target_s.as_bytes();

        if init[..n] == target[..n] {
            out.push_str("0\n");
        } else {
            let mut min_ = 99i32;
            let diff = calc_diff(init, target, n);
            rotate_count(&diff, n, 0, &mut min_);
            out.push_str(&format!("{min_}\n"));
        }
    }

    print!("{out}");
}
