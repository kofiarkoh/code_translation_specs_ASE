use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Copy, Eq, PartialEq)]
struct State {
    cost: u64,
    row: usize,
    col: usize,
}

impl Ord for State {
    fn cmp(&self, other: &Self) -> Ordering {
        other.cost.cmp(&self.cost) // Reverse ordering for min-heap
    }
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

const MAXV: usize = 1005;
const MOVES: [(isize, isize); 8] = [(-1, 0), (0, 1), (1, 0), (0, -1), (-1, -1), (-1, 1), (1, 1), (1, -1)];

fn dijkstra(
    sr: usize,
    sc: usize,
    gr: usize,
    gc: usize,
    bomb: &[[bool; MAXV]; MAXV],
    map: &[[char; MAXV]; MAXV],
    tt: &[[u64; 2]; 8],
    h: usize,
    w: usize,
) -> Option<u64> {
    let mut heap = BinaryHeap::new();
    let mut visited = [[false; MAXV]; MAXV];

    heap.push(State { cost: 0, row: sr, col: sc });

    while let Some(State { cost, row, col }) = heap.pop() {
        if visited[row][col] {
            continue;
        }
        visited[row][col] = true;

        if row == gr && col == gc {
            return Some(cost);
        }

        let f = bomb[row][col] as usize;
        let k = (2 - f) << 2;

        for i in 0..k {
            let rr = (row as isize + MOVES[i as usize].0) as usize;
            let cc = (col as isize + MOVES[i as usize].1) as usize;

            if rr < 1 || rr > h || cc < 1 || cc > w {
                continue;
            }

            let g = (map[rr][cc] == '#') as usize;
            if f != 0 && g != 0 {
                continue;
            }

            if (i < 4 || g != 0) && !visited[rr][cc] {
                heap.push(State {
                    cost: cost + tt[i as usize][g],
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
    let mut dimensions = first_line.split_whitespace().map(|s| s.parse::<usize>().unwrap());
    let h = dimensions.next().unwrap();
    let w = dimensions.next().unwrap();

    let second_line = lines.next().unwrap().unwrap();
    let mut costs = second_line.split_whitespace().map(|s| s.parse::<u64>().unwrap());
    let a = costs.next().unwrap();
    let b = costs.next().unwrap();

    let mut map = [[' '; MAXV]; MAXV];
    let mut bomb = [[false; MAXV]; MAXV];
    let mut sr = 0;
    let mut sc = 0;
    let mut tr = 0;
    let mut tc = 0;

    for r in 1..=h {
        let line = lines.next().unwrap().unwrap();
        for (c, ch) in line.chars().enumerate() {
            map[r][c + 1] = ch;
            if ch == '*' {
                bomb[r][c + 1] = true;
                for (dr, dc) in MOVES.iter() {
                    let rr = (r as isize + dr) as usize;
                    let cc = (c + 1) as isize + dc;
                    if rr < MAXV && cc < MAXV as isize {
                        bomb[rr][cc as usize] = true;
                    }
                }
            } else if ch == 's' {
                sr = r;
                sc = c + 1;
            } else if ch == 'g' {
                tr = r;
                tc = c + 1;
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

    match dijkstra(sr, sc, tr, tc, &bomb, &map, &tt, h, w) {
        Some(ans) => println!("{}", ans),
        None => println!("INF"),
    }
}
