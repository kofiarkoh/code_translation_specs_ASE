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
    // Note: remaining elements (j..20) are left as-is; C code doesn't care.
}

fn solve(buf: &[i8; 20], depth: i32) {
    let last = (20 - 2 * depth) as usize;

    // Local buffers correspond to `char mybuf[4][20];`
    // In C they are uninitialized; we initialize to 0 but only the prefix is used.
    let mut mybuf = [[0i8; 20]; 4];

    for i in 0..last {
        // vertical (same column, next row)
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            remove2(buf, &mut mybuf[0], i, i + 4);
            solve(&mybuf[0], depth + 1);
        }
        // horizontal (same row, next column)
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            remove2(buf, &mut mybuf[1], i, i + 1);
            solve(&mybuf[1], depth + 1);
        }
        // diagonal down-right
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            remove2(buf, &mut mybuf[2], i, i + 5);
            solve(&mybuf[2], depth + 1);
        }
        // diagonal down-left
        if i < 16 && (i % 4) > 0 && i + 3 < last && buf[i] == buf[i + 3] {
            remove2(buf, &mut mybuf[3], i, i + 3);
            solve(&mybuf[3], depth + 1);
        }
    }

    // Mirrors `max_=max(max_,depth);` with global state.
    unsafe {
        if depth > MAX_DEPTH {
            MAX_DEPTH = depth;
        }
    }
}

fn main() {
    // Read entire stdin; we will parse like scanf("%d ", &n) and scanf("%c ", &buf[j]).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    fn skip_ws(bytes: &[u8], idx: &mut usize) {
        while *idx < bytes.len() && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
    }

    fn read_i32(bytes: &[u8], idx: &mut usize) -> i32 {
        skip_ws(bytes, idx);
        let mut sign = 1i32;
        if *idx < bytes.len() && bytes[*idx] == b'-' {
            sign = -1;
            *idx += 1;
        }
        let mut val = 0i32;
        while *idx < bytes.len() && bytes[*idx].is_ascii_digit() {
            val = val * 10 + (bytes[*idx] - b'0') as i32;
            *idx += 1;
        }
        // scanf("%d ") consumes trailing whitespace; emulate by skipping it.
        skip_ws(bytes, idx);
        sign * val
    }

    fn read_char(bytes: &[u8], idx: &mut usize) -> u8 {
        // scanf("%c ") does NOT skip leading whitespace for %c, but the trailing
        // space in the format string consumes any following whitespace.
        if *idx >= bytes.len() {
            return 0;
        }
        let c = bytes[*idx];
        *idx += 1;
        skip_ws(bytes, idx);
        c
    }

    let n = read_i32(bytes, &mut idx) as usize;
    let mut out = String::new();

    for _ in 0..n {
        let mut buf = [0i8; 20];
        for j in 0..20 {
            let c = read_char(bytes, &mut idx);
            // C: buf[j] -= '0'; with `char` arithmetic.
            // Here we keep it in i8; input is expected to be ASCII digits.
            buf[j] = (c as i16 - b'0' as i16) as i8;
        }

        unsafe {
            MAX_DEPTH = 0;
        }
        solve(&buf, 0);

        let max_depth = unsafe { MAX_DEPTH };
        let remaining = 20 - (max_depth * 2);
        out.push_str(&format!("{}\n", remaining));
    }

    print!("{out}");
}
