use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s).unwrap();
        Self {
            input: s.into_bytes(),
            idx: 0,
        }
    }
    fn next_usize(&mut self) -> usize {
        while self.idx < self.input.len() && self.input[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut n: usize = 0;
        while self.idx < self.input.len() && !self.input[self.idx].is_ascii_whitespace() {
            n = n * 10 + (self.input[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        n
    }
}

/// Derive S(k) sequence for one grid:
/// S(0)=0, and for k>=1, S(k) is the minimal possible maximum cell value among all
/// connected k-cell sets containing the start. Per spec, this equals the "greedy expansion"
/// that always picks the currently reachable unvisited cell with minimum value, tracking
/// the running maximum.
fn derive_levels(grid: &[Vec<i32>], w: usize, h: usize, start_x: usize, start_y: usize) -> Vec<i32> {
    let mut visited = vec![vec![false; w]; h];
    let mut heap: BinaryHeap<Reverse<(i32, usize, usize)>> = BinaryHeap::new();

    let mut levels: Vec<i32> = Vec::with_capacity(w * h + 1);
    levels.push(0); // S(0)=0

    visited[start_y][start_x] = true;
    heap.push(Reverse((grid[start_y][start_x], start_y, start_x)));

    let mut current_max: i32 = 0;

    let dirs = [(-1isize, 0isize), (0, 1), (1, 0), (0, -1)];
    while let Some(Reverse((t, r, c))) = heap.pop() {
        if t > current_max {
            current_max = t;
        }
        levels.push(current_max);

        for (dr, dc) in dirs {
            let nr = r as isize + dr;
            let nc = c as isize + dc;
            if nr < 0 || nr >= h as isize || nc < 0 || nc >= w as isize {
                continue;
            }
            let (nr, nc) = (nr as usize, nc as usize);
            if visited[nr][nc] {
                continue;
            }
            // Match C behavior: mark visited when enqueued so a cell is never enqueued twice.
            visited[nr][nc] = true;
            heap.push(Reverse((grid[nr][nc], nr, nc)));
        }
    }

    levels
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let r_budget = sc.next_usize();
        if r_budget == 0 {
            break;
        }

        let mut seqs: [Vec<i32>; 2] = [Vec::new(), Vec::new()];

        for g in 0..2 {
            let w = sc.next_usize();
            let h = sc.next_usize();
            let x0 = sc.next_usize() - 1; // 1-based to 0-based
            let y0 = sc.next_usize() - 1;

            let mut grid = vec![vec![0i32; w]; h];
            for r in 0..h {
                for c in 0..w {
                    grid[r][c] = sc.next_usize() as i32;
                }
            }

            seqs[g] = derive_levels(&grid, w, h, x0, y0);
        }

        // Min over i+j=R with i,j within derived sequence lengths.
        let mut ans: i32 = i32::MAX;
        for i in 0..=r_budget {
            let j = r_budget - i;
            if i < seqs[0].len() && j < seqs[1].len() {
                let t = seqs[0][i] + seqs[1][j];
                if t < ans {
                    ans = t;
                }
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
