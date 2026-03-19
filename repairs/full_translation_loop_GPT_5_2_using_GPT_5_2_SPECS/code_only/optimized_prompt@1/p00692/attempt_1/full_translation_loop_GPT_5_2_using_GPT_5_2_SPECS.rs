use std::io::{self, Read};

static mut MAX_DEPTH: i32 = 0;

fn remove2(cards: &[u8; 20], out: &mut [u8; 20], m1: usize, m2: usize) {
    let mut j = 0usize;
    for i in 0..20usize {
        if i != m1 && i != m2 {
            out[j] = cards[i];
            j += 1;
        }
    }
    // Note: remaining bytes in `out` are left as-is, matching C behavior where
    // only the first (20-2*depth) bytes are relevant at each recursion level.
}

fn solve(buf: &[u8; 20], depth: i32) {
    let last = (20 - 2 * depth) as usize;

    // In C: char mybuf[4][20]; (uninitialized). We emulate by creating buffers
    // and only relying on the prefix written by remove2, just like the C code.
    let mut mybuf = [[0u8; 20]; 4];

    for i in 0..last {
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            remove2(buf, &mut mybuf[0], i, i + 4);
            solve(&mybuf[0], depth + 1);
        }
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            remove2(buf, &mut mybuf[1], i, i + 1);
            solve(&mybuf[1], depth + 1);
        }
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            remove2(buf, &mut mybuf[2], i, i + 5);
            solve(&mybuf[2], depth + 1);
        }
        if i < 16 && (i % 4) > 0 && i + 3 < last && buf[i] == buf[i + 3] {
            remove2(buf, &mut mybuf[3], i, i + 3);
            solve(&mybuf[3], depth + 1);
        }
    }

    unsafe {
        if depth > MAX_DEPTH {
            MAX_DEPTH = depth;
        }
    }
}

fn main() {
    // Read entire stdin; the C code uses scanf with whitespace in the format
    // strings, which effectively skips any whitespace between tokens.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    for _ in 0..n {
        let mut buf = [0u8; 20];
        for j in 0..20 {
            let tok = it.next().unwrap_or("0");
            let b = tok.as_bytes()[0];
            buf[j] = b.wrapping_sub(b'0'); // matches C's `char` arithmetic
        }

        unsafe {
            MAX_DEPTH = 0;
        }
        solve(&buf, 0);

        let max_depth = unsafe { MAX_DEPTH };
        let remaining = 20 - (max_depth * 2);
        println!("{}", remaining);
    }
}
