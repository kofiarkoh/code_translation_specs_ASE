use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = String::new();
        io::stdin().read_to_string(&mut input).unwrap();
        Self { buf: input.into_bytes(), idx: 0 }
    }
    fn next_usize(&mut self) -> usize {
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        let mut n: usize = 0;
        while self.idx < self.buf.len() && !self.buf[self.idx].is_ascii_whitespace() {
            n = n * 10 + (self.buf[self.idx] - b'0') as usize;
            self.idx += 1;
        }
        n
    }
}

/// Derive sequence S(k) for k=0..reachable_count, where:
/// S(0)=0 and for k>=1, S(k) is the minimal possible maximum cell value
/// among connected (4-adjacent) sets of exactly k cells containing the start.
/// Per spec invariant, this equals the smallest threshold T such that at least
/// k cells are reachable from the start using only cells with value <= T.
///
/// Implementation matches the C code: a best-first expansion by cell value,
/// tracking the running maximum of popped values; after each pop, append that max.
fn derive_levels(grid: &[Vec<i32>], w: usize, h: usize, sx: usize, sy: usize) -> Vec<i32> {
    let mut seen = vec![vec![false; w]; h];
    let mut heap: BinaryHeap<(Reverse<i32>, usize, usize)> = BinaryHeap::new();

    let start_val = grid[sy][sx];
    seen[sy][sx] = true;
    heap.push((Reverse(start_val), sy, sx));

    let mut levels: Vec<i32> = Vec::new();
    levels.push(0); // S(0)=0

    let mut current_max: i32 = 0;
    let dirs = [(-1isize, 0isize), (0, 1), (1, 0), (0, -1)];

    while let Some((Reverse(t), r, c)) = heap.pop() {
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
            if seen[nr][nc] {
                continue;
            }
            // Match C behavior: mark when enqueued so each cell is enqueued at most once.
            seen[nr][nc] = true;
            heap.push((Reverse(grid[nr][nc]), nr, nc));
        }
    }

    levels
}

fn main() {
    let mut sc = Scanner::new();
    let mut out = String::new();

    loop {
        let r_total = sc.next_usize();
        if r_total == 0 {
            break;
        }

        let mut seqs: [Vec<i32>; 2] = [Vec::new(), Vec::new()];

        for gi in 0..2 {
            let w = sc.next_usize();
            let h = sc.next_usize();
            let x1 = sc.next_usize(); // 1-based
            let y1 = sc.next_usize(); // 1-based
            let sx = x1 - 1;
            let sy = y1 - 1;

            let mut grid = vec![vec![0i32; w]; h];
            for rr in 0..h {
                for cc in 0..w {
                    grid[rr][cc] = sc.next_usize() as i32;
                }
            }

            seqs[gi] = derive_levels(&grid, w, h, sx, sy);
        }

        // Combine: min over k where 0<=k<=R, k in range of seq1 and R-k in range of seq2.
        // If no feasible pair exists, C would print a large sentinel; spec assumes feasibility.
        let mut ans: i32 = i32::MAX;
        for k in 0..=r_total {
            let j = r_total - k;
            if k < seqs[0].len() && j < seqs[1].len() {
                let t = seqs[0][k] + seqs[1][j];
                if t < ans {
                    ans = t;
                }
            }
        }

        out.push_str(&format!("{ans}\n"));
    }

    print!("{out}");
}
