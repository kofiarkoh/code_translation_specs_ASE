use std::cmp::Ordering;
use std::io::{self, Read};

const INF: i64 = 0x1010_1010_1010_1010i64;

#[derive(Clone, Copy)]
struct Item {
    x: i64, // position-like
    w: i64, // weight-like (read as non-negative int in C; keep i64 for arithmetic)
}

struct Scanner {
    input: Vec<u8>,
    idx: usize,
}

impl Scanner {
    fn new() -> Self {
        let mut input = Vec::new();
        io::stdin().read_to_end(&mut input).unwrap();
        Self { input, idx: 0 }
    }

    fn skip_non_digit(&mut self) {
        while self.idx < self.input.len() && (self.input[self.idx] < b'0' || self.input[self.idx] > b'9') {
            self.idx += 1;
        }
    }

    fn next_u64(&mut self) -> u64 {
        self.skip_non_digit();
        let mut n: u64 = 0;
        while self.idx < self.input.len() {
            let c = self.input[self.idx];
            if c < b'0' || c > b'9' {
                break;
            }
            n = n * 10 + (c - b'0') as u64;
            self.idx += 1;
        }
        n
    }

    fn next_usize(&mut self) -> usize {
        self.next_u64() as usize
    }

    fn next_i64_nonneg(&mut self) -> i64 {
        self.next_u64() as i64
    }
}

fn main() {
    let mut sc = Scanner::new();

    let n = sc.next_usize();
    let mut items: Vec<Item> = Vec::with_capacity(n);
    for _ in 0..n {
        let x = sc.next_i64_nonneg();
        let w = sc.next_i64_nonneg();
        items.push(Item { x, w });
    }

    // Sort by x ascending, then w ascending (matches spec and C comparator).
    items.sort_unstable_by(|a, b| match a.x.cmp(&b.x) {
        Ordering::Equal => a.w.cmp(&b.w),
        other => other,
    });

    // Compute:
    // max_{l<=r} (sum_w[l..=r] - (x_r - x_l))
    // = max_r ( (prefix_w[r] - x_r) - min_{l<=r}(prefix_w[l-1] - x_l) )
    let mut min_val: i64 = INF;
    let mut max_val: i64 = -INF;

    let mut prefix: i64 = 0;
    for it in items.iter() {
        // t = s[i-1] - x_i
        let t1 = prefix - it.x;
        if t1 < min_val {
            min_val = t1;
        }

        // s[i] = s[i-1] + w_i
        prefix += it.w;

        // t = s[i] - x_i - min
        let t2 = prefix - it.x - min_val;
        if t2 > max_val {
            max_val = t2;
        }
    }

    // Spec guarantees non-empty subsequence; with non-negative weights, result is >= 0.
    print!("{max_val}\n");
}
