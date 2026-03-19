use std::io::{self, Read};

fn remove2(cards: &[u8], m1: usize, m2: usize) -> Vec<u8> {
    // Remove exactly two positions (m1, m2) from the current compacted list.
    // The C code always iterates i=0..20 but only the first `last_` entries are meaningful.
    // Here we operate on the current length directly (which equals `last_`).
    let mut out = Vec::with_capacity(cards.len().saturating_sub(2));
    for (i, &v) in cards.iter().enumerate() {
        if i != m1 && i != m2 {
            out.push(v);
        }
    }
    out
}

fn solve(cards: &[u8], depth: usize, best: &mut usize) {
    // `cards` is the current compacted row-major list of remaining cards.
    // Adjacency is computed in a conceptual 5x4 grid using indices 0..last_-1,
    // with the same bounds checks as the C code.
    let last_ = cards.len();
    if depth > *best {
        *best = depth;
    }

    for i in 0..last_ {
        // Vertical neighbor: i and i+4 (same column, next row)
        if i < 16 && i + 4 < last_ && cards[i] == cards[i + 4] {
            let next = remove2(cards, i, i + 4);
            solve(&next, depth + 1, best);
        }
        // Horizontal neighbor: i and i+1 (same row, next column)
        if (i % 4) < 3 && i + 1 < last_ && cards[i] == cards[i + 1] {
            let next = remove2(cards, i, i + 1);
            solve(&next, depth + 1, best);
        }
        // Diagonal down-right: i and i+5
        if i < 16 && (i % 4) < 3 && i + 5 < last_ && cards[i] == cards[i + 5] {
            let next = remove2(cards, i, i + 5);
            solve(&next, depth + 1, best);
        }
        // Diagonal down-left: i and i+3
        if i < 16 && (i % 4) > 0 && i + 3 < last_ && cards[i] == cards[i + 3] {
            let next = remove2(cards, i, i + 3);
            solve(&next, depth + 1, best);
        }
    }
}

fn main() {
    // Input format matches the C program: integers and characters read with whitespace skipping.
    // We therefore tokenize by whitespace.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let t: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut outputs = String::new();

    for case_idx in 0..t {
        let mut cards: Vec<u8> = Vec::with_capacity(20);
        for _ in 0..20 {
            let tok = it.next().unwrap();
            let b = tok.as_bytes()[0];
            // Per spec, input is intended to be digits; behavior otherwise is undefined.
            cards.push(b.wrapping_sub(b'0'));
        }

        let mut best_depth = 0usize;
        solve(&cards, 0, &mut best_depth);

        let remaining = 20usize - best_depth * 2;
        outputs.push_str(&remaining.to_string());
        if case_idx + 1 < t {
            outputs.push('\n');
        }
    }

    print!("{outputs}");
}
