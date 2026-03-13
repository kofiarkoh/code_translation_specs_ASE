// prim_mst.rs
// Translated from the provided C program to Rust following the SPEC.
// Notes:
// - The SPEC requires weight transformation w = floor(dist/100) - 1.
//   The original C code used integer truncation (dist / 100) - 1 which differs for negative dist.
//   This Rust implementation follows the SPEC and uses floor division semantics for dist/100.
// - The SPEC is authoritative in cases of conflict with the C code (e.g., cost matrix
//   should be initialized for indices 0..V-1). This implementation initializes per-V matrices.

use std::io::{self, Read};
use std::cmp::min;

const INF: i32 = 1_000_000;

struct Scanner {
    data: Vec<char>,
    pos: usize,
}

impl Scanner {
    fn new(s: String) -> Self {
        Self {
            data: s.chars().collect(),
            pos: 0,
        }
    }

    // Advance until we find a digit or a sign, or reach EOF.
    fn skip_to_number(&mut self) {
        while self.pos < self.data.len() {
            let c = self.data[self.pos];
            if c.is_ascii_digit() || c == '+' || c == '-' {
                break;
            }
            self.pos += 1;
        }
    }

    // Parse next i32 if present. Returns None on EOF/no more numbers.
    fn next_i32(&mut self) -> Option<i32> {
        self.skip_to_number();
        if self.pos >= self.data.len() {
            return None;
        }
        let start = self.pos;
        // include optional sign
        if self.data[self.pos] == '+' || self.data[self.pos] == '-' {
            self.pos += 1;
        }
        // consume digits
        let mut any_digit = false;
        while self.pos < self.data.len() && self.data[self.pos].is_ascii_digit() {
            any_digit = true;
            self.pos += 1;
        }
        if !any_digit {
            // No digits after optional sign: treat as no number.
            return None;
        }
        let s: String = self.data[start..self.pos].iter().collect();
        s.parse::<i32>().ok()
    }
}

// Floor division of a by b, where b > 0.
// For positive a this is a / b; for negative a this yields the mathematical floor.
fn floor_div(a: i32, b: i32) -> i32 {
    debug_assert!(b > 0);
    if a >= 0 {
        a / b
    } else {
        // For negative a, floor division: (a - (b - 1)) / b
        (a - (b - 1)) / b
    }
}

fn prim(cost: &Vec<Vec<i32>>, v_count: usize) -> i32 {
    let mut mincost = vec![INF; v_count];
    let mut used = vec![false; v_count];

    mincost[0] = 0;
    let mut res: i32 = 0;

    loop {
        let mut v_opt: Option<usize> = None;
        for u in 0..v_count {
            if !used[u] {
                if v_opt.is_none() || mincost[u] < mincost[v_opt.unwrap()] {
                    v_opt = Some(u);
                }
            }
        }
        let v = match v_opt {
            None => break,
            Some(x) => x,
        };
        used[v] = true;
        // Sum chosen connection cost (may be INF for unreachable vertices).
        res = res + mincost[v];

        for u in 0..v_count {
            mincost[u] = min(mincost[u], cost[v][u]);
        }
    }

    res
}

fn main() {
    // Read entire stdin
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).expect("Failed to read stdin");
    let mut scanner = Scanner::new(input);

    // Loop over test cases
    while let Some(v_raw) = scanner.next_i32() {
        let v = v_raw as usize;
        if v == 0 {
            break;
        }

        // Read num
        let num = match scanner.next_i32() {
            Some(n) => n as usize,
            None => break, // malformed input; SPEC leaves behavior undefined. We stop.
        };

        // Initialize cost matrix for vertices 0..v-1 to INF (per SPEC).
        let mut cost = vec![vec![INF; v]; v];

        // Read num triplets: from,to,dist
        for _ in 0..num {
            // The SPEC guarantees well-formed inputs; unwraps assumed safe in that context.
            let from = scanner.next_i32().expect("Missing 'from' value") as usize;
            let to = scanner.next_i32().expect("Missing 'to' value')") as usize;
            let dist = scanner.next_i32().expect("Missing 'dist' value");

            // Compute w = floor(dist/100) - 1 as per SPEC.
            let w = floor_div(dist, 100) - 1;
            // Trusting inputs satisfy 0 <= from < v and 0 <= to < v per SPEC.
            cost[from][to] = w;
            cost[to][from] = w;
        }

        let result = prim(&cost, v);
        println!("{}", result);
    }
}
