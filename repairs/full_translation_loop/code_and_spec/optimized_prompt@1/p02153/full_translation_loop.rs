use std::collections::BinaryHeap;
use std::cmp::Ordering;
use std::io::{self, BufRead};

#[derive(Copy, Clone, Eq, PartialEq)]
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
    let mut dist = vec![vec![u64::MAX; w + 1]; h + 1];
    let mut heap = BinaryHeap::new();
    
    dist[sr][sc] = 0;
    heap.push(State { cost: 0, row: sr, col: sc });

    while let Some(State { cost, row, col }) = heap.pop() {
        if row == gr && col == gc {
            return Some(cost);
        }
        if cost > dist[row][col] {
            continue;
        }

        let f = bomb[row][col] as usize;
        let k = (2 - f) << 2;

        for i in 0..k {
            let rr = (row as isize + MOVES[i].0) as usize;
            let cc = (col as isize + MOVES[i].1) as usize;

            if rr < 1 || rr > h || cc < 1 || cc > w {
                continue;
            }

            let g = (map[rr][cc] == '#') as usize;
            if f != 0 && g != 0 {
                continue;
            }

            let new_cost = cost + tt[i][g];
            if new_cost < dist[rr][cc] {
                dist[rr][cc] = new_cost;
                heap.push(State { cost: new_cost, row: rr, col: cc });
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
