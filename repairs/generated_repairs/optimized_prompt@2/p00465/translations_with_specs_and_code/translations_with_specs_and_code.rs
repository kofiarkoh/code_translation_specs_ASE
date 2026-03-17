// AOJ 0542: Authentication Level
// Translated from C to Rust

use std::collections::BinaryHeap;
use std::io::{self, BufRead};

#[derive(Debug, Clone, Eq, PartialEq)]
struct Que {
    t: i32,
    r: usize,
    c: usize,
}

// Implementing Ord for priority queue behavior
impl Ord for Que {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        other.t.cmp(&self.t) // Reverse order for min-heap
    }
}

impl PartialOrd for Que {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

fn main() {
    let stdin = io::stdin();
    let reader = stdin.lock();
    let mut lines = reader.lines().filter_map(Result::ok);
    
    let mut results = Vec::new();
    
    while let Some(line) = lines.next() {
        let r: usize = line.trim().parse().unwrap();
        
        for _ in 0..2 {
            let dimensions: Vec<usize> = lines.next().unwrap().trim().split_whitespace()
                .map(|s| s.parse().unwrap()).collect();
            let (w, h, x, y) = (dimensions[0], dimensions[1], dimensions[2] - 1, dimensions[3] - 1);
            
            let mut l = vec![vec![0; w]; h];
            for r in 0..h {
                let levels: Vec<i32> = lines.next().unwrap().trim().split_whitespace()
                    .map(|s| s.parse().unwrap()).collect();
                for c in 0..w {
                    l[r][c] = levels[c];
                }
            }
            
            let mut mk = vec![vec![false; w]; h];
            let mut heap = BinaryHeap::new();
            let mut level = vec![0; h * w];
            let mut lsz = 0;
            let mut max = 0;

            heap.push(Que { t: l[y][x], r: y, c: x });
            mk[y][x] = true;

            while let Some(Que { t, r, c }) = heap.pop() {
                if t > max {
                    max = t;
                }
                level[lsz] = max;
                lsz += 1;

                let mv = [(0, -1isize), (1, 0), (0, 1), (-1isize, 0)];
                for (dr, dc) in mv.iter() {
                    let r2 = r.wrapping_add(*dr as usize);
                    let c2 = c.wrapping_add(*dc as usize);
                    if r2 < h && c2 < w && !mk[r2][c2] {
                        mk[r2][c2] = true;
                        heap.push(Que { t: l[r2][c2], r: r2, c: c2 });
                    }
                }
            }

            let mut ans = i32::MAX;
            for i in 0..=r {
                let j = r - i;
                if i < lsz && j < lsz {
                    let t = level[i] + level[j];
                    if t < ans {
                        ans = t;
                    }
                }
            }
            results.push(ans);
        }
    }

    for result in results {
        println!("{}", result);
    }
}
