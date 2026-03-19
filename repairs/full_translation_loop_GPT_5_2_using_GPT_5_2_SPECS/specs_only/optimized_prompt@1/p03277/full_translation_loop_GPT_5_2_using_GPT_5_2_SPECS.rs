use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = match it.next() {
        Some(x) => x.parse().unwrap(),
        None => return,
    };

    let mut a = Vec::with_capacity(n);
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
    }

    // We search only within the constrained domain [1, 1_000_000_000] as specified.
    let lo = 1i64;
    let hi = 1_000_000_000i64;

    // Total number of subarrays; specified to fit in signed 64-bit.
    let total_sub: i64 = (n as i64) * ((n as i64) + 1) / 2;
    // "Upper median" of the multiset: smallest x such that count(median >= x) >= ceil(total/2).
    let need: i64 = (total_sub + 1) / 2;

    // Persistent auxiliary storage across calls to the internal counting routine.
    // Here, we allocate once and reuse within the binary search.
    let mut bit = Fenwick::new(2 * n + 5);

    // Count how many subarrays have (upper) median >= x.
    // Standard reduction:
    // For a fixed x, map b[i] = +1 if a[i] >= x else -1.
    // For subarray [l..r], let S = sum(b[l..r]).
    // Then median >= x (upper median convention) iff S > 0.
    // So we count pairs of prefix sums P[j] - P[i] > 0 with i < j, i.e., P[i] < P[j].
    let mut count_ge = |x: i64, bit: &mut Fenwick| -> i64 {
        bit.reset();
        let offset: i64 = (n as i64) + 2; // ensure indices stay positive
        let mut pref: i64 = 0;
        let mut ans: i64 = 0;

        // Insert P[0] = 0
        bit.add((offset + pref) as usize, 1);

        for &v in &a {
            if v >= x {
                pref += 1;
            } else {
                pref -= 1;
            }
            // Count previous prefix sums strictly less than current: P[i] < P[j]
            let idx = (offset + pref) as usize;
            ans += bit.sum(idx.saturating_sub(1));
            bit.add(idx, 1);
        }
        ans
    };

    // Binary search for the maximum x in [lo, hi] such that count_ge(x) >= need.
    // This matches: at least half have median >= x, and fewer than half have median >= x+1.
    let mut l = lo;
    let mut r = hi;
    while l < r {
        let mid = l + (r - l + 1) / 2;
        let c = count_ge(mid, &mut bit);
        if c >= need {
            l = mid;
        } else {
            r = mid - 1;
        }
    }

    println!("{}", l);
}

#[derive(Clone)]
struct Fenwick {
    n: usize,
    bit: Vec<i64>,
}

impl Fenwick {
    fn new(n: usize) -> Self {
        Fenwick { n, bit: vec![0; n + 1] }
    }

    fn reset(&mut self) {
        // Reuse allocated storage as specified; O(n) reset is acceptable under O(n log n log V).
        self.bit.fill(0);
    }

    fn add(&mut self, mut i: usize, delta: i64) {
        if i == 0 {
            // Indices are constructed to be >= 1; if not, behavior is not inferable per spec.
            return;
        }
        while i <= self.n {
            self.bit[i] += delta;
            i += i & (!i + 1);
        }
    }

    fn sum(&self, mut i: usize) -> i64 {
        let mut res = 0i64;
        while i > 0 {
            res += self.bit[i];
            i &= i - 1;
        }
        res
    }
}
