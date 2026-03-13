// all_pairs_shortest.rs
// Translation of the provided specification into Rust.
//
// Behavior summary:
// - Reads n and m (whitespace separated) from stdin.
// - Then reads m lines each formatted as "%d,%d,%d,%d" (comma-separated integers).
// - Then reads one final query line formatted as "%d,%d,%d,%d".
// - Builds a fixed 20x20 distance matrix with INF = 2^21, sets diagonal to 0.
// - For each edge line sets dist[a-1][b-1] = c and dist[b-1][a-1] = d (assignment).
// - Runs Floyd–Warshall on the n x n prefix of the matrix.
// - Prints: query_c - dist[a-1][b-1] - dist[b-1][a-1] - query_d followed by a newline.
//
// Notes on implementation choices to preserve specification semantics:
// - Uses i64 for arithmetic to avoid unintended overflow while preserving INF semantics.
// - INF is 2^21 = 2097152 as specified.
// - Edge lines and query lines are parsed by splitting on ',' expecting exactly 4 integers.
// - The parser for the initial n and m reads two integers from the start of stdin
//   by scanning and skipping whitespace, leaving the remainder for line-oriented parsing.
// - The code assumes well-formed input as the specification states; many malformed cases
//   will cause a panic (mirroring "undefined" behavior described in the spec).

use std::io::{self, Read};

const NMAX: usize = 20;
const INF: i64 = 1 << 21;

fn parse_int_from(s: &str, mut idx: usize) -> Option<(i64, usize)> {
    let bytes = s.as_bytes();
    let len = bytes.len();
    // Skip whitespace
    while idx < len && bytes[idx].is_ascii_whitespace() {
        idx += 1;
    }
    if idx >= len {
        return None;
    }
    // Optional sign
    let mut neg = false;
    if bytes[idx] == b'-' {
        neg = true;
        idx += 1;
    } else if bytes[idx] == b'+' {
        idx += 1;
    }
    // At least one digit required
    if idx >= len || !bytes[idx].is_ascii_digit() {
        return None;
    }
    let mut val: i64 = 0;
    while idx < len && bytes[idx].is_ascii_digit() {
        val = val * 10 + ((bytes[idx] - b'0') as i64);
        idx += 1;
    }
    if neg {
        val = -val;
    }
    Some((val, idx))
}

fn parse_csv_four(line: &str) -> (i64, i64, i64, i64) {
    let parts: Vec<&str> = line.split(',').collect();
    if parts.len() != 4 {
        panic!("expected 4 comma-separated integers per line");
    }
    let a = parts[0].parse::<i64>().expect("parse int");
    let b = parts[1].parse::<i64>().expect("parse int");
    let c = parts[2].parse::<i64>().expect("parse int");
    let d = parts[3].parse::<i64>().expect("parse int");
    (a, b, c, d)
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("read stdin");

    // Parse first two integers (n and m) by scanning from the start
    let (n_raw, pos) = parse_int_from(&input, 0).expect("expected n");
    let (m_raw, pos2) =
        parse_int_from(&input, pos).expect("expected m after n");

    let n = n_raw as usize;
    let m = m_raw as usize;

    // Prepare distance matrix 20x20
    let mut dist = [[INF; NMAX]; NMAX];
    for i in 0..NMAX {
        dist[i][i] = 0;
    }

    // Remaining input from pos2: process lines to get m edge lines then one query line.
    // We'll iterate over lines in order, skipping empty lines.
    let remaining = &input[pos2..];
    let mut lines_iter = remaining.lines().filter(|ln| !ln.is_empty());

    // Read m edge lines
    for _ in 0..m {
        let line = lines_iter
            .next()
            .expect("expected edge line but input ended");
        let (a_raw, b_raw, c_raw, d_raw) = parse_csv_four(line);
        // Convert 1-based to 0-based indices
        let ai = (a_raw as usize) - 1;
        let bi = (b_raw as usize) - 1;
        // The specification says to set the directed weights to the provided values.
        // We assign directly (not min).
        dist[ai][bi] = c_raw;
        dist[bi][ai] = d_raw;
    }

    // Read final query line
    let query_line = lines_iter
        .next()
        .expect("expected final query line but input ended");
    let (qa_raw, qb_raw, qc_raw, qd_raw) = parse_csv_four(query_line);
    let qa = (qa_raw as usize) - 1;
    let qb = (qb_raw as usize) - 1;
    let qc = qc_raw;
    let qd = qd_raw;

    // Floyd–Warshall on the n x n submatrix
    for k in 0..n {
        for i in 0..n {
            // Skip if dist[i][k] is INF to avoid INF+something overflow semantics
            if dist[i][k] >= INF {
                continue;
            }
            for j in 0..n {
                if dist[k][j] >= INF {
                    continue;
                }
                let via = dist[i][k] + dist[k][j];
                if via < dist[i][j] {
                    dist[i][j] = via;
                }
            }
        }
    }

    // Compute result: qc - dist[qa][qb] - dist[qb][qa] - qd
    let dab = dist[qa][qb];
    let dba = dist[qb][qa];
    let result = qc - dab - dba - qd;

    println!("{}", result);
}
