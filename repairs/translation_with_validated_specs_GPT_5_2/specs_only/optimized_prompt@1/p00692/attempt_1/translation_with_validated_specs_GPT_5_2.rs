use std::io::{self, Read};

fn dfs(cards: &mut [u8; 20], len: usize, depth: usize, best: &mut usize) {
    if depth > *best {
        *best = depth;
    }

    // Enumerate all valid removals under the fixed 4-column neighbor rules.
    for i in 0..len {
        let v = cards[i];

        // Right neighbor: j = i + 1, i % 4 < 3
        if i % 4 < 3 {
            let j = i + 1;
            if j < len && cards[j] == v {
                let mut next = *cards;
                // Remove indices i and j, compacting while preserving relative order.
                let mut k = 0usize;
                for idx in 0..len {
                    if idx != i && idx != j {
                        next[k] = cards[idx];
                        k += 1;
                    }
                }
                dfs(&mut next, len - 2, depth + 1, best);
            }
        }

        // Down neighbor: j = i + 4, i < 16
        if i < 16 {
            let j = i + 4;
            if j < len && cards[j] == v {
                let mut next = *cards;
                let mut k = 0usize;
                for idx in 0..len {
                    if idx != i && idx != j {
                        next[k] = cards[idx];
                        k += 1;
                    }
                }
                dfs(&mut next, len - 2, depth + 1, best);
            }
        }

        // Down-right neighbor: j = i + 5, i < 16, i % 4 < 3
        if i < 16 && i % 4 < 3 {
            let j = i + 5;
            if j < len && cards[j] == v {
                let mut next = *cards;
                let mut k = 0usize;
                for idx in 0..len {
                    if idx != i && idx != j {
                        next[k] = cards[idx];
                        k += 1;
                    }
                }
                dfs(&mut next, len - 2, depth + 1, best);
            }
        }

        // Down-left neighbor: j = i + 3, i < 16, i % 4 > 0
        if i < 16 && i % 4 > 0 {
            let j = i + 3;
            if j < len && cards[j] == v {
                let mut next = *cards;
                let mut k = 0usize;
                for idx in 0..len {
                    if idx != i && idx != j {
                        next[k] = cards[idx];
                        k += 1;
                    }
                }
                dfs(&mut next, len - 2, depth + 1, best);
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut outputs = String::new();

    for case_idx in 0..n {
        let mut cards = [0u8; 20];
        for i in 0..20 {
            let tok = it.next().unwrap();
            // Specification: each card value is a single digit character '0'..'9'.
            let b = tok.as_bytes()[0];
            cards[i] = b.wrapping_sub(b'0');
        }

        let mut best = 0usize;
        dfs(&mut cards, 20, 0, &mut best);

        let remaining = 20usize - 2usize * best;
        outputs.push_str(&remaining.to_string());
        if case_idx + 1 < n {
            outputs.push('\n');
        }
    }

    print!("{outputs}");
}
