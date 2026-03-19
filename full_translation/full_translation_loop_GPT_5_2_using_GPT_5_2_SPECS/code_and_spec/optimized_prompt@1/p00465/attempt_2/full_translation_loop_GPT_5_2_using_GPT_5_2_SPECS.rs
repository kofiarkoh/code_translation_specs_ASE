use std::cmp::Reverse;
use std::collections::BinaryHeap;
use std::io::{self, Read};

struct Scanner {
    buf: Vec<u8>,
    idx: usize,
}
impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { buf: input, idx: 0 }
    }
    #[inline]
    fn next_usize(&mut self) -> usize {
        // Skip whitespace
        while self.idx < self.buf.len() && self.buf[self.idx].is_ascii_whitespace() {
            self.idx += 1;
        }
        // If called at EOF, return 0 (input is well-formed per spec; this is just a guard)
        if self.idx >= self.buf.len() {
            return 0;
        }
        let mut n: usize = 0;
        while self.idx < self.buf.len() {
            let b = self.buf[self.idx];
            if b.is_ascii_whitespace() {
                break;
            }
            n = n * 10 + (b - b'0') as usize;
            self.idx += 1;
        }
        n
    }
}

/// Compute S(k) for k=0..=W*H for one grid, per spec.
/// Implementation matches the C program's behavior:
/// - Use a min-priority queue over currently reachable cells.
/// - Mark a cell as visited when it is enqueued (so it is enqueued at most once).
/// - Pop cells in increasing value order; S(k) is the running maximum of popped values.
fn derive_levels_flat(grid: &[i32], w: usize, h: usize, sx: usize, sy: usize) -> Vec<i32> {
    let n = w * h;
    let mut visited = vec![false; n];
    let mut heap: BinaryHeap<Reverse<(i32, u32)>> = BinaryHeap::new(); // (value, idx)

    let mut levels = Vec::with_capacity(n + 1);
    levels.push(0); // S(0)=0

    let start = sy * w + sx;
    visited[start] = true;
    heap.push(Reverse((grid[start], start as u32)));

    let mut current_max: i32 = 0;

    while let Some(Reverse((t, idx_u))) = heap.pop() {
        let idx = idx_u as usize;
        if t > current_max {
            current_max = t;
        }
        levels.push(current_max);

        let r = idx / w;
        let c = idx - r * w;

        // 4-neighbors
        if r > 0 {
            let ni = idx - w;
            if !visited[ni] {
                visited[ni] = true;
                heap.push(Reverse((grid[ni], ni as u32)));
            }
        }
        if c + 1 < w {
            let ni = idx + 1;
            if !visited[ni] {
                visited[ni] = true;
                heap.push(Reverse((grid[ni], ni as u32)));
            }
        }
        if r + 1 < h {
            let ni = idx + w;
            if !visited[ni] {
                visited[ni] = true;
                heap.push(Reverse((grid[ni], ni as u32)));
            }
        }
        if c > 0 {
            let ni = idx - 1;
            if !visited[ni] {
                visited[ni] = true;
                heap.push(Reverse((grid[ni], ni as u32)));
            }
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

        let mut seq0: Vec<i32> = Vec::new();
        let mut seq1: Vec<i32> = Vec::new();

        for g in 0..2 {
            let w = sc.next_usize();
            let h = sc.next_usize();
            let x0 = sc.next_usize() - 1; // 1-based to 0-based
            let y0 = sc.next_usize() - 1;

            let mut grid = vec![0i32; w * h];
            for r in 0..h {
                let base = r * w;
                for c in 0..w {
                    grid[base + c] = sc.next_usize() as i32;
                }
            }

            let seq = derive_levels_flat(&grid, w, h, x0, y0);
            if g == 0 {
                seq0 = seq;
            } else {
                seq1 = seq;
            }
        }

        let mut ans: i32 = i32::MAX;
        // Only consider feasible splits where indices exist in both sequences.
        // (Matches C: i < lsz[0] && j < lsz[1])
        for i in 0..=r_budget {
            let j = r_budget - i;
            if i < seq0.len() && j < seq1.len() {
                let t = seq0[i] + seq1[j];
                if t < ans {
                    ans = t;
                }
            }
        }

        out.push_str(&ans.to_string());
        out.push('\n');
    }

    print!("{out}");
}
