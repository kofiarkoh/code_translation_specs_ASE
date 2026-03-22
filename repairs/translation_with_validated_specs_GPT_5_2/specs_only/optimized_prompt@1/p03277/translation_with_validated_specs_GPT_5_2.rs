use std::io::{self, Read};

fn main() {
    // Read all input
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        a.push(it.next().unwrap().parse::<i64>().unwrap());
    }

    // Count subarrays whose (upper) median >= m.
    //
    // Transform b[i] = +1 if a[i] >= m else -1.
    // For a subarray, let S = sum(b). Then:
    // - If length is odd: upper median >= m  <=>  #(+1) >= #(-1)  <=> S >= 1
    // - If length is even: upper median >= m <=>  #(+1) >  #(-1)  <=> S >= 2
    // Both cases are equivalent to: S >= 1 + (len % 2 == 0 ? 1 : 0).
    //
    // Using prefix sums P[0]=0, P[k]=sum_{i<k} b[i], for subarray [l, r):
    // S = P[r] - P[l], len = r-l.
    // Condition becomes:
    //   if len odd:  P[l] <= P[r] - 1
    //   if len even: P[l] <= P[r] - 2
    //
    // We count pairs (l, r) with l<r satisfying the above, split by parity of indices.
    fn count_ge_m(a: &[i64], m: i64) -> i128 {
        let n = a.len();
        let mut pref: Vec<i32> = vec![0; n + 1];
        for i in 0..n {
            let v = if a[i] >= m { 1 } else { -1 };
            pref[i + 1] = pref[i] + v;
        }

        // Coordinate compress prefix sums for Fenwick indexing.
        let mut vals = pref.clone();
        vals.sort_unstable();
        vals.dedup();

        let idx_of = |x: i32, vals: &Vec<i32>| -> usize {
            match vals.binary_search(&x) {
                Ok(i) => i,
                Err(_) => unreachable!(),
            }
        };

        // Fenwick tree for counts (i64 counts are enough, but we accumulate into i128).
        #[derive(Clone)]
        struct Fenwick {
            bit: Vec<i64>,
        }
        impl Fenwick {
            fn new(n: usize) -> Self {
                Fenwick { bit: vec![0; n + 1] }
            }
            fn add(&mut self, mut i: usize, delta: i64) {
                // i is 0-based index into compressed coords; Fenwick uses 1-based.
                i += 1;
                while i < self.bit.len() {
                    self.bit[i] += delta;
                    i += i & (!i + 1);
                }
            }
            fn sum_prefix(&self, mut i: usize) -> i64 {
                // sum over [0..=i] in 0-based coords
                i += 1;
                let mut res = 0i64;
                while i > 0 {
                    res += self.bit[i];
                    i &= i - 1;
                }
                res
            }
        }

        let mut fen_even = Fenwick::new(vals.len());
        let mut fen_odd = Fenwick::new(vals.len());

        // Insert l = 0 (even index) into even structure.
        fen_even.add(idx_of(pref[0], &vals), 1);

        let mut total: i128 = 0;

        for r in 1..=n {
            let pr = pref[r];

            // For odd length subarrays ending at r: l parity != r parity, need P[l] <= P[r] - 1
            // For even length subarrays ending at r: l parity == r parity, need P[l] <= P[r] - 2
            let thr_odd = pr - 1;
            let thr_even = pr - 2;

            // Find largest index with vals[idx] <= threshold.
            let upper_idx = |t: i32, vals: &Vec<i32>| -> Option<usize> {
                // upper_bound(t) - 1
                let mut lo = 0usize;
                let mut hi = vals.len();
                while lo < hi {
                    let mid = (lo + hi) / 2;
                    if vals[mid] <= t {
                        lo = mid + 1;
                    } else {
                        hi = mid;
                    }
                }
                if lo == 0 { None } else { Some(lo - 1) }
            };

            let r_is_even = (r % 2) == 0;

            // odd length: l parity != r parity
            if let Some(ui) = upper_idx(thr_odd, &vals) {
                let add = if r_is_even {
                    // r even => l odd
                    fen_odd.sum_prefix(ui)
                } else {
                    // r odd => l even
                    fen_even.sum_prefix(ui)
                };
                total += add as i128;
            }

            // even length: l parity == r parity
            if let Some(ui) = upper_idx(thr_even, &vals) {
                let add = if r_is_even {
                    fen_even.sum_prefix(ui)
                } else {
                    fen_odd.sum_prefix(ui)
                };
                total += add as i128;
            }

            // Insert current prefix as potential l for future r.
            let id = idx_of(pr, &vals);
            if r_is_even {
                fen_even.add(id, 1);
            } else {
                fen_odd.add(id, 1);
            }
        }

        total
    }

    // Total number of subarrays
    let total_sub: i128 = (n as i128) * ((n as i128) + 1) / 2;
    // "Upper median" of the multiset: smallest k such that >=k elements are >= x.
    // Using the spec's criterion: at least half have median >= x, but fewer than half have >= x+1.
    // For integer counts, "at least half" corresponds to >= ceil(total/2).
    let need: i128 = (total_sub + 1) / 2;

    // Binary search over value range [1, 1e9], maintaining invariant:
    // lo is feasible (count >= need), hi is infeasible (count < need), searching greatest feasible.
    let mut lo: i64 = 1;
    let mut hi: i64 = 1_000_000_000 + 1; // exclusive upper bound, treated as infeasible

    while lo + 1 < hi {
        let mid = lo + (hi - lo) / 2;
        let cnt = count_ge_m(&a, mid);
        if cnt >= need {
            lo = mid;
        } else {
            hi = mid;
        }
    }

    println!("{}", lo);
}
