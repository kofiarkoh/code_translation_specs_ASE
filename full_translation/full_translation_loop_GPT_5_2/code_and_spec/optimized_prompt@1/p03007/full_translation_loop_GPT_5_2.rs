use std::io::{self, Read};

fn dfs(f: usize, b: usize, l: usize, sign: i32, a: &[i64], out: &mut String) -> i64 {
    if f == l {
        let mut sum = a[l];
        if sign == 1 {
            // i = l+1 ..= b-1
            for i in (l + 1)..b {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum -= a[i];
            }
            out.push_str(&format!("{} {}\n", a[b], sum));
            sum = a[b] - sum;
            return sum;
        } else {
            // i = l+1 ..= b
            for i in (l + 1)..=b {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum -= a[i];
            }
            return sum;
        }
    }

    if b == l + 1 {
        let mut sum = a[l + 1];
        if sign == 1 {
            // i = f ..= l
            for i in f..=l {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum -= a[i];
            }
            return sum;
        } else {
            // i = f ..< l
            for i in f..l {
                out.push_str(&format!("{} {}\n", sum, a[i]));
                sum -= a[i];
            }
            out.push_str(&format!("{} {}\n", a[l], sum));
            sum = a[l] - sum;
            return sum;
        }
    }

    if sign == 1 {
        out.push_str(&format!("{} {}\n", a[b], a[f]));
        let x = dfs(f + 1, b - 1, l, -sign, a, out);
        out.push_str(&format!("{} {}\n", a[b] - a[f], x));
        a[b] - a[f] - x
    } else {
        out.push_str(&format!("{} {}\n", a[f], a[b]));
        let x = dfs(f + 1, b - 1, l, -sign, a, out);
        out.push_str(&format!("{} {}\n", a[f] - a[b], x));
        a[f] - a[b] - x
    }
}

fn main() {
    // Read all input
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut it = input.split_whitespace();

    let n: usize = it.next().unwrap().parse().unwrap();
    let mut a: Vec<i64> = Vec::with_capacity(n);

    let mut sum_abs: i64 = 0;
    for _ in 0..n {
        let v: i64 = it.next().unwrap().parse().unwrap();
        sum_abs += v.abs();
        a.push(v);
    }

    a.sort();

    let mut out = String::new();

    if a[0] >= 0 {
        // All non-negative
        out.push_str(&format!("{}\n", sum_abs - 2 * a[0]));
        let mut x = a[0];
        for i in 1..(n - 1) {
            out.push_str(&format!("{} {}\n", x, a[i]));
            x -= a[i];
        }
        out.push_str(&format!("{} {}\n", a[n - 1], x));
    } else if a[n - 1] <= 0 {
        // All non-positive
        out.push_str(&format!("{}\n", sum_abs + 2 * a[n - 1]));
        let mut x = a[n - 1];
        for i in 0..(n - 1) {
            out.push_str(&format!("{} {}\n", x, a[i]));
            x -= a[i];
        }
    } else {
        // Mixed signs
        out.push_str(&format!("{}\n", sum_abs));

        // Find l: the last index with a[l] < 0 (binary search as in C)
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

    print!("{out}");
}
