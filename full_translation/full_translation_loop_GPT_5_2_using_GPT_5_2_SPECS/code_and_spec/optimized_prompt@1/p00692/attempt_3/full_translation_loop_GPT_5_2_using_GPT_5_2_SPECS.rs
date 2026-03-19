use std::collections::HashMap;
use std::io::{self, Read};

fn pack_state(cards: &[u8]) -> u64 {
    // low 5 bits: len, then 20 nibbles (4 bits each) for digits
    let mut s = cards.len() as u64;
    let mut shift = 5u32;
    for &v in cards {
        s |= (v as u64) << shift;
        shift += 4;
    }
    s
}

fn remove2_packed(state: u64, len: usize, m1: usize, m2: usize) -> u64 {
    // Build next packed state directly (no Vec allocation).
    // Extract digits from `state` and skip indices m1,m2.
    let mut out_len = len - 2;
    let mut out: u64 = out_len as u64; // len in low 5 bits
    let mut out_shift = 5u32;

    for i in 0..len {
        if i == m1 || i == m2 {
            continue;
        }
        let shift = 5u32 + (i as u32) * 4;
        let v = ((state >> shift) & 0xF) as u64;
        out |= v << out_shift;
        out_shift += 4;
    }
    out
}

fn max_removals_packed(state: u64, memo: &mut HashMap<u64, u8>) -> u8 {
    if let Some(&v) = memo.get(&state) {
        return v;
    }
    let len = (state & 0x1F) as usize;
    let mut best: u8 = 0;

    // Helper to read digit at index i
    let get = |st: u64, i: usize| -> u8 {
        let shift = 5u32 + (i as u32) * 4;
        ((st >> shift) & 0xF) as u8
    };

    for i in 0..len {
        let vi = get(state, i);

        // Vertical: i and i+4
        if i < 16 && i + 4 < len && vi == get(state, i + 4) {
            let next = remove2_packed(state, len, i, i + 4);
            best = best.max(1 + max_removals_packed(next, memo));
        }
        // Horizontal: i and i+1
        if (i % 4) < 3 && i + 1 < len && vi == get(state, i + 1) {
            let next = remove2_packed(state, len, i, i + 1);
            best = best.max(1 + max_removals_packed(next, memo));
        }
        // Diagonal down-right: i and i+5
        if i < 16 && (i % 4) < 3 && i + 5 < len && vi == get(state, i + 5) {
            let next = remove2_packed(state, len, i, i + 5);
            best = best.max(1 + max_removals_packed(next, memo));
        }
        // Diagonal down-left: i and i+3
        if i < 16 && (i % 4) > 0 && i + 3 < len && vi == get(state, i + 3) {
            let next = remove2_packed(state, len, i, i + 3);
            best = best.max(1 + max_removals_packed(next, memo));
        }
    }

    memo.insert(state, best);
    best
}

fn main() {
    // Whitespace-separated tokens, matching scanf("%d ", ...) and scanf("%c ", ...)
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut out = String::new();

    for case_idx in 0..t {
        let mut cards: [u8; 20] = [0; 20];
        for j in 0..20 {
            let tok = it.next().unwrap();
            let b = tok.as_bytes()[0];
            cards[j] = b.wrapping_sub(b'0'); // undefined for non-digits, per spec
        }

        let init_state = pack_state(&cards);
        let mut memo: HashMap<u64, u8> = HashMap::with_capacity(1 << 16);
        let m = max_removals_packed(init_state, &mut memo) as usize;
        let remaining = 20usize - 2 * m;

        out.push_str(&remaining.to_string());
        if case_idx + 1 < t {
            out.push('\n');
        }
    }

    print!("{out}");
}
