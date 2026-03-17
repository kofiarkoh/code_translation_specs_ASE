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

fn dijkstra(grid: &Vec<Vec<char>>, start: (usize, usize), goal: (usize, usize), bomb: &Vec<Vec<bool>>, costs: &[(u64, u64)]) -> Option<u64> {
    let (h, w) = (grid.len(), grid[0].len());
    let mut dist = vec![vec![u64::MAX; w]; h];
    let mut heap = BinaryHeap::new();
    
    dist[start.0][start.1] = 0;
    heap.push(State { cost: 0, row: start.0, col: start.1 });

    while let Some(State { cost, row, col }) = heap.pop() {
        if (row, col) == goal {
            return Some(cost);
        }
        if cost > dist[row][col] {
            continue;
        }

        let f = bomb[row][col] as usize;
        let k = (2 - f) << 2;

        for i in 0..k {
            let (rr, cc) = (row as isize + MOVES[i].0, col as isize + MOVES[i].1);
            if rr < 1 || rr >= h as isize || cc < 1 || cc >= w as isize {
                continue;
            }
            let g = (grid[rr as usize][cc as usize] == '#') as usize;
            if f != 0 && g != 0 {
                continue;
            }
            if (i < 4 || g != 0) && dist[rr as usize][cc as usize] > cost + costs[i][g] {
                dist[rr as usize][cc as usize] = cost + costs[i][g];
                heap.push(State { cost: dist[rr as usize][cc as usize], row: rr as usize, col: cc as usize });
            }
        }
    }
    None
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().map(|line| line.unwrap());

    let first_line: Vec<usize> = lines.next().unwrap().split_whitespace().map(|x| x.parse().unwrap()).collect();
    let (h, w) = (first_line[0], first_line[1]);
    let (a, b) = (lines.next().unwrap().parse::<u64>().unwrap(), lines.next().unwrap().parse::<u64>().unwrap());

    let mut grid = vec![vec![' '; w + 2]; h + 2];
    let mut bomb = vec![vec![false; w + 2]; h + 2];
    let mut start = (0, 0);
    let mut goal = (0, 0);

    for r in 1..=h {
        let line: Vec<char> = lines.next().unwrap().chars().collect();
        for c in 1..=w {
            grid[r][c] = line[c - 1];
            if grid[r][c] == '*' {
                bomb[r][c] = true;
                for (dr, dc) in MOVES.iter() {
                    bomb[(r as isize + dr) as usize][(c as isize + dc) as usize] = true;
                }
            } else if grid[r][c] == 's' {
                start = (r, c);
            } else if grid[r][c] == 'g' {
                goal = (r, c);
            }
        }
    }

    let mut costs = vec![(0, 0); 8];
    for i in 0..4 {
        costs[i] = (a, a + b);
    }
    for i in 4..8 {
        costs[i] = (a * 2, a * 2 + b);
    }

    match dijkstra(&grid, start, goal, &bomb, &costs) {
        Some(ans) => println!("{}", ans),
        None => println!("INF"),
    }
}
