use std::collections::BinaryHeap;
use std::io::{self, BufRead};

const MAXV: usize = 1005;

#[derive(Copy, Clone, Eq, PartialEq)]
struct State {
    time: u64,
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.time.cmp(&self.time) // Reverse order for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn dijkstra(
    sr: usize,
    sc: usize,
    gr: usize,
    gc: usize,
    bomb: &[[bool; MAXV]; MAXV],
    map: &[[char; MAXV]; MAXV],
    tt: &[[u64; 2]; 8],
) -> Option<u64> {
    let mut vis = [[false; MAXV]; MAXV];
    let mut heap = BinaryHeap::new();
    heap.push(State { time: 0, row: sr, col: sc });

    let mv: [(isize, isize); 8] = [
        (-1, 0), (0, 1), (1, 0), (0, -1),
        (-1, -1), (-1, 1), (1, 1), (1, -1),
    ];

    while let Some(State { time, row, col }) = heap.pop() {
        if row == gr && col == gc {
            return Some(time);
        }
        if vis[row][col] {
            continue;
        }
        vis[row][col] = true;

        let f = bomb[row][col] as usize;
        let k = (2 - f) << 2;

        for i in 0..k {
            let rr = (row as isize + mv[i].0) as usize;
            let cc = (col as isize + mv[i].1) as usize;

            if rr < 1 || rr > MAXV - 1 || cc < 1 || cc > MAXV - 1 {
                continue;
            }
            let g = (map[rr][cc] == '#') as usize;
            if f != 0 && g != 0 {
                continue;
            }
            if (i < 4 || g != 0) && !vis[rr][cc] {
                heap.push(State {
                    time: time + tt[i][g],
                    row: rr,
                    col: cc,
                });
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines();

    let first_line = lines.next().unwrap().unwrap();
    let mut params = first_line.split_whitespace();
    let h: usize = params.next().unwrap().parse().unwrap();
    let w: usize = params.next().unwrap().parse().unwrap();
    let a: u64 = params.next().unwrap().parse().unwrap();
    let b: u64 = params.next().unwrap().parse().unwrap();

    let mut map = [[' '; MAXV]; MAXV];
    let mut bomb = [[false; MAXV]; MAXV];
    let mut sr = 0;
    let mut sc = 0;
    let mut tr = 0;
    let mut tc = 0;

    for r in 1..=h {
        let line = lines.next().unwrap().unwrap();
        for (c, ch) in line.chars().enumerate() {
            let col = c + 1; // Adjust for 1-based indexing
            map[r][col] = ch;
            if ch == '*' {
                bomb[r][col] = true;
                for (dr, dc) in &[(0, 0), (-1, 0), (0, -1), (1, 0), (0, 1), (-1, -1), (-1, 1), (1, 1), (1, -1)] {
                    let rr = (r as isize + dr) as usize;
                    let cc = (col as isize + dc) as usize;
                    if rr < MAXV && cc < MAXV {
                        bomb[rr][cc] = true;
                    }
                }
            } else if ch == 's' {
                sr = r;
                sc = col;
            } else if ch == 'g' {
                tr = r;
                tc = col;
            }
        }
    }

    let mut tt = [[0; 2]; 8];
    for i in 0..4 {
        tt[i][0] = a;
        tt[i][1] = a + b;
    }
    let a_doubled = a * 2;
    for i in 4..8 {
        tt[i][0] = a_doubled;
        tt[i][1] = a_doubled + b;
    }

    match dijkstra(sr, sc, tr, tc, &bomb, &map, &tt) {
        Some(ans) => println!("{}", ans),
        None => println!("INF"),
    }
}
