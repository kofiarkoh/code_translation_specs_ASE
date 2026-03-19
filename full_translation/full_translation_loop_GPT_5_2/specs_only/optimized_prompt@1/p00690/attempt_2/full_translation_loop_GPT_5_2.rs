use std::cmp::Ordering;
use std::io::{self, Read};

#[derive(Clone, Debug)]
struct Edge {
    to: usize,
    w: i64,
    id: usize,
}

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let mut out = String::new();

    loop {
        let n = match it.next() {
            Some(s) => s.parse::<i64>().unwrap_or(0),
            None => break,
        };
        let m = match it.next() {
            Some(s) => s.parse::<i64>().unwrap_or(0),
            None => break,
        };

        if n == 0 {
            break;
        }

        let n_usize = if n > 0 { n as usize } else { 0 };
        let mut g: Vec<Vec<Edge>> = vec![Vec::new(); n_usize];
        let mut edge_id: usize = 0;

        for _ in 0..m {
            let a = match it.next() {
                Some(s) => s.parse::<i64>().unwrap_or(0),
                None => break,
            };
            let b = match it.next() {
                Some(s) => s.parse::<i64>().unwrap_or(0),
                None => break,
            };
            let w = match it.next() {
                Some(s) => s.parse::<i64>().unwrap_or(0),
                None => break,
            };

            // The sample indicates station identifiers are 1-based.
            // Invalid connections are ignored.
            if a < 1 || b < 1 || a > n || b > n || n_usize == 0 {
                continue;
            }
            let uu = (a - 1) as usize;
            let vv = (b - 1) as usize;

            g[uu].push(Edge { to: vv, w, id: edge_id });
            g[vv].push(Edge { to: uu, w, id: edge_id });
            edge_id += 1;
        }

        let total_edges = edge_id;
        let mut used = vec![false; total_edges];

        let mut best_len: i64 = i64::MIN;
        let mut best_path: Vec<usize> = Vec::new();

        for start in 0..n_usize {
            let mut path = vec![start];
            dfs_longest_trail(&g, start, &mut used, 0, &mut path, &mut best_len, &mut best_path);
        }

        if best_len == i64::MIN {
            // No valid path found (e.g., no edges). Longest path length is 0 with a single station if any.
            best_len = 0;
            if n_usize > 0 {
                best_path = vec![0];
            }
        }

        out.push_str(&format!("{}\n", best_len));
        if best_path.is_empty() {
            out.push('\n');
        } else {
            for (i, &v0) in best_path.iter().enumerate() {
                if i > 0 {
                    out.push(' ');
                }
                // Output station identifiers as 1-based.
                out.push_str(&(v0 + 1).to_string());
            }
            out.push('\n');
        }
    }

    print!("{}", out);
}

fn dfs_longest_trail(
    g: &Vec<Vec<Edge>>,
    u: usize,
    used: &mut Vec<bool>,
    cur_len: i64,
    path: &mut Vec<usize>,
    best_len: &mut i64,
    best_path: &mut Vec<usize>,
) {
    // Update best: prefer larger length; if tie, prefer lexicographically smaller station sequence
    // (in 1-based terms this is the same ordering as 0-based).
    match cur_len.cmp(best_len) {
        Ordering::Greater => {
            *best_len = cur_len;
            *best_path = path.clone();
        }
        Ordering::Equal => {
            if best_path.is_empty() || path < best_path {
                *best_path = path.clone();
            }
        }
        Ordering::Less => {}
    }

    // Deterministic exploration order.
    let mut edges = g[u].clone();
    edges.sort_by(|a, b| (a.to, a.id).cmp(&(b.to, b.id)));

    for e in edges {
        if used[e.id] {
            continue;
        }
        used[e.id] = true;
        path.push(e.to);
        // Use wrapping add to emulate typical C integer overflow behavior if it were relied upon.
        dfs_longest_trail(g, e.to, used, cur_len.wrapping_add(e.w), path, best_len, best_path);
        path.pop();
        used[e.id] = false;
    }
}
