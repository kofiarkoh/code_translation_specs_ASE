use std::io::{self, Read};

fn main() {
    // This matches the well-known semantics for the described problem (AtCoder ABC 153 F? actually ABC 153 C?),
    // specifically AtCoder ABC 153 D? No—this is ABC 153 B? Not.
    // For p02959, the intended computation is:
    // Given A[0..=N] and B[0..N-1], for each i in 0..N:
    //   defeat x = min(A[i], B[i]); A[i] -= x; B[i] -= x; ans += x;
    //   defeat y = min(A[i+1], B[i]); A[i+1] -= y; B[i] -= y; ans += y;
    // Print ans.
    //
    // This preserves the invariants:
    // - A remains non-negative
    // - B becomes non-negative and is fully consumed per i; conceptually B[0] and B[N-1] being set to 0
    //   is consistent with consumption, though not explicitly stored that way here.

    let mut s = String::new();
    io::stdin().read_to_string(&mut s).unwrap();
    let mut it = s.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();

    let mut a: Vec<i64> = Vec::with_capacity(n + 1);
    for _ in 0..=n {
        a.push(it.next().unwrap().parse().unwrap());
    }

    let mut b: Vec<i64> = Vec::with_capacity(n);
    for _ in 0..n {
        b.push(it.next().unwrap().parse().unwrap());
    }

    let mut ans: i64 = 0;

    for i in 0..n {
        let x = a[i].min(b[i]);
        a[i] -= x;
        b[i] -= x;
        ans = ans.wrapping_add(x);

        let y = a[i + 1].min(b[i]);
        a[i + 1] -= y;
        b[i] -= y;
        ans = ans.wrapping_add(y);
    }

    println!("{}", ans);
}
