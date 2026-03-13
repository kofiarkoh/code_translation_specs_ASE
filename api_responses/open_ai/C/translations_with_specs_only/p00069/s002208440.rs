use std::io::{self, Read};

fn traverse(n: usize, rows: &Vec<Vec<u8>>) -> Vec<usize> {
    // labels[i] is the label at column i (0-based). Initially labels are 1..=n
    let mut labels: Vec<usize> = (1..=n).collect();
    for row in rows.iter() {
        // For each possible rung position j between column j and j+1, j = 0..n-2
        for j in 0..(n - 1) {
            if row[j] == b'1' {
                labels.swap(j, j + 1);
            }
        }
    }
    labels
}

fn main() {
    // Read entire stdin and split by whitespace
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    // Helper to parse next token to usize
    let mut next_usize = || -> Option<usize> {
        it.next().and_then(|s| s.parse::<usize>().ok())
    };

    while let Some(n) = next_usize() {
        if n == 0 {
            // terminate without producing output
            break;
        }

        // read m, atari, d
        let m = match next_usize() {
            Some(v) => v,
            None => break,
        };
        let atari = match next_usize() {
            Some(v) => v,
            None => break,
        };
        let d = match next_usize() {
            Some(v) => v,
            None => break,
        };

        // read d row strings
        let mut rows: Vec<Vec<u8>> = Vec::with_capacity(d);
        for _ in 0..d {
            let s = match it.next() {
                Some(tok) => tok,
                None => "",
            };
            // Convert to bytes. Specification guarantees length >= n-1 and characters '0'/'1'.
            // We'll keep the bytes; we only access indices 0..n-2.
            rows.push(s.as_bytes().to_vec());
        }

        // Check existing mapping
        let labels = traverse(n, &rows);
        // atari is 1-based column index; m is desired value
        if atari >= 1 && atari <= n && labels[atari - 1] == m {
            println!("0");
            continue;
        }

        // Search for single insertion. Rows are scanned top-to-bottom (r = 1..d),
        // columns left-to-right (c = 1..n-1).
        let mut found: Option<(usize, usize)> = None;
        'outer: for r in 1..=d {
            // For each candidate column c between 1 and n-1 inclusive
            for c in 1..=n - 1 {
                // Check adjacency rules and emptiness at position (r,c)
                // rows vector index r-1, position index c-1
                let row = &rows[r - 1];
                // Specification guarantees row.len() >= n-1, so direct indexing is OK.
                if row[c - 1] != b'0' {
                    continue; // position already occupied
                }
                if c > 1 && row[c - 2] == b'1' {
                    continue; // left adjacent rung present
                }
                if c < n - 1 && row[c] == b'1' {
                    continue; // right adjacent rung present
                }

                // Try insertion: make a temporary copy of rows, set that position to '1'
                let mut rows2 = rows.clone();
                rows2[r - 1][c - 1] = b'1';

                let labels2 = traverse(n, &rows2);
                if atari >= 1 && atari <= n && labels2[atari - 1] == m {
                    found = Some((r, c));
                    break 'outer;
                }
            }
        }

        if let Some((r, c)) = found {
            println!("{} {}", r, c);
        } else {
            println!("1");
        }
    }
}
