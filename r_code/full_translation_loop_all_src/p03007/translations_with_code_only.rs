use std::io::{self, Read};

fn dfs(f: usize, b: usize, l: usize, sign: i32, a: &[i64], out: &mut String) -> i64 {
    if f == l {
        let mut sum = a[l];
        if sign == 1 {
            // for (int i = l + 1; i <= b - 1; i++)
            if l + 1 <= b.saturating_sub(1) {
                for i in (l + 1)..=b - 1 {
                    out.push_str(&format!("{} {}\n", sum, a[i]));
                    sum = sum.wrapping_sub(a[i]);
                }
            }
            out.push_str(&format!("{} {}\n", a[b], sum));
            sum = a[b].wrapping_sub(sum);
            return sum;
        } else {
            // for (int i = l + 1; i <= b; i++)
            for i in (l + 1)..=b {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum = sum.wrapping_sub(a[i]);
            }
            return sum;
        }
    }

    if b == l + 1 {
        let mut sum = a[l + 1];
        if sign == 1 {
            // for (int i = f; i <= l; i++)
            for i in f..=l {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum = sum.wrapping_sub(a[i]);
            }
            return sum;
        } else {
            // for (int i = f; i < l; i++)
            for i in f..l {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum = sum.wrapping_sub(a[i]);
            }
            out.push_str(&format!("{} {}\n", a[l], sum));
            sum = a[l].wrapping_sub(sum);
            return sum;
        }
    }

    if sign == 1 {
        out.push_str(&format!("{} {}\n", a[b], a[f]));
        let x = dfs(f + 1, b - 1, l, -sign, a, out);
        out.push_str(&format!("{} {}\n", a[b].wrapping_sub(a[f]), x));
        a[b].wrapping_sub(a[f]).wrapping_sub(x)
    } else {
        out.push_str(&format!("{} {}\n", a[f], a[b]));
        let x = dfs(f + 1, b - 1, l, -sign, a, out);
        out.push_str(&format!("{} {}\n", a[f].wrapping_sub(a[b]), x));
        a[f].wrapping_sub(a[b]).wrapping_sub(x)
    }
}

fn main() {
    // Read entire stdin, parse as integers (scanf-like whitespace handling).
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = match it.next() {
        Some(v) => v.parse().unwrap(),
        None => return,
    };

    let mut a: Vec<i64> = Vec::with_capacity(n);
    let mut sum: i64 = 0;

    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        a.push(v);
        // C's abs macro on ll can overflow for LLONG_MIN (UB in C).
        // We emulate typical two's-complement wraparound behavior.
        let av = if v > 0 { v } else { 0i64.wrapping_sub(v) };
        sum = sum.wrapping_add(av);
    }

    a.sort(); // matches qsort with asc comparator

    let mut out = String::new();

    if a[0] >= 0 {
        // printf("%lld\n", sum - a[0] - a[0]);
        let ans = sum.wrapping_sub(a[0]).wrapping_sub(a[0]);
        out.push_str(&format!("{}\n", ans));

        let mut x = a[0];
        for i in 1..(n - 1) {
            out.push_str(&format!("{} {}\n", x, a[i]));
            x = x.wrapping_sub(a[i]);
        }
        out.push_str(&format!("{} {}\n", a[n - 1], x));
    } else if a[n - 1] <= 0 {
        // printf("%lld\n", sum + a[n - 1] + a[n - 1]);
        let ans = sum.wrapping_add(a[n - 1]).wrapping_add(a[n - 1]);
        out.push_str(&format!("{}\n", ans));

        let mut x = a[n - 1];
        for i in 0..(n - 1) {
            out.push_str(&format!("{} {}\n", x, a[i]));
            x = x.wrapping_sub(a[i]);
        }
    } else {
        out.push_str(&format!("{}\n", sum));

        // Find last negative index l (C code uses l=0, r=n and binary search).
        let mut l: usize = 0;
        let mut r: usize = n;
        while r - l > 1 {
            let med = (l + r) / 2;
            if a[med] < 0 {
                l = med;
            } else {
                r = med;
            }
        }

        let _ = dfs(0, n - 1, l, 1, &a, &mut out);
    }

    print!("{}", out);
}
