use std::io::{self, Read};

fn remove2(cards: &[u8], m1: usize, m2: usize) -> Vec<u8> {
    let mut out = Vec::with_capacity(cards.len().saturating_sub(2));
    for (i, &v) in cards.iter().enumerate() {
        if i != m1 && i != m2 {
            out.push(v);
        }
    }
    out
}

fn solve(cards: &[u8], depth: usize, max_depth: &mut usize) {
    let last_ = cards.len();

    for i in 0..last_ {
        // vertical adjacency (same column, next row): i and i+4
        if i < 16 && i + 4 < last_ && cards[i] == cards[i + 4] {
            let next = remove2(cards, i, i + 4);
            solve(&next, depth + 1, max_depth);
        }
        // horizontal adjacency (same row, next column): i and i+1
        if (i % 4) < 3 && i + 1 < last_ && cards[i] == cards[i + 1] {
            let next = remove2(cards, i, i + 1);
            solve(&next, depth + 1, max_depth);
        }
        // diagonal down-right: i and i+5
        if i < 16 && (i % 4) < 3 && i + 5 < last_ && cards[i] == cards[i + 5] {
            let next = remove2(cards, i, i + 5);
            solve(&next, depth + 1, max_depth);
        }
        // diagonal down-left: i and i+3
        if i < 16 && (i % 4) > 0 && i + 3 < last_ && cards[i] == cards[i + 3] {
            let next = remove2(cards, i, i + 3);
            solve(&next, depth + 1, max_depth);
        }
    }

    *max_depth = (*max_depth).max(depth);
}

fn main() {
    // The original C uses scanf("%c ", ...) which reads a single character and then
    // consumes trailing whitespace. Inputs may therefore be given as 20 digits with
    // or without spaces/newlines. Using split_whitespace() is insufficient when
    // digits are contiguous, so we scan all digit characters from the entire input.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();

    let mut digits = input
        .bytes()
        .filter(|&b| b.is_ascii_digit())
        .map(|b| b - b'0');

    // First number of test cases: in the judge input it's an integer token.
    // We reconstruct it from leading digits.
    // (If the input is malformed, behavior is undefined per spec; we still avoid panics.)
    let mut n_opt: Option<usize> = None;
    {
        // Parse n from the original input more robustly: read first integer via bytes.
        let mut seen = false;
        let mut n: usize = 0;
        for b in input.bytes() {
            if b.is_ascii_digit() {
                seen = true;
                n = n * 10 + (b - b'0') as usize;
            } else if seen {
                break;
            }
        }
        if seen {
            n_opt = Some(n);
        }
    }
    let n = match n_opt {
        Some(v) => v,
        None => return,
    };

    // After parsing n from the raw input, we must advance the digit iterator past
    // the digits that formed n. Easiest: re-create an iterator that skips those digits.
    let n_digits = n.to_string().len();
    let mut digits = input
        .bytes()
        .filter(|&b| b.is_ascii_digit())
        .skip(n_digits)
        .map(|b| b - b'0');

    for _ in 0..n {
        let mut cards: Vec<u8> = Vec::with_capacity(20);
        for _ in 0..20 {
            if let Some(d) = digits.next() {
                cards.push(d);
            } else {
                // Undefined input per spec; exit quietly instead of panicking.
                return;
            }
        }

        let mut max_depth = 0usize;
        solve(&cards, 0, &mut max_depth);
        let remaining = 20usize - max_depth * 2;
        println!("{remaining}");
    }
}
