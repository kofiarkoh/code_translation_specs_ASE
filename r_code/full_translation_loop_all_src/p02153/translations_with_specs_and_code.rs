use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

fn main() {
    // Read all input as whitespace-delimited tokens (matches C's ins/in behavior).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let h: usize = it.next().unwrap().parse().unwrap();
    let w: usize = it.next().unwrap().parse().unwrap();
    let a_in: i64 = it.next().unwrap().parse().unwrap();
    let b: i64 = it.next().unwrap().parse().unwrap();

    // We allocate with a 1-cell margin around the grid so that hazard expansion to 8 neighbors
    // can be done without bounds checks, matching the C program's behavior.
    // SPEC precondition: inputs must be such that this does not go out of internal storage.
    let mut map = vec![vec![b'\0'; w + 2]; h + 2];
    let mut bomb = vec![vec![false; w + 2]; h + 2];
    let mut vis = vec![vec![false; w + 2]; h + 2];

    let mv: [(isize, isize); 8] = [
        (-1, 0),
        (0, 1),
        (1, 0),
        (0, -1),
        (-1, -1),
        (-1, 1),
        (1, 1),
        (1, -1),
    ];

    let mut sr = 0usize;
    let mut sc = 0usize;
    let mut gr = 0usize;
    let mut gc = 0usize;

    for r in 1..=h {
        let row = it.next().unwrap().as_bytes();
        // Use first W characters as per spec.
        for c in 1..=w {
            let ch = row[c - 1];
            map[r][c] = ch;
            if ch == b'*' {
                bomb[r][c] = true;
                // Mark 8 neighbors without explicit boundary checks (as in C).
                for &(dr, dc) in &mv {
                    let rr = (r as isize + dr) as usize;
                    let cc = (c as isize + dc) as usize;
                    bomb[rr][cc] = true;
                }
            } else if ch == b's' {
                sr = r;
                sc = c;
            } else if ch == b'g' {
                gr = r;
                gc = c;
            }
        }
    }

    // Precompute step costs tt[i][g] where i is direction index, g is 0/1 for wall destination.
    // Orthogonal: A_in (+B if wall). Diagonal: 2*A_in (+B if wall).
    let mut tt = [[0i64; 2]; 8];
    for i in 0..4 {
        tt[i][0] = a_in;
        tt[i][1] = a_in + b;
    }
    let a2 = a_in * 2;
    for i in 4..8 {
        tt[i][0] = a2;
        tt[i][1] = a2 + b;
    }

    // Dijkstra: state is (cost, r, c). Finalize on first pop (vis).
    let mut pq: BinaryHeap<(Reverse<i64>, usize, usize)> = BinaryHeap::new();
    pq.push((Reverse(0), sr, sc));

    let mut ans: Option<i64> = None;

    while let Some((Reverse(t), r, c)) = pq.pop() {
        if r == gr && c == gc {
            ans = Some(t);
            break;
        }
        if vis[r][c] {
            continue;
        }
        vis[r][c] = true;

        let f = bomb[r][c]; // dangerous?
        let k = if f { 4 } else { 8 }; // dangerous => only orthogonal
        for i in 0..k {
            let (dr, dc) = mv[i];
            let rr_i = r as isize + dr;
            let cc_i = c as isize + dc;
            if rr_i < 1 || rr_i > h as isize || cc_i < 1 || cc_i > w as isize {
                continue;
            }
            let rr = rr_i as usize;
            let cc = cc_i as usize;

            let g_wall = map[rr][cc] == b'#';
            if f && g_wall {
                continue; // dangerous -> wall forbidden
            }

            // Diagonal moves only allowed when destination is a wall.
            // Orthogonal moves allowed regardless of wall (subject to above rule).
            if (i < 4 || g_wall) && !vis[rr][cc] {
                let g_idx = if g_wall { 1 } else { 0 };
                pq.push((Reverse(t + tt[i][g_idx]), rr, cc));
            }
        }
    }

    match ans {
        Some(v) => {
            print!("{v}\n");
        }
        None => {
            print!("INF\n");
        }
    }
}
