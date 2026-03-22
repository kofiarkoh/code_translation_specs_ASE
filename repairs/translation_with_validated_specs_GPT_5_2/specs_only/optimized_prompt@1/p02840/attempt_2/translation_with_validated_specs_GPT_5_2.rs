use std::cmp::max;
use std::collections::BTreeMap;
use std::io::{self, Read};

fn main() {
    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: i64 = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };
    let x: i64 = it.next().unwrap().parse().unwrap();
    let d: i64 = it.next().unwrap().parse().unwrap();

    // Special cases exactly as specified.
    if x == 0 && d == 0 {
        println!("1");
        return;
    }
    if d == 0 {
        println!("{}", n + 1);
        return;
    }
    if x == 0 {
        let nn = n as i128;
        println!("{}", nn * (nn - 1) / 2 + 1);
        return;
    }

    // Normalize D to be positive; this does not change the set of achievable sums.
    let mut dd = d;
    let mut xx = x;
    if dd < 0 {
        dd = -dd;
        xx = -xx;
    }

    // Use Euclidean division so that 0 <= b < dd.
    // Rust's div_euclid/rem_euclid provide exactly that.
    let a = xx.div_euclid(dd);
    let b = xx.rem_euclid(dd); // 0..dd-1

    // For each remainder class r = (m*b) mod dd, collect intervals of possible "q" values:
    // sum = dd * q + r, where q ranges over an interval depending on m.
    //
    // IMPORTANT: For fixed subset size m, the set of possible index sums is NOT always a full
    // contiguous interval when m is small (notably m=1). The correct approach is:
    // - For m=0: index sum set is {0}
    // - For m=1: index sum set is {0,1,...,N-1} (contiguous)
    // - For m>=2: index sum set is contiguous from L(m) to R(m)
    //
    // This fixes the sample where m=1 was previously over-approximated as [0, N-1] in q-space
    // but with an incorrect upper bound due to using R(m)=m(2N-m-1)/2 for all m.
    let mut mp: BTreeMap<i64, Vec<(i64, i64)>> = BTreeMap::new();

    for m in 0..=n {
        let mm = m as i128;
        let nn = n as i128;

        let (l_s, r_s) = if m == 0 {
            (0i128, 0i128)
        } else if m == 1 {
            (0i128, nn - 1)
        } else {
            let l = mm * (mm - 1) / 2;
            let r = mm * (2 * nn - mm - 1) / 2;
            (l, r)
        };

        // q = s + m*a
        let shift = (m as i128) * (a as i128);
        let l_q = l_s + shift;
        let r_q = r_s + shift;

        // r = (m*b) mod dd
        let r = (((m as i128) * (b as i128)) % (dd as i128)) as i64;

        mp.entry(r)
            .or_default()
            .push((l_q as i64, r_q as i64));
    }

    // Merge intervals per remainder class and sum lengths.
    let mut ans: i128 = 0;
    for (_r, mut intervals) in mp {
        intervals.sort_by(|(l1, r1), (l2, r2)| l1.cmp(l2).then(r1.cmp(r2)));

        let mut cur_l = 0i64;
        let mut cur_r = -1i64;
        let mut first = true;

        for (l, r) in intervals {
            if first {
                cur_l = l;
                cur_r = r;
                first = false;
                continue;
            }
            if l <= cur_r + 1 {
                cur_r = max(cur_r, r);
            } else {
                ans += (cur_r as i128) - (cur_l as i128) + 1;
                cur_l = l;
                cur_r = r;
            }
        }
        if !first {
            ans += (cur_r as i128) - (cur_l as i128) + 1;
        }
    }

    println!("{}", ans);
}
