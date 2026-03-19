use std::io::{self, Read};

fn main() {
    // Fast integer parsing to avoid timeout from repeated parse::<usize>() calls.
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let bytes = input.as_bytes();
    let mut idx: usize = 0;

    fn next_usize(bytes: &[u8], idx: &mut usize) -> Option<usize> {
        let n = bytes.len();
        while *idx < n && bytes[*idx].is_ascii_whitespace() {
            *idx += 1;
        }
        if *idx >= n {
            return None;
        }
        let mut val: usize = 0;
        while *idx < n && !bytes[*idx].is_ascii_whitespace() {
            val = val * 10 + (bytes[*idx] - b'0') as usize;
            *idx += 1;
        }
        Some(val)
    }

    let mut out = String::new();

    loop {
        let n = match next_usize(bytes, &mut idx) {
            Some(v) => v,
            None => break,
        };
        let m = match next_usize(bytes, &mut idx) {
            Some(v) => v,
            None => break,
        };

        if n == 0 && m == 0 {
            break;
        }

        // Use adjacency lists (still within the spec's semantics) to avoid O(N^2) scans.
        // This preserves the set-based behavior: multiple edges/self-loops don't change marking.
        let mut adj: Vec<Vec<usize>> = vec![Vec::new(); 501];

        for _ in 0..m {
            let p1 = next_usize(bytes, &mut idx).unwrap();
            let p2 = next_usize(bytes, &mut idx).unwrap();
            // As in the C program, behavior for out-of-range endpoints is not constrained;
            // we keep the fixed 0..=500 capacity and ignore anything outside to avoid panic.
            if p1 <= 500 && p2 <= 500 {
                adj[p1].push(p2);
                adj[p2].push(p1);
            }
        }

        let mut marked = vec![false; 501];

        // Distance 1: neighbors of 1 (excluding 1 itself; we only count 2..=n anyway).
        for &v in &adj[1] {
            if v >= 2 && v <= n {
                marked[v] = true;
            }
        }

        // Distance 2: neighbors of neighbors of 1.
        for &u in &adj[1] {
            if u >= 2 && u <= n {
                for &v in &adj[u] {
                    if v >= 2 && v <= n {
                        marked[v] = true;
                    }
                }
            }
        }

        let mut count: usize = 0;
        for v in 2..=n {
            if marked[v] {
                count += 1;
            }
        }

        out.push_str(&count.to_string());
        out.push('\n');
    }

    print!("{out}");
}
