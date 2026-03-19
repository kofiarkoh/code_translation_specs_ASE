use std::io::{self, Read};

type Int64 = i64;

/// Fenwick tree (BIT) for prefix sums over indices 1..=n.
/// We keep it as a reusable buffer to match the C code's "static" storage behavior.
#[derive(Default)]
struct Fenwick {
    n: usize,
    bit: Vec<i32>,
}

impl Fenwick {
    fn ensure_size(&mut self, n: usize) {
        if self.n != n {
            self.n = n;
            self.bit = vec![0; n + 1];
        } else {
            // reset
            for v in &mut self.bit[1..] {
                *v = 0;
            }
        }
    }

    fn add(&mut self, mut idx: usize, delta: i32) {
        while idx <= self.n {
            self.bit[idx] += delta;
            idx += idx & (!idx + 1); // idx += idx & -idx, but in unsigned form
        }
    }

    fn sum(&self, mut idx: usize) -> i32 {
        let mut res: i32 = 0;
        while idx > 0 {
            res += self.bit[idx];
            idx &= idx - 1;
        }
        res
    }
}

/// Counts the number of contiguous subarrays whose (upper) median is >= m,
/// using the standard transform: b[i] = +1 if a[i] >= m else -1.
/// Then count pairs (i<j) with prefix[j] >= prefix[i].
fn count_ge_median(m: i32, a: &[i32], fenwick: &mut Fenwick) -> Int64 {
    let n = a.len();
    // Indices range over possible prefix sums shifted by (n+1):
    // prefix starts at 0, then changes by +/-1, so range is [-n, n].
    // Shift by (n+1) to make it 1-based and positive: [1, 2n+1].
    let size = 2 * n + 1;
    fenwick.ensure_size(size);

    let mut cnt: Int64 = 0;
    let mut sum_idx: i32 = (n as i32) + 1; // corresponds to prefix sum 0

    fenwick.add(sum_idx as usize, 1);

    for &val in a {
        sum_idx += if val < m { -1 } else { 1 };
        // number of previous prefix sums <= current (i.e., prefix_prev <= prefix_cur)
        cnt += fenwick.sum(sum_idx as usize) as Int64;
        fenwick.add(sum_idx as usize, 1);
    }
    cnt
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i32 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // Binary search over the constrained domain [1, 1_000_000_000]
    // using the same half-open upper bound as C: r = 1_000_000_000 + 1.
    let mut l: i32 = 1;
    let mut r: i32 = 1_000_000_000 + 1;

    let mut fenwick = Fenwick::default();

    while (r - l) > 1 {
        let m = l + (r - l) / 2;
        let cnt = count_ge_median(m, &a, &mut fenwick);

        // C condition: if 4*cnt >= n*(n+1) then l=m else r=m
        // Use i128 to avoid overflow in intermediate multiplication.
        let left = (cnt as i128) * 4;
        let right = (n as i128) * ((n + 1) as i128);

        if left >= right {
            l = m;
        } else {
            r = m;
        }
    }

    print!("{l}\n");
}
