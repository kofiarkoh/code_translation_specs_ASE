// mst_transform.rs
// Translates the provided specification into Rust.
// Reads multiple graph instances from stdin, transforms edge distances via
// w = floor(dist / 100) - 1, then computes MST weight using Prim's algorithm
// starting from vertex 0. Prints one integer per test case (except the termination V==0).
//
// Notes on faithful translation choices:
// - Uses integer floor division for negative distances (mathematical floor).
// - Uses INF = 1_000_000 as the sentinel for "no direct edge".
// - If multiple edges are provided for the same pair, the last one in input overwrites the cost.
// - If the graph is disconnected, unreachable vertices increase the sum (via INF) as described.
// - Input parsing tolerates arbitrary non-numeric separators (commas/newlines/whitespace) and reads ints in sequence,
//   matching scanf-style behavior for the expected well-formed input.

use std::io::{self, Read};

const MAXV: usize = 100;
const INF: i32 = 1_000_000;

struct Scanner {
    buf: Vec<u8>,
    pos: usize,
    len: usize,
}

impl Scanner {
    fn new() -> io::Result<Self> {
        let mut s = String::new();
        io::stdin().read_to_string(&mut s)?;
        let buf = s.into_bytes();
        let len = buf.len();
        Ok(Scanner { buf, pos: 0, len })
    }

    // Parse next signed 32-bit integer from the buffer.
    // Skips any non-digit/non-sign characters. Returns None if no more numbers.
    fn next_i32(&mut self) -> Option<i32> {
        // Skip until we find '-' or digit
        while self.pos < self.len {
            let c = self.buf[self.pos];
            if c == b'-' || (c'0'..=b'9').contains(&c) {
                break;
            } else {
                self.pos += 1;
            }
        }
        if self.pos >= self.len {
            return None;
        }
        // Handle sign
        let mut sign = 1i32;
        if self.buf[self.pos] == b'-' {
            sign = -1;
            self.pos += 1;
        }

        // Must have at least one digit now for well-formed input; accumulate digits
        let mut val: i64 = 0;
        let mut any = false;
        while self.pos < self.len {
            let c = self.buf[self.pos];
            if (b'0'..=b'9').contains(&c) {
                any = true;
                val = val * 10 + (c - b'0') as i64;
                self.pos += 1;
            } else {
                break;
            }
        }
        if !any {
            // No digits found after optional sign: treat as no number.
            return None;
        }
        let res = val as i32 * sign;
        Some(res)
    }
}

// Compute floor(a / b) for integers where b > 0.
// This returns the mathematical floor, differing from truncating division for negative a.
fn floor_div(a: i32, b: i32) -> i32 {
    // b is expected to be positive (100 in this program).
    if b <= 0 {
        // Should not happen in this spec. Fallback to Rust's truncating division.
        return a / b;
    }
    if a >= 0 {
        a / b
    } else {
        // For negative a, floor(a/b) = - ((-a + b - 1) / b)
        -((( - (a + 0) ) + (b - 1)) / b)
    }
}

fn main() -> io::Result<()> {
    let mut scanner = Scanner::new()?;

    loop {
        // Read V
        let v_opt = scanner.next_i32();
        let v = match v_opt {
            Some(x) => x,
            None => break, // no more data
        };
        if v == 0 {
            // Terminate immediately without output for this case.
            break;
        }
        // As per specification, V is in [1,100]
        let v_usize = v as usize;

        // Initialize cost matrix (only use v_usize x v_usize portion).
        // Use INF sentinel for no direct edge.
        let mut cost = vec![vec![INF; v_usize]; v_usize];

        // Read num
        let num = scanner
            .next_i32()
            .expect("Expected edge count after V per spec") as i32;
        let num = if num < 0 { 0 } else { num } as usize;

        // Read num triplets: from,to,dist (commas/whitespace tolerated)
        for _ in 0..num {
            let from = scanner
                .next_i32()
                .expect("Expected 'from' in edge triplet") as i32;
            let to = scanner
                .next_i32()
                .expect("Expected 'to' in edge triplet") as i32;
            let dist = scanner
                .next_i32()
                .expect("Expected 'dist' in edge triplet") as i32;
            // Per spec, assume indices are in-range [0, V-1]; we cast and use as usize.
            let u = from as usize;
            let widx = to as usize;

            // Transform weight: w = floor(dist/100) - 1
            let flo = floor_div(dist, 100);
            let w = flo - 1;

            if u < v_usize && widx < v_usize {
                cost[u][widx] = w;
                cost[widx][u] = w;
            } else {
                // Per spec, out-of-range indices are undefined behavior; we ignore/skip to stay safe.
                // (We do not abort; this matches the spec's undefined behavior stance.)
            }
        }

        // Prim's algorithm starting at vertex 0 (dense O(V^2) implementation)
        let mut used = vec![false; v_usize];
        let mut mincost = vec![INF; v_usize];
        used[0] = true;
        for i in 0..v_usize {
            mincost[i] = cost[0][i];
        }
        mincost[0] = 0;

        let mut total: i64 = 0;

        // We will add V-1 vertices (or less if V==1).
        for _iter in 1..v_usize {
            // select unused vertex with minimal mincost; tie breaks by first encountered.
            let mut sel: Option<usize> = None;
            for j in 0..v_usize {
                if !used[j] {
                    if sel.is_none() || mincost[j] < mincost[sel.unwrap()] {
                        sel = Some(j);
                    }
                }
            }
            // sel should be Some because there is at least one unused vertex when iter < V.
            let s = match sel {
                Some(x) => x,
                None => {
                    // Defensive: no unused vertex found; break out.
                    break;
                }
            };
            // Add its connection cost (may be INF for unreachable vertices).
            total += mincost[s] as i64;
            used[s] = true;

            // Update mincosts using newly added vertex.
            for j in 0..v_usize {
                if !used[j] && cost[s][j] < mincost[j] {
                    mincost[j] = cost[s][j];
                }
            }
        }

        // Print result for this test case.
        println!("{}", total);
    }

    Ok(())
}
