use std::collections::HashMap;
use std::io::{self, Read};

fn remove2(cards: &[u8; 20], out: &mut [u8; 20], m1: usize, m2: usize) {
    let mut j = 0usize;
    for i in 0..20usize {
        if i != m1 && i != m2 {
            out[j] = cards[i];
            j += 1;
        }
    }
}

/// Memoized search for maximum depth reachable from this state.
/// Returns the maximum depth (number of removals) achievable starting at `depth`.
fn solve_memo(buf: &[u8; 20], depth: u8, memo: &mut HashMap<(Vec<u8>, u8), u8>) -> u8 {
    let last = (20u8 - 2 * depth) as usize;

    // Key must preserve the entire relevant prefix; previous attempt used u128 which
    // cannot hold 20 bytes and caused collisions/wrong answers.
    let key_vec = buf[..last].to_vec();
    let state = (key_vec, depth);
    if let Some(&ans) = memo.get(&state) {
        return ans;
    }

    let mut best = depth;
    let mut mybuf = [[0u8; 20]; 4];

    for i in 0..last {
        if i < 16 && i + 4 < last && buf[i] == buf[i + 4] {
            remove2(buf, &mut mybuf[0], i, i + 4);
            best = best.max(solve_memo(&mybuf[0], depth + 1, memo));
        }
        if (i % 4) < 3 && i + 1 < last && buf[i] == buf[i + 1] {
            remove2(buf, &mut mybuf[1], i, i + 1);
            best = best.max(solve_memo(&mybuf[1], depth + 1, memo));
        }
        if i < 16 && (i % 4) < 3 && i + 5 < last && buf[i] == buf[i + 5] {
            remove2(buf, &mut mybuf[2], i, i + 5);
            best = best.max(solve_memo(&mybuf[2], depth + 1, memo));
        }
        if i < 16 && (i % 4) > 0 && i + 3 < last && buf[i] == buf[i + 3] {
            remove2(buf, &mut mybuf[3], i, i + 3);
            best = best.max(solve_memo(&mybuf[3], depth + 1, memo));
        }
    }

    memo.insert(state, best);
    best
}

fn main() {
    // Need to mimic scanf("%d ", &n) then scanf("%c ", &buf[j]).
    // The "%c " reads a single non-whitespace char, skipping any whitespace after it.
    // Therefore we must read raw bytes and pick non-whitespace characters.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx = 0usize;

    // Parse integer n (skipping leading whitespace).
    while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }
    let mut n: usize = 0;
    while idx < bytes.len() && !bytes[idx].is_ascii_whitespace() {
        n = n * 10 + (bytes[idx] - b'0') as usize;
        idx += 1;
    }

    let mut out = String::new();
    for _ in 0..n {
        let mut buf = [0u8; 20];
        for j in 0..20 {
            // scanf("%c ") does not skip leading whitespace for %c itself,
            // but the trailing space in the format causes scanf to skip any
            // whitespace after reading the character. In typical AOJ inputs,
            // cards are digits possibly separated by whitespace/newlines.
            // To robustly match that, we skip whitespace before taking the next char.
            while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                idx += 1;
            }
            let c = if idx < bytes.len() { bytes[idx] } else { b'0' };
            idx = idx.saturating_add(1);
            buf[j] = c.wrapping_sub(b'0'); // match C's `char` arithmetic
            // Skip any whitespace after the character (the trailing space in "%c ").
            while idx < bytes.len() && bytes[idx].is_ascii_whitespace() {
                idx += 1;
            }
        }

        let mut memo: HashMap<(Vec<u8>, u8), u8> = HashMap::new();
        let max_depth = solve_memo(&buf, 0, &mut memo) as i32;
        let remaining = 20 - max_depth * 2;
        out.push_str(&format!("{remaining}\n"));
    }

    print!("{out}");
}
