use std::io::{self, Read};

static mut MAX_DEPTH: i32 = 0;

fn remove2(cards: &[i8; 20], out: &mut [i8; 20], m1: usize, m2: usize) {
    let mut j = 0usize;
    for i in 0..20usize {
        if i != m1 && i != m2 {
            out[j] = cards[i];
            j += 1;
        }
    }
    // Note: C leaves the remaining tail of `out` uninitialized; we keep whatever was there.
}

fn solve(buf: &[i8; 20], depth: i32) {
    let last = (20 - 2 * depth) as usize;

    // Local buffers correspond to `char mybuf[4][20];` in C.
    // In C, these are uninitialized; we emulate that by starting with arbitrary values.
    // Only the prefix written by `remove2` is ever read (up to `last-2`), so this is fine.
    let mut mybuf = [[0i8; 20]; 4];

    for i in 0..last {
        // vertical (i and i+4)
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            remove2(buf, &mut mybuf[0], i, i + 4);
            solve(&mybuf[0], depth + 1);
        }
        // horizontal (i and i+1)
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            remove2(buf, &mut mybuf[1], i, i + 1);
            solve(&mybuf[1], depth + 1);
        }
        // diagonal down-right (i and i+5)
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            remove2(buf, &mut mybuf[2], i, i + 5);
            solve(&mybuf[2], depth + 1);
        }
        // diagonal down-left (i and i+3)
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
    // C's `solve` is declared `int` but returns nothing (UB). Caller ignores return value.
}

fn main() {
    // Read entire stdin; C's scanf("%d ", ...) and scanf("%c ", ...) treat whitespace as separators.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    for _ in 0..n {
        let mut buf = [0i8; 20];
        for j in 0..20 {
            let tok = it.next().unwrap_or("0");
            let ch = tok.as_bytes()[0] as i8; // token is expected to be a single digit char
            buf[j] = ch - b'0' as i8;
        }

        unsafe { MAX_DEPTH = 0; }
        solve(&buf, 0);

        let max_depth = unsafe { MAX_DEPTH };
        let remaining = 20 - (max_depth * 2);
        println!("{}", remaining);
    }
}
